use program::Rule;
use std::error;
use std::fmt;
use std::str;

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    ParseError(String),
    InputError(String),
    ProgramError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::ParseError(ref s) => write!(f, "{}", s),
            Error::InputError(ref s) => write!(f, "{}", s),
            Error::ProgramError(ref s) => write!(f, "{}", s),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        "AshPaper error"
    }
}

impl From<pest::error::Error<Rule>> for Error {
    fn from(err: pest::error::Error<Rule>) -> Self {
        Error::ParseError(format!("{}", err))
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::InputError(format!("{}", err))
    }
}

#[cfg(test)]
mod tests {
    // None of these errors should be possible... how can we "test" them?
}
