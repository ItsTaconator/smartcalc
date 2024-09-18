#[derive(Clone)]
pub struct Command {
    pub name: &'static str,
    pub help_text: Option<&'static str>,
    pub action: fn(&String),
    pub aliases: Option<Vec<&'static str>>,
}
