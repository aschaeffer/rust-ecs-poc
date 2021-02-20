use std::collections::HashMap;
use std::sync::Arc;

use serde_json::Value;
use uuid::Uuid;

use crate::api::{ReactiveRelationInstanceManager, ReactiveRelationInstanceCreationError};
use crate::model::ReactiveRelationInstance;
use indradb::{EdgeKey, Type};

#[allow(dead_code)]
pub struct ReactiveRelationInstanceBuilder {
    outbound_id: Uuid,
    type_name: String,
    inbound_id: Uuid,
    properties: HashMap<String, Value>
}

#[allow(dead_code)]
impl ReactiveRelationInstanceBuilder {
    pub fn new<S: Into<String>>(outbound_id: Uuid, type_name: S, inbound_id: Uuid)
        -> ReactiveRelationInstanceBuilder
    {
        ReactiveRelationInstanceBuilder {
            outbound_id,
            type_name: type_name.into(),
            inbound_id,
            properties: HashMap::new(),
        }
    }

    pub fn property<'a, S: Into<String>>(&'a mut self, property_name: S, value: Value) -> &'a mut ReactiveRelationInstanceBuilder {
        self.properties.insert(property_name.into(), value);
        self
    }

    pub fn create<'a>(&'a mut self, reactive_relation_instance_manager: Arc<dyn ReactiveRelationInstanceManager>)
        -> Result<Arc<ReactiveRelationInstance>, ReactiveRelationInstanceCreationError>
    {
        match Type::new(self.type_name.clone()) {
            Ok(t) => {
                let edge_key = EdgeKey::new(self.outbound_id, t, self.inbound_id);
                reactive_relation_instance_manager.create(edge_key, self.properties.clone())
            },
            Err(_err) => Err(ReactiveRelationInstanceCreationError.into())
        }
    }

}
