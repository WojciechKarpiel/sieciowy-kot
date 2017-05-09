use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum ProgramArgumentError {
    LackingPortError,
    LackingHostError,
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
          ProgramArgumentError::LackingHostError => "Trzeba podać adres gościa!"
        }
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}