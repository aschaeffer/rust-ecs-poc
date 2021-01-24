use crate::api::PropertyInstanceSetter;
use crate::model::ReactiveEntityInstance;
use crate::bidule::Stream;
use indradb::{VertexProperties, Type, NamedProperty, Vertex};
use serde_json::{json, Value};
use std::sync::{RwLock, Arc};
use uuid::Uuid;
use std::str::FromStr;
use log::debug;

pub type NumericOperationFunction = fn(f64) -> f64;

/// Generic implementation of numeric_operation operations with one input and one result.
///
/// The implementation is realized using reactive streams.
pub struct NumericOperation<'a> {
    pub f: NumericOperationFunction,

    pub internal_result: RwLock<Stream<'a, Value>>,

    pub entity: Arc<ReactiveEntityInstance<'a>>,

    pub handle_id: u128,
}

impl NumericOperation<'_> {
    pub const PROPERTY_NAME_NUMBER_1: &'static str = "number_1";
    pub const PROPERTY_NAME_RESULT_1: &'static str = "result_1";

    pub fn new<'a>(e: Arc<ReactiveEntityInstance<'static>>, f: NumericOperationFunction) -> NumericOperation<'static> {
        let handle_id = e.properties.get(Self::PROPERTY_NAME_RESULT_1).unwrap().id.as_u128();

        let internal_result = e.properties.get(Self::PROPERTY_NAME_NUMBER_1).unwrap()
            .stream.read().unwrap()
            .map(move |v| {
                json!(f(v.as_f64().unwrap()))
            });
        let numeric_operation = NumericOperation {
            f,
            internal_result: RwLock::new(internal_result),
            entity: e.clone(),
            handle_id
        };

        // Connect the internal result with the stream of the result property
        numeric_operation.internal_result.read().unwrap()
            .observe_with_handle(move |v| {
                debug!("Setting result of numeric gate: {}", v);
                e.set(Self::PROPERTY_NAME_RESULT_1.to_string(), json!(*v));
            }, handle_id);

        numeric_operation
    }

    /// TODO: Add guard: disconnect only if actually connected
    pub fn disconnect(&self) {
        debug!("Disconnect numeric operation {}", self.handle_id);
        self.internal_result.read().unwrap().remove(self.handle_id);
    }

    /// TODO: unit test
    pub fn type_name(&self) -> String {
        self.entity.type_name.clone()
    }
}

/// Automatically disconnect streams on destruction
impl Drop for NumericOperation<'_> {
    fn drop(&mut self) {
        debug!("Drop numeric operation");
        self.disconnect();
    }
}

/// Creates an numeric operation entity with 2 numeric inputs and 1 numeric output
pub fn create_numeric_operation_entity(type_name: String) -> ReactiveEntityInstance<'static> {
    let uuid = Uuid::new_v4();
    let t = Type::from_str(type_name.as_str()).unwrap();
    let property_lhs = NamedProperty {
        name: NumericOperation::PROPERTY_NAME_NUMBER_1.to_string(),
        value: json!(false)
    };
    let property_result = NamedProperty {
        name: NumericOperation::PROPERTY_NAME_RESULT_1.to_string(),
        value: json!(false)
    };
    let properties = vec![
        property_lhs,
        property_result
    ];
    let vertex_properties = VertexProperties {
        vertex: Vertex { id: uuid, t: t.clone() },
        props: properties.clone()
    };
    ReactiveEntityInstance::from(vertex_properties)
}

// TODO: pub fn create_numeric_operation_entity_with_id(type_name: String, id: String) -> ReactiveEntityInstance<'static> {
