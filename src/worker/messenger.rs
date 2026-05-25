
use std::{io::{Read, Write}, net::{TcpListener, TcpStream}};

use crate::worker::message::{MessageError, ParsedMessage};

use super::message;
use super::message::Message;
use super::result;

struct MessengerListener {
    listener: TcpListener,
}

impl MessengerListener {
    pub fn listen(port: u16) -> Result<Self, std::io::Error> {
        let addr = format!("127.0.0.1:{}", port);
        let listener = TcpListener::bind(&addr)?;
        Ok(MessengerListener { listener })
    }

    pub fn accept(&self) -> Result<Messenger, std::io::Error> {
        let (stream, _) = self.listener.accept()?;
        Ok(Messenger { stream })
    }
}

struct Messenger {
    stream: std::net::TcpStream,
}

impl Messenger {
    pub fn connect(port: u16) -> Result<Self, std::io::Error> {
        let addr = format!("127.0.0.1:{}", port);
        let stream = TcpStream::connect(&addr)?;
        Ok(Messenger { stream })
    }

    pub fn send<T: message::Message>(&mut self, msg: &T) -> Result<(), std::io::Error> {
        let bin = msg.serialize();
        self.stream.write(&bin)?;
        Ok(())
    }

    pub fn recv(&mut self) -> Result<ParsedMessage, MessageError> {
        let mut buf = [0; 1024];
        let n = self.stream.read(&mut buf)
            .map_err(|e| MessageError::IOError { source: e })?;
        let result = message::parse_message(&buf[..n])?;
        Ok(result)
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

    pub fn run(&mut self, problem_id: i64, solution_id: usize) -> Result<result::RunResult, MessageError> {
        let msg = message::MessageRun::request(problem_id as i32, solution_id as i32);
        self.send(&msg)
            .map_err(|e| MessageError::IOError { source: e })?;

        let reply = self.recv()?;
        match reply {
            ParsedMessage::Result(result) => {
                Ok(result::RunResult::basic(result.result, result.time_cost))
            },
            _ => Err(MessageError::WrongReplyType {
                got: match reply {
                    ParsedMessage::Ping(_) => message::MessageType::Ping,
                    ParsedMessage::Pong(_) => message::MessageType::Pong,
                    ParsedMessage::Run(_) => message::MessageType::Run,
                    ParsedMessage::Result(_) => message::MessageType::Result,
                },
                exp: message::MessageType::Result,
            }),
        }
    }
}
