use std::error::Error;
use std::fmt;
use std::fmt::Display;

#[derive(Debug)]
pub enum ParseErr {
    // expected public fields
    Empty,
    Malformed(Box<dyn Error>),
}

// required by error trait
impl Display for ParseErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //unimplemented!();
        return write!(f, "Failed to parses todo ");
    }
}

#[derive(Debug)]
pub struct ReadErr {
    // expected public fields
    pub child_err: Box<dyn Error>,
}

// required by error trait
impl Display for ReadErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "Fail to read todo file ");
    }
}

impl Error for ParseErr {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        //unimplemented!();
        match self {
            ParseErr::Empty => None,
            ParseErr::Malformed(_e) => Some(self),
        }
    }
}

impl Error for ReadErr {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        // unimplemented!();
        Some(self.child_err.as_ref())
    }
}
