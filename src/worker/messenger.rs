use std::{io::{Read, Write}, net::{TcpListener, TcpStream}};
use std::time::Duration;
use std::process;

use regex::Error;

use crate::worker::message::{MessageError, ParsedMessage};

use super::message;
use super::result;

pub struct MessengerListener {
    listener: TcpListener,
}

impl MessengerListener {
    pub fn listen(port: u16) -> Result<Self, std::io::Error> {
        let addr = format!("127.0.0.1:{}", port);
        let listener = TcpListener::bind(&addr)?;
        Ok(MessengerListener { listener })
    }

    pub fn accept(&self) -> Result<Messenger, std::io::Error> {
        let (stream, remote) = self.listener.accept()?;
        Ok(Messenger { stream, remote_addr: Some(remote) })
    }
}

pub struct Messenger {
    stream: std::net::TcpStream,
    remote_addr: Option<std::net::SocketAddr>,
}

impl Messenger {
    pub fn connect(port: u16) -> Result<Self, std::io::Error> {
        let addr = format!("127.0.0.1:{}", port);
        let stream = TcpStream::connect(&addr)?;
        Ok(Messenger { stream, remote_addr: None })
    }

    pub fn send_raw(&mut self, data: &[u8]) -> Result<usize, std::io::Error> {
        let n = self.stream.write(data)?;
        Ok(n)
    }

    pub fn send<T: message::Message>(&mut self, msg: &T) -> Result<usize, std::io::Error> {
        let bin = msg.serialize();
        self.send_raw(&bin)
    }

    pub fn recv(&mut self) -> Result<ParsedMessage, MessageError> {
        let mut buf = [0; 1024];
        let n = self.stream.read(&mut buf)
            .map_err(|e| {
                if e.kind() == std::io::ErrorKind::WouldBlock {
                    MessageError::ReadTimeout
                } else {
                    MessageError::IOError { source: e }
                }
            })?;

        let result = message::parse_message(&buf[..n])?;
        Ok(result)
    }

    pub fn set_recv_timeout(&self, timeout: Option<Duration>) {
        let _ = self.stream.set_read_timeout(timeout);
    }

    pub fn ping(&mut self) -> Result<(), MessageError> {
        let msg = message::MessagePing::random();
        self.send(&msg)
            .map_err(|e| MessageError::IOError { source: e })?;

        let reply = self.recv()?;
        match reply {
            ParsedMessage::Pong(pong) => {
                if msg.seq != pong.seq {
                    Err(MessageError::WrongPingSeq {
                        got: pong.seq,
                        exp: msg.seq,
                    })
                } else {
                    Ok(())
                }
            },
            _ => Err(MessageError::WrongReplyType {
                got: match reply {
                    ParsedMessage::Ping(_) => message::MessageType::Ping,
                    ParsedMessage::Pong(_) => message::MessageType::Pong,
                    ParsedMessage::Run(_) => message::MessageType::Run,
                    ParsedMessage::Result(_) => message::MessageType::Result,
                },
                exp: message::MessageType::Pong,
            }),
        }
    }

    pub fn run(&mut self, problem_id: i64, solution_id: usize, timeout: Option<Duration>) -> Result<result::RunResult, result::RunError> {
        let msg = message::MessageRun::request(problem_id as i32, solution_id as i32);
        self.send(&msg)
            .map_err(|e| result::RunError::NetworkError { source: e })?;

        let extra_timeout = match timeout {
            Some(t) => Some(t + Duration::from_millis(100)),
            None => None,
        };
        self.set_recv_timeout(extra_timeout);

        let reply = self.recv()
            .map_err(|e| {
                match e {
                    MessageError::ReadTimeout =>
                        result::RunError::Timeout,
                    MessageError::IOError { source } => 
                        result::RunError::NetworkError { source },
                    e =>
                        result::RunError::ProtocolMessageError { source: e },
                }
            })?;
        match reply {
            ParsedMessage::Result(result) => {
                if result.flags.empty() {
                    Ok(result::RunResult::basic(result.result, result.time_cost))
                } else {
                    if result.flags.is_not_found() {
                        if result.solutions_id < 0 {
                            Err(result::RunError::ProblemNotFound { problem_id })
                        } else {
                            Err(result::RunError::SolutionNotFound { problem_id, solution_id })
                        }
                    } else {
                        Err(result::RunError::NetworkError { source: std::io::Error::new(std::io::ErrorKind::Other, "Invalid magic number") })
                    }
                }
            },
            _ => Err(result::RunError::ProtocolMessageError {
                source: MessageError::WrongReplyType {
                    got: match reply {
                        ParsedMessage::Ping(_) => message::MessageType::Ping,
                        ParsedMessage::Pong(_) => message::MessageType::Pong,
                        ParsedMessage::Run(_) => message::MessageType::Run,
                        ParsedMessage::Result(_) => message::MessageType::Result,
                    },
                    exp: message::MessageType::Result,
                }
            }),
        }
    }

    pub fn close(self) -> Result<(), std::io::Error> {
        self.stream.shutdown(std::net::Shutdown::Both)
    }
}
