use std::fmt;
use std::fmt::Display;
use std::error::Error;

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
       return write!(f, "{}", self.to_string());
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
        return write!(f, "{}", self.child_err.to_string());
    }
}

impl Error for ParseErr {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        //unimplemented!();
        match self {
            ParseErr::Empty => None,
            ParseErr::Malformed(e) => Some(e.as_ref()),
        }
    }
}

impl Error for ReadErr {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        // unimplemented!();
        Some(self.child_err.as_ref())
    }
}