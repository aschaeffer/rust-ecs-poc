use serde::{Deserialize, Serialize};
use crate::model::{SocketType, DataType};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PropertyType {
    pub name: String,
    pub data_type: DataType, // TODO: change to enum

    #[serde(default = "SocketType::none")]
    pub socket_type: SocketType,
}

impl PropertyType {
    pub fn new<S: Into<String>>(name: S, data_type: DataType) -> PropertyType {
        PropertyType {
            name: name.into(),
            data_type,
            socket_type: SocketType::None,
        }
    }

    pub fn input<S: Into<String>>(name: S, data_type: DataType) -> PropertyType {
        PropertyType {
            name: name.into(),
            data_type,
            socket_type: SocketType::Input,
        }
    }

    pub fn output<S: Into<String>>(name: S, data_type: DataType) -> PropertyType {
        PropertyType {
            name: name.into(),
            data_type,
            socket_type: SocketType::Output,
        }
    }
}
