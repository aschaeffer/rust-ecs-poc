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

pub static LHS_DEFAULT: f64 = 0.0;
pub static RHS_DEFAULT: f64 = 0.0;

pub type NumericExpressionValue = ExpressionValue<f64>;

pub type NumericGateFunction = fn(f64, f64) -> f64;

/// Generic implementation of numeric_gates operations with two inputs (LHS,RHS) and one result.
///
/// The implementation is realized using reactive streams.
pub struct NumericGate<'a> {
    pub lhs: RwLock<Stream<'a, NumericExpressionValue>>,

    pub rhs: RwLock<Stream<'a, NumericExpressionValue>>,

    pub f: NumericGateFunction,

    pub internal_result: RwLock<Stream<'a, f64>>,

    pub entity: Arc<ReactiveEntityInstance<'a>>,

    pub handle_id: u128,
}

impl NumericGate<'_> {
    const PROPERTY_NAME_NUMBER_1: &'static str = "number_1";
    const PROPERTY_NAME_NUMBER_2: &'static str = "number_2";
    const PROPERTY_NAME_RESULT_1: &'static str = "result_1";

    pub fn new<'a>(e: Arc<ReactiveEntityInstance<'static>>, f: NumericGateFunction) -> NumericGate<'static> {
        let lhs = e.properties.get(Self::PROPERTY_NAME_NUMBER_1).unwrap()
            .stream.read().unwrap()
            .map(|v| match v.as_f64() {
                Some(b) => (OperatorPosition::LHS, b),
                None => (OperatorPosition::LHS, LHS_DEFAULT),
            });
        let rhs = e.properties.get(Self::PROPERTY_NAME_NUMBER_2).unwrap()
            .stream.read().unwrap()
            .map(|v| -> NumericExpressionValue {
                match v.as_f64() {
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
        let handle_id = e.properties.get(Self::PROPERTY_NAME_RESULT_1).unwrap().id.as_u128();

        let numeric_gate = NumericGate {
            lhs: RwLock::new(lhs),
            rhs: RwLock::new(rhs),
            f,
            internal_result: RwLock::new(internal_result),
            entity: e.clone(),
            handle_id
        };

        // Connect the internal result with the stream of the result property
        numeric_gate.internal_result.read().unwrap()
            .observe_with_handle(move |v| {
                debug!("Setting result of numeric gate: {}", v);
                e.set(Self::PROPERTY_NAME_RESULT_1.to_string(), json!(*v));
            }, handle_id);

        numeric_gate
    }

    /// TODO: Add guard: disconnect only if actually connected
    pub fn disconnect(&self) {
        debug!("Disconnect numeric gate {}", self.handle_id);
        self.internal_result.read().unwrap().remove(self.handle_id);
    }

    /// TODO: unit test
    pub fn type_name(&self) -> String {
        self.entity.type_name.clone()
    }
}

/// Automatically disconnect streams on destruction
impl Drop for NumericGate<'_> {
    fn drop(&mut self) {
        debug!("Drop numeric gate");
        self.disconnect();
    }
}

/// Creates an numeric_gates gate entity with 2 numeric inputs and 1 numeric output
pub fn create_numeric_gate_entity(type_name: String) -> ReactiveEntityInstance<'static> {
    let uuid = Uuid::new_v4();
    let t = Type::from_str(type_name.as_str()).unwrap();
    let property_lhs = NamedProperty {
        name: NumericGate::PROPERTY_NAME_NUMBER_1.to_string(),
        value: json!(false)
    };
    let property_rhs = NamedProperty {
        name: NumericGate::PROPERTY_NAME_NUMBER_2.to_string(),
        value: json!(false)
    };
    let property_result = NamedProperty {
        name: NumericGate::PROPERTY_NAME_RESULT_1.to_string(),
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

// TODO: pub fn create_numeric_gate_entity_with_id(type_name: String, id: String) -> ReactiveEntityInstance<'static> {
