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
use crate::reactive::entity::arithmetic_gate::ArithmeticGateProperties;
use crate::reactive::Disconnectable;

pub type ArithmeticExpressionValue = ExpressionValue<i64>;

// TODO: Floating Point / Numeric Date / Different types for input and output (e.g. abs)
pub type ArithmeticGateFunction<T> = fn(T, T) -> T;

/// Generic implementation of arithmetic_gates operations with two inputs (LHS,RHS) and one result.
///
/// The implementation is realized using reactive streams.
pub struct ArithmeticGate<'a> {
    pub lhs: RwLock<Stream<'a, ArithmeticExpressionValue>>,

    pub rhs: RwLock<Stream<'a, ArithmeticExpressionValue>>,

    pub f: ArithmeticGateFunction<i64>,

    pub internal_result: RwLock<Stream<'a, i64>>,

    pub entity: Arc<ReactiveEntityInstance>,

    pub handle_id: u128,
}

impl ArithmeticGate<'_> {
    pub fn new<'a>(e: Arc<ReactiveEntityInstance>, f: ArithmeticGateFunction<i64>) -> ArithmeticGate<'static> {
        let lhs = e.properties.get(ArithmeticGateProperties::LHS.as_ref()).unwrap()
            .stream.read().unwrap()
            .map(|v| match v.as_i64() {
                Some(b) => (OperatorPosition::LHS, b),
                None => (OperatorPosition::LHS, ArithmeticGateProperties::LHS.default_value()),
            });
        let rhs = e.properties.get(ArithmeticGateProperties::RHS.as_ref()).unwrap()
            .stream.read().unwrap()
            .map(|v| -> ArithmeticExpressionValue {
                match v.as_i64() {
                    Some(b) => (OperatorPosition::RHS, b),
                    None => (OperatorPosition::RHS, ArithmeticGateProperties::RHS.default_value()),
                }
            });

        let expression = lhs.merge(&rhs).fold(
            Expression::new(ArithmeticGateProperties::LHS.default_value(), ArithmeticGateProperties::RHS.default_value()),
            |old_state, (o, value)| match *o {
                OperatorPosition::LHS => old_state.lhs(*value),
                OperatorPosition::RHS => old_state.rhs(*value),
            },
        );

        // The internal result
        let internal_result = expression.map(move |e| f(e.lhs, e.rhs));

        // TODO: handle result based on outbound property id and inbound property id
        let handle_id = e.properties.get(ArithmeticGateProperties::RESULT.as_ref()).unwrap().id.as_u128();

        let arithmetic_gate = ArithmeticGate {
            lhs: RwLock::new(lhs),
            rhs: RwLock::new(rhs),
            f,
            internal_result: RwLock::new(internal_result),
            entity: e.clone(),
            handle_id
        };

        // Connect the internal result with the stream of the result property
        arithmetic_gate.internal_result.read().unwrap()
            .observe_with_handle(move |v| {
                debug!("Setting result of arithmetic gate: {}", v);
                e.set(ArithmeticGateProperties::RESULT.as_ref(), json!(*v));
            }, handle_id);

        arithmetic_gate
    }

    /// TODO: unit test
    pub fn type_name(&self) -> String {
        self.entity.type_name.clone()
    }
}

impl Disconnectable for ArithmeticGate<'_> {
    /// TODO: Add guard: disconnect only if actually connected
    fn disconnect(&self) {
        debug!("Disconnect arithmetic gate {}", self.handle_id);
        self.internal_result.read().unwrap().remove(self.handle_id);
    }
}

impl Operation for ArithmeticGate<'_> {
    fn lhs(&self, value: Value) {
        self.entity.set(ArithmeticGateProperties::LHS.as_ref(), value);
    }

    fn result(&self) -> Value {
        self.entity.get(ArithmeticGateProperties::RESULT.as_ref()).unwrap().clone()
    }
}

impl Gate for ArithmeticGate<'_> {
    fn rhs(&self, value: Value) {
        self.entity.set(ArithmeticGateProperties::RHS.as_ref(), value);
    }
}

/// Automatically disconnect streams on destruction
impl Drop for ArithmeticGate<'_> {
    fn drop(&mut self) {
        debug!("Drop arithmetic gate");
        self.disconnect();
    }
}
