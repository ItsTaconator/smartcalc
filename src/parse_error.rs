#[derive(Debug)]
pub(crate) struct ParseError {
    pub(crate) message: String,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ParseError {}

impl ParseError {
    pub(crate) fn new<S: AsRef<str>>(message: S) -> Self {
        ParseError {
            message: message.as_ref().to_string(),
        }
    }
}
