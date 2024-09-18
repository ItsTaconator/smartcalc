use std::fmt::Display;

pub struct InvalidExpression {
    pub message: String,
}

impl Default for InvalidExpression {
    fn default() -> Self {
        Self {
            message: "Expression is invalid".to_owned(),
        }
    }
}

impl Display for InvalidExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}
