use std::cmp::Ordering;

use crate::location::Location;

#[derive(Clone, PartialEq, Debug)]
pub struct Error {
    pub message: String,
    pub location: Location,
}

impl Error {
    pub fn new(message: String, location: Location) -> Self {
        Self { message, location }
    }
}

impl From<Error> for Vec<Error> {
    fn from(error: Error) -> Self {
        vec![error]
    }
}
