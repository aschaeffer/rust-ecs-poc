use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use core::fmt;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum SocketType {
    None,
    Input,
    Output
}

impl SocketType {
    pub fn none() -> Self {
        SocketType::None
    }
}

impl From<&str> for SocketType {
    fn from(value: &str) -> Self {
        return match value.to_lowercase().as_str() {
            "none" => Self::None,
            "input" => Self::Input,
            "output" => Self::Output,
            _ => Self::None
        }
    }
}

impl Display for SocketType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
