use crate::location::Location;

#[derive(Debug, Clone, PartialEq)]
pub struct Comment {
    pub lines: Vec<String>,
    pub start_location: Location,
    pub end_location: Location,
}
