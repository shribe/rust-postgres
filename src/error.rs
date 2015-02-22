pub use ugh_privacy::DbError;

use std::error;
use std::old_io::IoError;
use std::fmt;

use openssl::ssl::error::SslError;
use phf;

use Result;
use types::Type;

include!(concat!(env!("OUT_DIR"), "/sqlstate.rs"));

/// Reasons a new Postgres connection could fail
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ConnectError {
    /// The provided URL could not be parsed
    InvalidUrl(String),
    /// The URL was missing a user
    MissingUser,
    /// An error from the Postgres server itself
    DbError(DbError),
    /// A password was required but not provided in the URL
    MissingPassword,
    /// The Postgres server requested an authentication method not supported
    /// by the driver
    UnsupportedAuthentication,
    /// The Postgres server does not support SSL encryption
    NoSslSupport,
    /// There was an error initializing the SSL session
    SslError(SslError),
    /// There was an error communicating with the server
    IoError(IoError),
    /// The server sent an unexpected response
    BadResponse,
}

impl fmt::Display for ConnectError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        try!(fmt.write_str(error::Error::description(self)));
        match *self {
            ConnectError::InvalidUrl(ref msg) => write!(fmt, ": {}", msg),
            _ => Ok(())
        }
    }
}

impl error::Error for ConnectError {
    fn description(&self) -> &str {
        match *self {
            ConnectError::InvalidUrl(_) => "Invalid URL",
            ConnectError::MissingUser => "User missing in URL",
            ConnectError::DbError(_) => "An error from the Postgres server itself",
            ConnectError::MissingPassword => "The server requested a password but none was provided",
            ConnectError::UnsupportedAuthentication => {
                "The server requested an unsupported authentication method"
            }
            ConnectError::NoSslSupport => "The server does not support SSL",
            ConnectError::SslError(_) => "Error initiating SSL session",
            ConnectError::IoError(_) => "Error communicating with server",
            ConnectError::BadResponse => "The server returned an unexpected response",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            ConnectError::DbError(ref err) => Some(err as &error::Error),
            ConnectError::SslError(ref err) => Some(err as &error::Error),
            ConnectError::IoError(ref err) => Some(err as &error::Error),
            _ => None
        }
    }
}

impl error::FromError<IoError> for ConnectError {
    fn from_error(err: IoError) -> ConnectError {
        ConnectError::IoError(err)
    }
}

impl error::FromError<DbError> for ConnectError {
    fn from_error(err: DbError) -> ConnectError {
        ConnectError::DbError(err)
    }
}

impl error::FromError<SslError> for ConnectError {
    fn from_error(err: SslError) -> ConnectError {
        ConnectError::SslError(err)
    }
}

/// Represents the position of an error in a query
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ErrorPosition {
    /// A position in the original query
    Normal(u32),
    /// A position in an internally generated query
    Internal {
        /// The byte position
        position: u32,
        /// A query generated by the Postgres server
        query: String
    }
}

/// An error encountered when communicating with the Postgres server
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Error {
    /// An error reported by the Postgres server
    DbError(DbError),
    /// An error communicating with the Postgres server
    IoError(IoError),
    /// The communication channel with the Postgres server has desynchronized
    /// due to an earlier communications error.
    StreamDesynchronized,
    /// An attempt was made to convert between incompatible Rust and Postgres
    /// types
    WrongType(Type),
    /// An attempt was made to read from a column that does not exist
    InvalidColumn,
    /// A value was NULL but converted to a non-nullable Rust type
    WasNull,
    /// The server returned an unexpected response
    BadResponse,
    /// The server provided data that the client could not parse
    BadData,
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        try!(fmt.write_str(error::Error::description(self)));
        match *self {
            Error::WrongType(ref ty) => write!(fmt, ": saw type {:?}", ty),
            _ => Ok(()),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::DbError(_) => "An error reported by the Postgres server",
            Error::IoError(_) => "An error communicating with the Postgres server",
            Error::StreamDesynchronized => {
                "Communication with the server has desynchronized due to an earlier IO error"
            }
            Error::WrongType(_) => "Unexpected type",
            Error::InvalidColumn => "Invalid column",
            Error::WasNull => "The value was NULL",
            Error::BadResponse => "The server returned an unexpected response",
            Error::BadData => "The server provided data that the client could not parse",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::DbError(ref err) => Some(err as &error::Error),
            Error::IoError(ref err) => Some(err as &error::Error),
            _ => None
        }
    }
}

impl error::FromError<DbError> for Error {
    fn from_error(err: DbError) -> Error {
        Error::DbError(err)
    }
}

impl error::FromError<IoError> for Error {
    fn from_error(err: IoError) -> Error {
        Error::IoError(err)
    }
}
