use std::io;
use std::fmt;
use std::error;
use bitcoin::consensus::encode;

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    //NetworkError(network::Error),
    DataError(encode::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::IoError(ref e) => fmt::Display::fmt(e, f),
            //Error::NetworkError(ref e) => fmt::Display::fmt(e, f),
            Error::DataError(ref e) => fmt::Display::fmt(e, f),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::IoError(ref e) => e.description(),
            //Error::NetworkError(ref e) => e.description(),
            Error::DataError(ref e) => e.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::IoError(ref e) => Some(e),
            //Error::NetworkError(ref e) => Some(e),
            Error::DataError(ref e) => Some(e),
        }
    }
}

#[doc(hidden)]
impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::IoError(error)
    }
}
