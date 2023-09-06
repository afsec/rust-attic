use std::error::Error;
use std::fmt;
use std::io::{Error as ioError, ErrorKind};
use std::sync::mpsc::RecvTimeoutError;

#[derive(Debug)]
pub enum TrickError {
    NoArgs,
    InvalidCommand,
    Timeout,
    Disconnected,
    BufferOverflow,
    DatagramSizeMismatch,
    UnknowIOError(String),
    IOError(String),
}

impl From<RecvTimeoutError> for TrickError {
    fn from(err: RecvTimeoutError) -> Self {
        match err {
            RecvTimeoutError::Timeout => Self::Timeout,
            RecvTimeoutError::Disconnected => Self::Disconnected,
        }
    }
}

impl From<ioError> for TrickError {
    fn from(err: ioError) -> Self {
        match err.kind() {
            ErrorKind::NotFound => TrickError::IOError("entity not found".to_owned()),
            ErrorKind::PermissionDenied => TrickError::IOError("permission denied".to_owned()),
            ErrorKind::ConnectionRefused => TrickError::IOError("connection refused".to_owned()),
            ErrorKind::ConnectionReset => TrickError::IOError("connection reset".to_owned()),
            ErrorKind::ConnectionAborted => TrickError::IOError("connection aborted".to_owned()),
            ErrorKind::NotConnected => TrickError::IOError("not connected".to_owned()),
            ErrorKind::AddrInUse => TrickError::IOError("address in use".to_owned()),
            ErrorKind::AddrNotAvailable => TrickError::IOError("address not available".to_owned()),
            ErrorKind::BrokenPipe => TrickError::IOError("broken pipe".to_owned()),
            ErrorKind::AlreadyExists => TrickError::IOError("entity already exists".to_owned()),
            ErrorKind::WouldBlock => TrickError::IOError("operation would block".to_owned()),
            ErrorKind::InvalidInput => TrickError::IOError("invalid input parameter".to_owned()),
            ErrorKind::InvalidData => TrickError::IOError("invalid data".to_owned()),
            ErrorKind::TimedOut => TrickError::IOError("timed out".to_owned()),
            ErrorKind::WriteZero => TrickError::IOError("write zero".to_owned()),
            ErrorKind::Interrupted => TrickError::IOError("operation interrupted".to_owned()),
            ErrorKind::Other => TrickError::IOError("other os error".to_owned()),
            ErrorKind::UnexpectedEof => TrickError::IOError("unexpected end of file".to_owned()),
            _ => TrickError::UnknowIOError(err.to_string()),
        }
    }
}

impl fmt::Display for TrickError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl Error for TrickError {}

// impl From<TrickError> for i32 {
//     fn from(err: TrickError) -> Self {
//         match err {
//             TrickError::NoArgs => 1,
//             TrickError::InvalidCommand => 2,
//             TrickError::Timeout => 3,
//             TrickError::Disconnected => 4,
//         }
//     }
// }
