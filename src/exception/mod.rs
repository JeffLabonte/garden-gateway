use std::fmt::{Display, Formatter, Result};

use self::constants::UNIQUE_CONSTRAINT_DB_EXCEPTION_MESSAGE;

mod constants;

trait BaseException {
    fn get_message(&self) -> String;
}

pub struct UniqueConstraintDbException {}

impl BaseException for UniqueConstraintDbException {
    fn get_message(&self) -> String {
        UNIQUE_CONSTRAINT_DB_EXCEPTION_MESSAGE.to_string()
    }
}

impl Display for dyn BaseException {
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        write!(fmt, "An Error Occured:\n {}", self.get_message())
    }
}
