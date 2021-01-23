use crate::api::PropertyInstanceSetter;
use crate::model::ReactiveEntityInstance;
use crate::reactive::{Expression, OperatorPosition, ExpressionValue};
use crate::bidule::Stream;
use indradb::{VertexProperties, Type, NamedProperty, Vertex};
use serde_json::json;
use std::sync::{RwLock, Arc};
use uuid::Uuid;
use std::str::FromStr;
use log::debug;

pub static PROPERTY_NAME_BIT_1: &'static str = "bit_1";
pub static PROPERTY_NAME_BIT_2: &'static str = "bit_2";
pub static PROPERTY_NAME_RESULT_1: &'static str = "result_1";

pub static LHS_DEFAULT: bool = false;
pub static RHS_DEFAULT: bool = false;

pub type BinaryExpressionValue = ExpressionValue<bool>;

pub type BinaryOperation = fn(bool, bool) -> bool;

/// Generic implementation of binary operations with two inputs (LHS,RHS) and one result.
///
/// The implementation is realized using reactive streams.
pub struct LogicalGate<'a> {
    pub lhs: RwLock<Stream<'a, BinaryExpressionValue>>,

    pub rhs: RwLock<Stream<'a, BinaryExpressionValue>>,

    pub f: BinaryOperation,

    pub internal_result: RwLock<Stream<'a, bool>>,

    pub entity: Arc<ReactiveEntityInstance<'a>>,

    pub handle_id: u128,
}

impl LogicalGate<'_> {
    pub fn new<'a>(e: Arc<ReactiveEntityInstance<'static>>, f: BinaryOperation) -> LogicalGate<'static> {
        let lhs = e.properties.get(PROPERTY_NAME_BIT_1).unwrap()
            .stream.read().unwrap()
            .map(|v| match v.as_bool() {
                Some(b) => (OperatorPosition::LHS, b),
                None => (OperatorPosition::LHS, LHS_DEFAULT),
            });
        let rhs = e.properties.get(PROPERTY_NAME_BIT_2).unwrap()
            .stream.read().unwrap()
            .map(|v| -> BinaryExpressionValue {
                match v.as_bool() {
                    Some(b) => (OperatorPosition::RHS, b),
                    None => (OperatorPosition::RHS, RHS_DEFAULT),
                }
            });

        let expression = lhs.merge(&rhs).fold(
            Expression::new(LHS_DEFAULT, RHS_DEFAULT),
            |old_state, (o, value)| match *o {
                OperatorPosition::LHS => old_state.lhs(*value),
                OperatorPosition::RHS => old_state.rhs(*value),
            },
        );

        // The internal result
        let internal_result = expression.map(move |e| f(e.lhs, e.rhs));

        // TODO: handle result based on outbound property id and inbound property id
        let handle_id = e.properties.get(PROPERTY_NAME_RESULT_1).unwrap().id.as_u128();

        let logical_gate = LogicalGate {
            lhs: RwLock::new(lhs),
            rhs: RwLock::new(rhs),
            f,
            internal_result: RwLock::new(internal_result),
            entity: e.clone(),
            handle_id
        };

        // Connect the internal result with the stream of the result property
        logical_gate.internal_result.read().unwrap()
            .observe_with_handle(move |v| {
                debug!("Setting result of logical gate: {}", v);
                e.set(PROPERTY_NAME_RESULT_1.to_string(), json!(*v));
            }, handle_id);

        logical_gate
    }

    /// TODO: Add guard: disconnect only if actually connected
    pub fn disconnect(&self) {
        debug!("Disconnect logical gate {}", self.handle_id);
        self.internal_result.read().unwrap().remove(self.handle_id);
    }

    /// TODO: unit test
    pub fn type_name(&self) -> String {
        self.entity.type_name.clone()
    }
}

/// Automatically disconnect streams on destruction
impl Drop for LogicalGate<'_> {
    fn drop(&mut self) {
        debug!("Drop logical gate");
        self.disconnect();
    }
}

/// Creates a logical gate entity with 2 boolean inputs and 1 boolean output
pub fn create_logical_gate_entity(type_name: String) -> ReactiveEntityInstance<'static> {
    let uuid = Uuid::new_v4();
    let t = Type::from_str(type_name.as_str()).unwrap();
    let property_lhs = NamedProperty {
        name: PROPERTY_NAME_BIT_1.to_string(),
        value: json!(false)
    };
    let property_rhs = NamedProperty {
        name: PROPERTY_NAME_BIT_2.to_string(),
        value: json!(false)
    };
    let property_result = NamedProperty {
        name: PROPERTY_NAME_RESULT_1.to_string(),
        value: json!(false)
    };
    let properties = vec![
        property_lhs,
        property_rhs,
        property_result
    ];
    let vertex_properties = VertexProperties {
        vertex: Vertex { id: uuid, t: t.clone() },
        props: properties.clone()
    };
    ReactiveEntityInstance::from(vertex_properties)
}

// TODO: pub fn create_logical_gate_entity_with_id(type_name: String, id: String) -> ReactiveEntityInstance<'static> {
