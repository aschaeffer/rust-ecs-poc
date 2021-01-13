use crate::model::{ReactiveEntityInstance, ReactivePropertyInstance};
use crate::reactive::{Expression, OperatorPosition};
use bidule::Stream;
use indradb::VertexProperties;
use serde_json::{json, Value};
use std::sync::RwLock;

type BinaryOperation = fn(bool, bool) -> bool;

// Generic implementation of an expression(LHS,RHS) using reactive streams
pub struct LogicalGate<'a> {
    pub lhs: RwLock<Stream<'a, (OperatorPosition, bool)>>,

    pub rhs: RwLock<Stream<'a, (OperatorPosition, bool)>>,

    pub f: BinaryOperation,

    pub internal_result: RwLock<Stream<'a, bool>>,
}

impl LogicalGate<'_> {
    fn new<'a>(e: ReactiveEntityInstance<'static>, f: BinaryOperation) -> LogicalGate<'static> {
        let lhs = e
            .properties
            .get("bit_1")
            .unwrap()
            .stream
            .read()
            .unwrap()
            .map(|v| match v.as_bool() {
                Some(b) => (OperatorPosition::LHS, b),
                None => (OperatorPosition::LHS, false),
            });
        let rhs = e
            .properties
            .get("bit_2")
            .unwrap()
            .stream
            .read()
            .unwrap()
            .map(|v| -> (OperatorPosition, bool) {
                match v.as_bool() {
                    Some(b) => (OperatorPosition::RHS, b),
                    None => (OperatorPosition::RHS, false),
                }
            });

        let expression = lhs.merge(&rhs).fold(
            Expression::new(false, false),
            |old_state, (o, value)| match *o {
                OperatorPosition::LHS => old_state.lhs(*value),
                OperatorPosition::RHS => old_state.rhs(*value),
            },
        );

        // The internal result
        let internal_result = expression.map(move |e| f(e.lhs, e.rhs));

        let logical_gate = LogicalGate {
            lhs: RwLock::new(lhs),
            rhs: RwLock::new(rhs),
            f,
            internal_result: RwLock::new(internal_result),
        };

        // Connect the internal result with the stream of the result property
        logical_gate
            .internal_result
            .read()
            .unwrap()
            .observe(move |v| {
                e.properties
                    .get("result_1")
                    .unwrap()
                    .stream
                    .read()
                    .unwrap()
                    .send(&json!(*v));
                // result_1.send(&json!(*v));
            });

        logical_gate
    }
}

struct AndGate {}
impl AndGate {
    pub fn from_vertex_properties(v: VertexProperties) -> LogicalGate<'static> {
        AndGate::from_entity_instance(ReactiveEntityInstance::from(v))
    }

    pub fn from_entity_instance(e: ReactiveEntityInstance<'static>) -> LogicalGate<'static> {
        let mut and: BinaryOperation = |lhs, rhs| lhs && rhs;
        LogicalGate::new(e, and)
    }
}
