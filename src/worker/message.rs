use std::fmt;
use std::time;

use rand;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MessageType {
    Ping = 2,
    Pong = 3,
    Run = 4,
    Result = 5
}

const MAGIC_NUMBER: [u8; 4] = [0x45, 0x55, 0x17, 0x07];

pub fn read_beu64_duration(bytes: &[u8]) -> time::Duration {
    let secs = u32::from_be_bytes(bytes[0..4].try_into().unwrap());
    let nanos = u32::from_be_bytes(bytes[4..8].try_into().unwrap());
    time::Duration::new(secs as u64, nanos)
}

pub fn write_beu64_duration(duration: time::Duration) -> [u8; 8] {
    let secs = duration.as_secs() as u32;
    let nanos = duration.subsec_nanos();
    let mut bytes = [0u8; 8];
    bytes[0..4].copy_from_slice(&secs.to_be_bytes());
    bytes[4..8].copy_from_slice(&nanos.to_be_bytes());
    bytes
}

pub trait Message: fmt::Debug + Clone{
    fn message_type(&self) -> MessageType;
    fn total_length(&self) -> u16;
    fn serialize(&self) -> Vec<u8>;
    fn deserialize(bytes: &[u8]) -> Self where Self: Sized;
}

#[derive(Debug)]
pub enum MessageError {
    InvalidMagicNumber,
    InvalidMessageType,
    InvalidLength {
        length: usize,
    },
    IOError {
        source: std::io::Error,
    },
    WrongReplyType {
        got: MessageType,
        exp: MessageType,
    },
    WrongPingSeq {
        got: u64,
        exp: u64,
    },
    ReadTimeout,
}

impl fmt::Display for MessageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MessageError::InvalidMagicNumber => write!(f, "Invalid magic number"),
            MessageError::InvalidMessageType => write!(f, "Invalid message type"),
            MessageError::InvalidLength {
                length ,
            } => write!(f, "Invalid message length: {}", length),
            MessageError::IOError {
                source,
            } => write!(f, "IO error: {}", source),
            MessageError::WrongReplyType {
                got,
                exp,
            } => write!(f, "Wrong reply type: expected {:?}, got {:?}", exp, got),
            MessageError::WrongPingSeq {
                got,
                exp,
            } => write!(f, "Wrong ping sequence: expected {}, got {}", exp, got),
            MessageError::ReadTimeout => write!(f, "Read timeout"),
        }
    }
}

// +-----------------+--------------+--------------+
// |   Magic Number  | Message Type | Total Length |
// +-----------------+--------------+--------------+
// | x45 x55 x17 x07 |   u16 (BE)   |   u16 (BE)   |
// +-----------------+--------------+--------------+
#[derive(Debug, Clone)]
pub struct MessageHeader {
    pub message_type: MessageType,
    pub total_length: u16,
}

// +------------------+-----------+
// |  Message Header  |    Seq    |
// +------------------+-----------+
// |  8 bytes header  |    u64    |
// +------------------+-----------+
#[derive(Debug, Clone)]
pub struct MessagePing {
    pub header: MessageHeader,
    pub seq: u64,
}

impl Message for MessagePing {
    fn message_type(&self) -> MessageType {
        self.header.message_type.clone()
    }

    fn total_length(&self) -> u16 {
        16
    }

    fn serialize(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&MAGIC_NUMBER);
        bytes.extend_from_slice(&(self.message_type() as u16).to_be_bytes());
        bytes.extend_from_slice(&(self.total_length() as u16).to_be_bytes());
        bytes.extend_from_slice(&self.seq.to_be_bytes());
        bytes
    }

    fn deserialize(bytes: &[u8]) -> Self {
        if bytes.len() != 16 {
            panic!("Invalid byte length for MessagePing");
        }
        let message_type = u16::from_be_bytes(bytes[4..6].try_into().unwrap());
        println!("Deserializing MessagePing: message_type={}, length={}", message_type, bytes.len());
        if (message_type != MessageType::Ping as u16) && (message_type != MessageType::Pong as u16) {
            panic!("Invalid message type for MessagePing");
        }
        let seq = u64::from_be_bytes(bytes[8..16].try_into().unwrap());
        Self {
            header: MessageHeader {
                message_type: MessageType::Ping,
                total_length: 16,
            },
            seq,
        }
    }
}

impl MessagePing {
    pub fn new(seq: u64) -> Self {
        MessagePing {
            header: MessageHeader {
                message_type: MessageType::Ping,
                total_length: 16,
            },
            seq,
        }
    }

    pub fn random() -> Self {
        let seq = rand::random();
        MessagePing::new(seq)
    }

    pub fn to_pong(&self) -> MessagePing {
        MessagePing {
            header: MessageHeader {
                message_type: MessageType::Pong,
                total_length: 16,
            },
            seq: self.seq,
        }
    }
}

// +------------------+--------------+----------------+
// |  Message Header  |  Problem ID  |  Solutions ID  |
// +------------------+--------------+----------------+
// |  8 bytes header  |     i32      |      i32       |
// +------------------+--------------+----------------+
#[derive(Debug, Clone)]
pub struct MessageRun {
    pub header: MessageHeader,
    // pub problem_timeout: time::Duration,
    // pub solution_timeout: time::Duration,
    pub problem_id: i32,
    pub solutions_id: i32,
}

impl Message for MessageRun {
    fn message_type(&self) -> MessageType {
        self.header.message_type.clone()
    }

    fn total_length(&self) -> u16 {
        8 + 4 + 4
    }

    fn serialize(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&MAGIC_NUMBER);
        bytes.extend_from_slice(&(self.message_type() as u16).to_be_bytes());
        bytes.extend_from_slice(&(self.total_length() as u16).to_be_bytes());
        bytes.extend_from_slice(&self.problem_id.to_be_bytes());
        bytes.extend_from_slice(&self.solutions_id.to_be_bytes());
        bytes
    }

    fn deserialize(bytes: &[u8]) -> Self {
        if bytes.len() != 16 {
            panic!("Invalid byte length for MessageRun");
        }
        let message_type = u16::from_be_bytes(bytes[4..6].try_into().unwrap());
        if message_type != MessageType::Run as u16 {
            panic!("Invalid message type for MessageRun");
        }
        let problem_id = i32::from_be_bytes(bytes[8..12].try_into().unwrap());
        let solutions_id = i32::from_be_bytes(bytes[12..16].try_into().unwrap());
        Self {
            header: MessageHeader {
                message_type: MessageType::Run,
                total_length: 32,
            },
            problem_id,
            solutions_id,
        }
    }
}

impl MessageRun {
    pub fn request(problem_id: i32, solutions_id: i32) -> Self {
        MessageRun {
            header: MessageHeader {
                message_type: MessageType::Run,
                total_length: 32,
            },
            problem_id,
            solutions_id,
        }
    }

    pub fn reply(&self, time_cost: time::Duration, result: i64, flags: MessageResultFlags) -> MessageResult {
        MessageResult {
            header: MessageHeader {
                message_type: MessageType::Result,
                total_length: 40,
            },
            time_cost,
            result,
            flags,
            problem_id: self.problem_id,
            solutions_id: self.solutions_id,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MessageResultFlags(u64);

impl MessageResultFlags {
    pub const NONE: Self = MessageResultFlags(0);
    pub const NOT_FOUND: Self = MessageResultFlags(1 << 0);
    pub const CRASHED: Self = MessageResultFlags(1 << 1);

    pub fn empty(&self) -> bool {
        self.0 == 0
    }

    pub fn is_not_found(&self) -> bool {
        self.0 & Self::NOT_FOUND.0 != 0
    }

    pub fn not_found(&mut self) -> &mut Self {
        self.0 |= Self::NOT_FOUND.0;
        self
    }

    pub fn is_crashed(&self) -> bool {
        self.0 & Self::CRASHED.0 != 0
    }

    pub fn crashed(&mut self) -> &mut Self {
        self.0 |= Self::CRASHED.0;
        self
    }
}

// +------------------+---------------+------------+--------------+--------------+----------------+
// |  Message Header  |   Time cost   |   Result   |    flags     |  Problem ID  |  Solutions ID  |
// +------------------+---------------+------------+--------------+--------------+----------------+
// |  8 bytes header  |      i64      |    i64     |     u64      |     i32      |      i32       |
// +------------------+---------------+------------+--------------+--------------+----------------+
#[derive(Debug, Clone)]
pub struct MessageResult {
    pub header: MessageHeader,
    pub time_cost: time::Duration,
    pub result: i64,
    pub flags: MessageResultFlags,
    pub problem_id: i32,
    pub solutions_id: i32,
}

impl Message for MessageResult {
    fn message_type(&self) -> MessageType {
        self.header.message_type.clone()
    }

    fn total_length(&self) -> u16 {
        8 + 8 + 8 + 8 + 4 + 4
    }

    fn serialize(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&MAGIC_NUMBER);
        bytes.extend_from_slice(&(self.message_type() as u16).to_be_bytes());
        bytes.extend_from_slice(&(self.total_length() as u16).to_be_bytes());
        bytes.extend_from_slice(&write_beu64_duration(self.time_cost));
        bytes.extend_from_slice(&self.result.to_be_bytes());
        bytes.extend_from_slice(&self.flags.0.to_be_bytes());
        bytes.extend_from_slice(&self.problem_id.to_be_bytes());
        bytes.extend_from_slice(&self.solutions_id.to_be_bytes());
        bytes
    }

    fn deserialize(bytes: &[u8]) -> Self where Self: Sized {
        if bytes.len() != 40 {
            panic!("Invalid byte length for MessageResult");
        }
        let message_type = u16::from_be_bytes(bytes[4..6].try_into().unwrap());
        if message_type != MessageType::Result as u16 {
            panic!("Invalid message type for MessageResult");
        }
        let time_cost = read_beu64_duration(bytes[8..16].try_into().unwrap());
        let result = i64::from_be_bytes(bytes[16..24].try_into().unwrap());
        let flags = u64::from_be_bytes(bytes[24..32].try_into().unwrap());
        let problem_id = i32::from_be_bytes(bytes[32..36].try_into().unwrap());
        let solutions_id = i32::from_be_bytes(bytes[36..40].try_into().unwrap());
        Self {
            header: MessageHeader {
                message_type: MessageType::Result,
                total_length: 40,
            },
            time_cost,
            result,
            flags: MessageResultFlags(flags),
            problem_id,
            solutions_id,
        }
    }
}

impl MessageResult {
    pub fn problem_not_found(pid: i32) -> Self {
        MessageResult {
            header: MessageHeader {
                message_type: MessageType::Result,
                total_length: 40,
            },
            time_cost: time::Duration::from_secs(0),
            result: 0,
            flags: MessageResultFlags::NOT_FOUND,
            problem_id: pid,
            solutions_id: -1,
        }
    }

    pub fn solution_not_found(pid: i32, sid: i32) -> Self {
        MessageResult {
            header: MessageHeader {
                message_type: MessageType::Result,
                total_length: 40,
            },
            time_cost: time::Duration::from_secs(0),
            result: 0,
            flags: MessageResultFlags::NOT_FOUND,
            problem_id: pid,
            solutions_id: sid,
        }
    }
}

#[derive(Debug)]
pub enum ParsedMessage {
    Ping(MessagePing),
    Pong(MessagePing),
    Run(MessageRun),
    Result(MessageResult),
}

pub fn parse_message(bytes: &[u8]) -> Result<ParsedMessage, MessageError> {
    if bytes.len() < 8 {
        return Err(MessageError::InvalidLength { length: bytes.len() });
    }
    
    let magic_number = &bytes[0..4];
    if magic_number != MAGIC_NUMBER {
        return Err(MessageError::InvalidMagicNumber);
    }

    let message_type = u16::from_be_bytes(bytes[4..6].try_into().unwrap());
    match message_type {
        x if x == MessageType::Ping as u16 => Ok(ParsedMessage::Ping(MessagePing::deserialize(bytes))),
        x if x == MessageType::Pong as u16 => Ok(ParsedMessage::Pong(MessagePing::deserialize(bytes))),
        x if x == MessageType::Run as u16 => Ok(ParsedMessage::Run(MessageRun::deserialize(bytes))),
        x if x == MessageType::Result as u16 => Ok(ParsedMessage::Result(MessageResult::deserialize(bytes))),
        _ => return Err(MessageError::InvalidMessageType),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_ping_serialization() {
        let ping = MessagePing {
            header: MessageHeader {
                message_type: MessageType::Ping,
                total_length: 16,
            },
            seq: 42,
        };
        let bytes = ping.serialize();
        let deserialized_ping = MessagePing::deserialize(&bytes);

        assert_eq!(ping.seq, deserialized_ping.seq);
        assert_eq!(ping.header.message_type as u16, deserialized_ping.header.message_type as u16);
        assert_eq!(ping.header.total_length, deserialized_ping.header.total_length);

        assert_eq!(ping.message_type(), MessageType::Ping);
        assert_eq!(ping.total_length(), 16);
    }

    #[test]
    fn test_message_run_serialization() {
        let run = MessageRun {
            header: MessageHeader {
                message_type: MessageType::Run,
                total_length: 16,
            },
            problem_id: 123,
            solutions_id: 456,
        };
        let bytes = run.serialize();
        let deserialized_run = MessageRun::deserialize(&bytes);

        assert_eq!(run.problem_id, deserialized_run.problem_id);
        assert_eq!(run.solutions_id, deserialized_run.solutions_id);
        assert_eq!(run.header.message_type as u16, deserialized_run.header.message_type as u16);
        assert_eq!(run.header.total_length, deserialized_run.header.total_length);

        assert_eq!(run.message_type(), MessageType::Run);
        assert_eq!(run.total_length(), 16);
    }
}
