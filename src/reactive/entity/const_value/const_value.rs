use std::convert::AsRef;
use std::sync::{Arc, RwLock};

use indradb::VertexProperties;
use log::debug;
use serde_json::Value;

use crate::bidule::Stream;
use crate::model::ReactiveEntityInstance;
use crate::reactive::ConstValueProperties;

/// Generic implementation of an entity instance representing a constant
/// The value is provided by a
pub struct ConstValue<'a> {
    pub internal_value: RwLock<Stream<'a, Value>>,

    // TODO:
    pub entity: Arc<ReactiveEntityInstance>,

    pub handle_id: u128,
}

#[allow(dead_code)]
impl ConstValue<'_> {
    /// The name of the non-specialized type.
    ///
    /// There might exist more specialized types of ConstValue which
    /// does more than storing the value within a defined property
    /// and provide convenient accessors.
    pub const DEFAULT_TYPE_NAME: &'static str = "value";

    // pub const PROPERTY_NAME_VALUE: &'static str = "value";

    // pub fn new<'a>(entity: Arc<ReactiveEntityInstance> /*, initial_value: Value */) -> ConstValue<'static> {
    //
    //     let handle_id = entity.properties.get(ConstValueProperties::VALUE.as_ref()).unwrap().id.as_u128();
    //
    //     let const_value = ConstValue {
    //         internal_value: RwLock::new(Stream::new()),
    //         entity: Arc::clone(&entity),
    //         handle_id
    //     };
    //
    //     let e = Arc::clone(&entity);
    //
    //     // Connect the internal result with the stream of the result property
    //     const_value.internal_value.read().unwrap()
    //         .observe_with_handle(move |v| {
    //             e.properties.get(ConstValueProperties::VALUE.as_ref()).unwrap().set(v.clone());
    //         }, handle_id);
    //
    //     const_value
    // }

    // TODO: pub fn new() -> ConstValue<'static> {} (Creates a new reactive entity instance?)
    //

    pub fn set(&self, value: &Value) {
        self.internal_value.read().unwrap().send(value);
    }

    pub fn get(&self) -> Option<Value> {
        self.entity.properties.get(ConstValueProperties::VALUE.as_ref())
            .and_then(|p| Some(p.value.read().unwrap().clone()))
    }

    // pub fn get_handle_id(&self) -> u128 {
    //     self.entity.properties.get(ConstValueProperties::VALUE.as_ref()).unwrap().id.as_u128()
    // }

    pub fn disconnect(&self) {
        debug!("Disconnect ConstValue {}", self.handle_id);
        self.entity.properties.get(ConstValueProperties::VALUE.as_ref()).unwrap()
            .stream.read().unwrap().remove(self.handle_id);
    }
}

impl From<VertexProperties> for ConstValue<'static> {
    fn from(properties: VertexProperties) -> Self {
        let entity = Arc::new(ReactiveEntityInstance::from(properties));
        return entity.into();

        // let handle_id = entity.properties.get(ConstValueProperties::VALUE.as_ref()).unwrap().id.as_u128();
        //
        // let const_value = ConstValue {
        //     internal_value: RwLock::new(Stream::new()),
        //     entity: Arc::clone(&entity),
        //     handle_id,
        // };
        //
        // let e = Arc::clone(&entity);
        //
        // // Connect the internal result with the stream of the result property
        // const_value.internal_value.read().unwrap()
        //     .observe_with_handle(move |v| {
        //         e.properties.get(ConstValueProperties::VALUE.as_ref()).unwrap().set(v.clone());
        //     }, handle_id);
        //
        // return const_value;
    }
}

impl From<Arc<ReactiveEntityInstance>> for ConstValue<'static> {
    fn from(entity: Arc<ReactiveEntityInstance>) -> Self {

        let handle_id = entity.properties.get(ConstValueProperties::VALUE.as_ref()).unwrap().id.as_u128();

        let const_value = ConstValue {
            internal_value: RwLock::new(Stream::new()),
            entity: Arc::clone(&entity),
            handle_id,
        };

        let e = Arc::clone(&entity);

        // Connect the internal result with the stream of the result property
        const_value.internal_value.read().unwrap()
            .observe_with_handle(move |v| {
                e.properties.get(ConstValueProperties::VALUE.as_ref()).unwrap().set(v.clone());
            }, handle_id);

        return const_value;
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
