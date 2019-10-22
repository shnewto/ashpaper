// use nom::{error::ErrorKind, Err, Needed};
use std::error;
use std::fmt;
use std::str;

#[derive(PartialEq, Debug, Clone)]
pub enum Error {
    InputError(String),
    ParseError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::InputError(ref s) => write!(f, "{}", s),
            Error::ParseError(ref s) => write!(f, "{}", s),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        "AshPaper error"
    }
}

// impl<'a> From<Err<(&'a [u8], ErrorKind)>> for Error {
//     fn from(err: Err<(&[u8], ErrorKind)>) -> Self {
//         match err {
//             Err::Incomplete(n) => Error::from(n),
//             Err::Error(e) => Error::from(e),
//             Err::Failure(e) => Error::from(e),
//         }
//     }
// }

// impl<'a> From<(&'a [u8], ErrorKind)> for Error {
//     fn from(err: (&[u8], ErrorKind)) -> Self {
//         let string = format!(
//             "Parsing error: {}\n {:?}",
//             err.1.description(),
//             str::from_utf8(err.0)
//         );
//         Error::ParseError(string)
//     }
// }

// impl From<Needed> for Error {
//     fn from(needed: Needed) -> Self {
//         let string = match needed {
//             Needed::Unknown => format!("Data error: insufficient size, expectation unknown"),
//             Needed::Size(s) => format!("Data error: insufficient size, expected {} bytes", s),
//         };

//         Error::ParseIncomplete(string)
//     }
// }
