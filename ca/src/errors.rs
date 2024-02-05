use std::{
    error,
    fmt::{Display, Debug, Error}
};

#[derive(Debug)]
pub struct SumError;

impl Display for SumError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid item To Sum")
    }
}

impl error::Error for SumError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> { None }
    fn cause(&self) -> Option<&dyn error::Error> { self.source() }
}

impl From<Error> for SumError {
    fn from(_value: Error) -> Self {
        SumError 
    }
}
