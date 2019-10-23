use program::Rule;
use std::fmt;

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
    use super::*;

    #[test]
    fn display() {
        let parse = "parse error".to_string();
        let input = "input error".to_string();
        let progam = "program error".to_string();

        assert_eq!(parse, format!("{}", Error::ParseError(parse.clone())));
        assert_eq!(input, format!("{}", Error::InputError(input.clone())));
        assert_eq!(progam, format!("{}", Error::ProgramError(progam.clone())));
    }

    #[test]
    fn io_err() {
        let err_str = "IO Errored!";
        let error = std::io::Error::new(std::io::ErrorKind::Other, err_str);

        assert_eq!(err_str.to_string(), format!("{}", Error::from(error)));
    }

    #[test]
    fn pest_err() {
        let err_str = " --> 1:1\n  |\n1 | \n  | ^---\n  |\n  = unexpected pop; expected push";

        let input = "";
        let pos = pest::Position::from_start(input);
        let error = pest::error::Error::new_from_pos(
            pest::error::ErrorVariant::ParsingError {
                positives: vec![Rule::push],
                negatives: vec![Rule::pop],
            },
            pos,
        );

        assert_eq!(err_str.to_string(), format!("{}", Error::from(error)));
    }
}
