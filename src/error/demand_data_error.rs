use std::fmt;

//TODO: Put to use (or not).
#[derive(Debug)]
pub struct DemandDataError{}

impl std::error::Error for DemandDataError {}
impl fmt::Display for DemandDataError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid demand data.")
    }
}