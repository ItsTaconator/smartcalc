#[derive(Clone, Debug)]
pub struct Variable {
    pub key: Box<str>,
    pub value: f64,
    pub aliases: Option<Vec<String>>,
}

impl Variable {
    pub fn new<S: ToString>(key: S, value: f64, aliases: Option<Vec<String>>) -> Self {
        Self {
            key: key.to_string().into_boxed_str(),
            value,
            aliases,
        }
    }

    pub fn new_f64<F: Into<f64>, S: ToString>(
        key: S,
        value: F,
        aliases: Option<Vec<String>>,
    ) -> Self {
        let fl: f64 = value.into();
        Self {
            key: key.to_string().into_boxed_str(),
            value: fl,
            aliases,
        }
    }

    pub fn exists<S: ToString>(self, key: S) -> bool {
        let key = key.to_string();
        if *self.key == key {
            return true;
        } else if let Some(aliases) = self.aliases {
            if aliases.contains(&key) {
                return true;
            }
        }

        false
    }
}
