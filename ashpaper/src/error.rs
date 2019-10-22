use pest;
use::program;
use std::error;
use std::fmt;
use std::str;

#[derive(PartialEq, Debug, Clone)]
pub enum Error {
    ParseError(String),
    InputError(String),
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

impl From<pest::error::Error<program::Rule>> for Error {
    fn from(err: pest::error::Error<program::Rule>) -> Self {
        match err.variant {
            pest::error::ErrorVariant::ParsingError { .. } => {
                Error::ParseError("TODO".to_string())
                }
            pest::error::ErrorVariant::CustomError { ref message } => {
                Error::ParseError(message.to_string())
            }
        }
    }
}

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
