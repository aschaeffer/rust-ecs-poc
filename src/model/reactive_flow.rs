use crate::model::{ReactiveEntityInstance, ReactiveRelationInstance, Flow};
use indradb::EdgeKey;
use std::collections::HashMap;
use uuid::Uuid;
use std::sync::{Arc, RwLock};
use serde_json::Value;
use crate::api::{PropertyInstanceSetter, PropertyInstanceGetter};
use std::convert::TryFrom;

#[derive(Debug)]
pub struct ReactiveFlowCreationError;

pub struct ReactiveFlow {
    pub id: Uuid,

    // TODO: flow name
    // pub name: String,

    pub entity_instances: RwLock<HashMap<Uuid, Arc<ReactiveEntityInstance>>>,

    pub relation_instances: RwLock<HashMap<EdgeKey, Arc<ReactiveRelationInstance>>>,

    // pub wrapper: Arc<ReactiveEntityInstance>

    pub entities_added: RwLock<Vec<Uuid>>,

    pub entities_removed: RwLock<Vec<Uuid>>,

    pub relations_added: RwLock<Vec<EdgeKey>>,

    pub relations_removed: RwLock<Vec<EdgeKey>>,
}

impl ReactiveFlow {
    pub fn new(wrapper: Arc<ReactiveEntityInstance>) -> ReactiveFlow {
        let mut entity_instances = HashMap::new();
        entity_instances.insert(wrapper.id, wrapper.clone());
        ReactiveFlow {
            id: wrapper.id,
            entity_instances: RwLock::new(entity_instances),
            relation_instances: RwLock::new(HashMap::new()),
            // wrapper,
            entities_added: RwLock::new(Vec::new()),
            entities_removed: RwLock::new(Vec::new()),
            relations_added: RwLock::new(Vec::new()),
            relations_removed: RwLock::new(Vec::new()),
        }
    }

    pub fn has_entity(&self, entity_instance: Arc<ReactiveEntityInstance>) -> bool {
        self.entity_instances.read().unwrap().contains_key(&entity_instance.id)
    }

    pub fn has_entity_by_id(&self, id: Uuid) -> bool {
        self.entity_instances.read().unwrap().contains_key(&id)
    }

    pub fn get_entity(&self, id: Uuid) -> Option<Arc<ReactiveEntityInstance>> {
        let reader = self.entity_instances.read().unwrap();
        let instance = reader.get(&id);
        if instance.is_some() {
            return Some(instance.unwrap().clone());
        }
        None
    }

    pub fn add_entity(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        if !self.has_entity_by_id(entity_instance.id) {
            self.entity_instances.write().unwrap().insert(entity_instance.id, entity_instance.clone());
            self.entities_added.write().unwrap().push(entity_instance.id);
            // self.entities_removed.write().unwrap().remove(entity_instance.id);
        }
    }

    pub fn remove_entity(&self, id: Uuid) {
        self.entity_instances.write().unwrap().remove(&id);
        self.entities_removed.write().unwrap().push(id);
    }

    pub fn has_relation(&self, relation_instance: Arc<ReactiveRelationInstance>) -> bool{
        let edge_key = relation_instance.get_key();
        if edge_key.is_some() {
            return self.relation_instances.read().unwrap().contains_key(&edge_key.unwrap());
        }
        false
    }

    pub fn has_relation_by_key(&self, edge_key: EdgeKey) -> bool {
        self.relation_instances.read().unwrap().contains_key(&edge_key.clone())
    }

    pub fn get_relation(&self, edge_key: EdgeKey) -> Option<Arc<ReactiveRelationInstance>> {
        let reader = self.relation_instances.read().unwrap();
        let instance = reader.get(&edge_key);
        if instance.is_some() {
            let reactive_relation_instance = instance.unwrap();
            let reactive_relation_instance = reactive_relation_instance.clone();
            return Some(reactive_relation_instance);
        }
        None
    }

    pub fn add_relation(&self, relation_instance: Arc<ReactiveRelationInstance>) {
        let edge_key = relation_instance.get_key();
        if edge_key.is_some() {
            let edge_key = edge_key.unwrap();
            if !self.has_relation_by_key(edge_key.clone()) {
                self.relation_instances.write().unwrap().insert(edge_key.clone(), relation_instance.clone());
                self.relations_added.write().unwrap().push(edge_key.clone());
            }
        }
    }

    pub fn remove_relation(&self, edge_key: EdgeKey) {
        self.relation_instances.write().unwrap().remove(&edge_key.clone());
        self.relations_removed.write().unwrap().push(edge_key.clone());
    }

    pub fn tick(&self) {
        let reader = self.entity_instances.read().unwrap();
        for (_, entity_instance) in reader.iter() {
            entity_instance.tick();
        }
    }
}

impl TryFrom<Flow> for ReactiveFlow {
    type Error = ReactiveFlowCreationError;

    fn try_from(flow: Flow) -> Result<Self, ReactiveFlowCreationError> {
        let flow_id = flow.id;
        let mut entity_instances = HashMap::new();
        let mut wrapper = None;
        for entity_instance in flow.entity_instances {
            let id = entity_instance.id;
            let reactive_entity_instance = Arc::new(ReactiveEntityInstance::from(entity_instance));
            entity_instances.insert(id, reactive_entity_instance.clone());
            if id == flow_id {
                wrapper = Some(reactive_entity_instance.clone());
            }
        }
        if wrapper.is_none() {
            return Err(ReactiveFlowCreationError.into());
        }
        let mut relation_instances = HashMap::new();
        for relation_instance in flow.relation_instances {
            let edge_key = relation_instance.get_key();
            if edge_key.is_some() {
                let edge_key = edge_key.unwrap();
                let outbound = entity_instances.get(&relation_instance.outbound_id);
                if outbound.is_none() {
                    // outbound entity missing
                    return Err(ReactiveFlowCreationError.into());
                }
                let inbound = entity_instances.get(&relation_instance.inbound_id);
                if inbound.is_none() {
                    // inbound entity missing
                    return Err(ReactiveFlowCreationError.into());
                }
                let outbound = outbound.unwrap().clone();
                let inbound = inbound.unwrap().clone();
                let reactive_relation_instance = Arc::new(ReactiveRelationInstance::from_instance(outbound, inbound, relation_instance.clone()));
                relation_instances.insert(edge_key.clone(), reactive_relation_instance);
            }
        }
        return Ok(ReactiveFlow {
            id: flow_id,
            entity_instances: RwLock::new(entity_instances),
            relation_instances: RwLock::new(relation_instances),
            // wrapper: wrapper.unwrap(),
            entities_added: RwLock::new(Vec::new()),
            entities_removed: RwLock::new(Vec::new()),
            relations_added: RwLock::new(Vec::new()),
            relations_removed: RwLock::new(Vec::new()),
        })
    }
}

impl PropertyInstanceGetter for ReactiveFlow {
    fn get<S: Into<String>>(&self, property_name: S) -> Option<Value> {
        self.get_entity(self.id).and_then(|e| e.properties.get(&property_name.into()).and_then(|p| Some(p.get())))
    }

    fn as_bool<S: Into<String>>(&self, property_name: S) -> Option<bool> {
        self.get_entity(self.id).and_then(|e| e.properties.get(&property_name.into()).and_then(|p| p.as_bool()))
    }

    fn as_u64<S: Into<String>>(&self, property_name: S) -> Option<u64> {
        self.get_entity(self.id).and_then(|e| e.properties.get(&property_name.into()).and_then(|p| p.as_u64()))
    }

    fn as_i64<S: Into<String>>(&self, property_name: S) -> Option<i64> {
        self.get_entity(self.id).and_then(|e| e.properties.get(&property_name.into()).and_then(|p| p.as_i64()))
    }

    fn as_f64<S: Into<String>>(&self, property_name: S) -> Option<f64> {
        self.get_entity(self.id).and_then(|e| e.properties.get(&property_name.into()).and_then(|p| p.as_f64()))
    }

    fn as_string<S: Into<String>>(&self, property_name: S) -> Option<String> {
        self.get_entity(self.id).and_then(|e| e.properties.get(&property_name.into()).and_then(|p| p.as_string()))
    }
}

impl PropertyInstanceSetter for ReactiveFlow {
    fn set<S: Into<String>>(&self, property_name: S, value: Value) {
        if let Some(instance) = self.get_entity(self.id) {
            if let Some(instance) = instance.properties.get(&property_name.into()) {
                instance.set(value);
            }
        }
    }

    // TODO: fn set(&self, Map<String, Value>
    // TODO: Set values transactional: first set all values internally, then send all affected streams
}
