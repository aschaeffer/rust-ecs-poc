use std::sync::{RwLock, Arc};
use crate::bidule::Stream;
use serde_json::Value;
use crate::model::ReactiveEntityInstance;
use indradb::VertexProperties;
use log::debug;

/// Generic implementation of an entity instance representing a constant
/// The value is provided by a
pub struct ConstValue<'a> {
    pub internal_value: RwLock<Stream<'a, Value>>,

    // TODO:
    pub entity: Arc<ReactiveEntityInstance<'a>>,

    pub handle_id: u128,
}

impl ConstValue<'_> {
    /// The name of the non-specialized type.
    ///
    /// There might exist more specialized types of ConstValue which
    /// does more than storing the value within a defined property
    /// and provide convenient accessors.
    pub const DEFAULT_TYPE_NAME: &'static str = "value";

    pub const PROPERTY_NAME_VALUE: &'static str = "value";

    pub fn new<'a>(entity: Arc<ReactiveEntityInstance<'static>> /*, initial_value: Value */) -> ConstValue<'static> {
        let handle_id = entity.properties.get(ConstValue::PROPERTY_NAME_VALUE).unwrap().id.as_u128();

        let const_value = ConstValue {
            internal_value: RwLock::new(Stream::new()),
            entity: Arc::clone(&entity),
            handle_id
        };

        let e = Arc::clone(&entity);

        // Connect the internal result with the stream of the result property
        const_value.internal_value.read().unwrap()
            .observe_with_handle(move |v| {
                e.properties.get(ConstValue::PROPERTY_NAME_VALUE).unwrap().set(v.clone());
            }, handle_id);

        const_value
    }

    pub fn from(properties: VertexProperties) -> ConstValue<'static> {
        let entity = Arc::new(ReactiveEntityInstance::from(properties));

        // TODO: Make this more simple: entity.get_handle_id(ConstValue::PROPERTY_NAME_VALUE)
        let handle_id = entity.properties.get(ConstValue::PROPERTY_NAME_VALUE).unwrap().id.as_u128();

        let const_value = ConstValue {
            internal_value: RwLock::new(Stream::new()),
            entity: Arc::clone(&entity),
            handle_id,
        };

        let e = Arc::clone(&entity);

        // Connect the internal result with the stream of the result property
        const_value.internal_value.read().unwrap()
            .observe_with_handle(move |v| {
                e.properties.get(ConstValue::PROPERTY_NAME_VALUE).unwrap().set(v.clone());
            }, handle_id);

        return const_value;
    }

    pub fn set(&self, value: &Value) {
        self.internal_value.read().unwrap().send(value);
    }

    pub fn get(&self) -> Option<Value> {
        self.entity.properties.get(ConstValue::PROPERTY_NAME_VALUE)
            .and_then(|p| Some(p.value.read().unwrap().clone()))
    }

    pub fn disconnect(&self) {
        debug!("Disconnect ConstValue {}", self.handle_id);
        self.entity.properties.get(ConstValue::PROPERTY_NAME_VALUE).unwrap()
            .stream.read().unwrap().remove(self.handle_id);
    }
}

/// Automatically disconnect streams on destruction
///
/// TODO: Unit Test
impl Drop for ConstValue<'_> {
    fn drop(&mut self) {
        debug!("Drop const value");
        self.disconnect();
    }
}

// TODO: Allow to specify a closure which provides the current value
//       => 1. Once (num_cpus)
//       => 2. Repeated (timestamp)
// TODO: ConstantsManager
//       => List of closures in HashMap<String, fn () -> Value>
//       => Add constants dynamically


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
