use std::convert::AsRef;
use std::sync::{Arc, RwLock};

use log::debug;
use serde_json::{json, Value};

use crate::api::{PropertyInstanceGetter, PropertyInstanceSetter};
use crate::bidule::Stream;
use crate::model::ReactiveEntityInstance;
use crate::reactive::entity::expression::{Expression, ExpressionValue, OperatorPosition};
use crate::reactive::entity::gate::Gate;
use crate::reactive::entity::operation::Operation;
use crate::reactive::entity::numeric_gate::NumericGateProperties;
use crate::reactive::Disconnectable;

pub type NumericExpressionValue = ExpressionValue<f64>;

// TODO: Floating Point / Numeric Date / Different types for input and output (e.g. abs)
pub type NumericGateFunction<T> = fn(T, T) -> T;

/// Generic implementation of numeric_gates operations with two inputs (LHS,RHS) and one result.
///
/// The implementation is realized using reactive streams.
pub struct NumericGate<'a> {
    pub lhs: RwLock<Stream<'a, NumericExpressionValue>>,

    pub rhs: RwLock<Stream<'a, NumericExpressionValue>>,

    pub f: NumericGateFunction<f64>,

    pub internal_result: RwLock<Stream<'a, f64>>,

    pub entity: Arc<ReactiveEntityInstance>,

    pub handle_id: u128,
}

impl NumericGate<'_> {
    pub fn new<'a>(e: Arc<ReactiveEntityInstance>, f: NumericGateFunction<f64>) -> NumericGate<'static> {
        let lhs = e.properties.get(NumericGateProperties::LHS.as_ref()).unwrap()
            .stream.read().unwrap()
            .map(|v| match v.as_f64() {
                Some(b) => (OperatorPosition::LHS, b),
                None => (OperatorPosition::LHS, NumericGateProperties::LHS.default_value()),
            });
        let rhs = e.properties.get(NumericGateProperties::RHS.as_ref()).unwrap()
            .stream.read().unwrap()
            .map(|v| -> NumericExpressionValue {
                match v.as_f64() {
                    Some(b) => (OperatorPosition::RHS, b),
                    None => (OperatorPosition::RHS, NumericGateProperties::RHS.default_value()),
                }
            });

        let expression = lhs.merge(&rhs).fold(
            Expression::new(NumericGateProperties::LHS.default_value(), NumericGateProperties::RHS.default_value()),
            |old_state, (o, value)| match *o {
                OperatorPosition::LHS => old_state.lhs(*value),
                OperatorPosition::RHS => old_state.rhs(*value),
            },
        );

        // The internal result
        let internal_result = expression.map(move |e| f(e.lhs, e.rhs));

        // TODO: handle result based on outbound property id and inbound property id
        let handle_id = e.properties.get(NumericGateProperties::RESULT.as_ref()).unwrap().id.as_u128();

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
                e.set(NumericGateProperties::RESULT.to_string(), json!(*v));
            }, handle_id);

        numeric_gate
    }

    /// TODO: unit test
    pub fn type_name(&self) -> String {
        self.entity.type_name.clone()
    }
}

impl Disconnectable for NumericGate<'_> {
    /// TODO: Add guard: disconnect only if actually connected
    fn disconnect(&self) {
        debug!("Disconnect numeric gate {} {}", self.type_name(), self.handle_id);
        self.internal_result.read().unwrap().remove(self.handle_id);
    }
}

impl Operation for NumericGate<'_> {
    fn lhs(&self, value: Value) {
        self.entity.set(NumericGateProperties::LHS.as_ref(), value);
    }

    fn result(&self) -> Value {
        self.entity.get(NumericGateProperties::RESULT.as_ref()).unwrap().clone()
    }
}

impl Gate for NumericGate<'_> {
    fn rhs(&self, value: Value) {
        self.entity.set(NumericGateProperties::RHS.as_ref(), value);
    }
}

/// Automatically disconnect streams on destruction
impl Drop for NumericGate<'_> {
    fn drop(&mut self) {
        debug!("Drop numeric gate");
        self.disconnect();
    }
}
