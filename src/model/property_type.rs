use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PropertyType {

    pub name: String,
    pub data_type: String, // TODO: change to enum

}

impl PropertyType {

    pub fn new(name: String, data_type: String) -> PropertyType {
        PropertyType {
            name,
            data_type
        }
    }

}
