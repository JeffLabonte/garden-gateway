use std::fmt::{Display, Formatter, Result};

mod constants;

trait BaseException {
    fn get_message(&self) -> String;
}

pub struct UniqueConstraintDbException {}

impl BaseException for UniqueConstraintDbException {
    fn get_message(&self) -> String {
        self.message.clone()
    }
}

impl Display for dyn BaseException {
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        write!(fmt, "An Error Occured:\n {}", self.get_message())
    }
}
