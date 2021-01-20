use std::sync::{RwLock, Arc};
use crate::bidule::Stream;
use serde_json::Value;
use crate::model::ReactiveEntityInstance;
use indradb::VertexProperties;

/// Generic implementation of an entity instance representing a constant
/// The value is provided by a
pub struct ConstValue<'a> {
    pub internal_value: RwLock<Stream<'a, Value>>,

    // TODO:
    pub entity_instance: Arc<ReactiveEntityInstance<'a>>,
}

impl ConstValue<'_> {
    pub fn new<'a>(entity_instance: Arc<ReactiveEntityInstance<'static>> /*, initial_value: Value */) -> ConstValue<'static> {
        let const_value = ConstValue {
            internal_value: RwLock::new(Stream::new()),
            entity_instance: Arc::clone(&entity_instance),
        };

        let e = Arc::clone(&entity_instance);

        // Connect the internal result with the stream of the result property
        const_value.internal_value.read().unwrap()
            .observe(move |v| {
                e.properties.get("value").unwrap().set(v.clone());
            });

        const_value
    }

    pub fn from(properties: VertexProperties) -> ConstValue<'static> {
        let entity_instance = Arc::new(ReactiveEntityInstance::from(properties));

        let const_value = ConstValue {
            internal_value: RwLock::new(Stream::new()),
            entity_instance: Arc::clone(&entity_instance),
        };

        let e = Arc::clone(&entity_instance);

        // Connect the internal result with the stream of the result property
        const_value.internal_value.read().unwrap()
            .observe(move |v| {
                e.properties.get("value").unwrap().set(v.clone());
            });

        return const_value;
    }

    pub fn set(&self, value: &Value) {
        self.internal_value.read().unwrap().send(value);
    }

    pub fn get(&self) -> Option<Value> {
        self.entity_instance.properties.get("value")
            .and_then(|p| Some(p.value.read().unwrap().clone()))
    }
}

// // TODO: move to a service and tests
// fn init_const_num_cpus (properties: VertexProperties) -> ConstValue<'static> {
//     let const_value= ConstValue::from(properties);
//     const_value.set(&json!(num_cpus::get()));
//     const_value.internal_value.read().unwrap().send(&json!(num_cpus::get()));
//     // TODO: make it shorter: const_value.set(&json!(num_cpus::get()))
//     const_value
// }
//
// // TODO: move to a service and tests
// fn init_const_num_cpus_physical (properties: VertexProperties) -> ConstValue<'static> {
//     let const_value= ConstValue::from(properties);
//     const_value.internal_value.read().unwrap().send(&json!(num_cpus::get_physical()));
//     // TODO: make it shorter:
//     const_value
// }
