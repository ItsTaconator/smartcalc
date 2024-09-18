use serde_derive::{Deserialize, Serialize};
#[derive(Deserialize, Serialize)]
pub struct Config {
    pub time_expression: bool,
}
