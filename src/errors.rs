use std::error::Error;
use std::num::ParseIntError;
use std::fmt;

#[derive(Debug)]
pub enum ProgramArgumentError {
    LackingPortError,
    LackingHostError,
    PortParsingError(ParseIntError)
}

impl fmt::Display for ProgramArgumentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self.description(), f)
    }
}

impl Error for ProgramArgumentError {
    fn description(&self) -> &str {
        match *self {
          ProgramArgumentError::LackingPortError => "Trzeba podać port!",
          ProgramArgumentError::LackingHostError => "Trzeba podać adres gościa!",
          ProgramArgumentError::PortParsingError(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            ProgramArgumentError::PortParsingError(ref err) => Some(err),
            _ => None,
        }
    }
}

impl From<ParseIntError> for ProgramArgumentError {
    fn from(err: ParseIntError) -> ProgramArgumentError {
        ProgramArgumentError::PortParsingError(err)
    }
}