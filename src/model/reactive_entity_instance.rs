use crate::model::ReactivePropertyInstance;
use indradb::VertexProperties;
use uuid::Uuid;
use std::collections::HashMap;

pub struct ReactiveEntityInstance<'a> {

    pub type_name: String,

    pub id: Uuid,

    pub description: String,

    pub properties: HashMap<String, ReactivePropertyInstance<'a>>,

}

impl ReactiveEntityInstance<'_> {

    pub fn from (properties: VertexProperties) -> ReactiveEntityInstance<'static> {
        let type_name = properties.vertex.t.0.clone();
        let id = properties.vertex.id.clone();
        let properties = properties.props
            .iter()
            .map(| named_property |

                (
                    named_property.name.clone(),
                    ReactivePropertyInstance::new(id.clone(), named_property.name.clone(), named_property.value.clone()) )
                )
            .collect();
        ReactiveEntityInstance {
            type_name,
            id,
            description: String::new(),
            properties
        }
    }

}
