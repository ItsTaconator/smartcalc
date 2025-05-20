use crate::parameter_documentation::ParameterDocumentation;

#[derive(Clone)]
pub struct Command {
    pub name: &'static str,
    pub help_text: Option<&'static str>,
    pub action: fn(Option<&str>),
    pub aliases: Option<Vec<&'static str>>,
    pub parameter_documentation: Option<ParameterDocumentation>,
}
