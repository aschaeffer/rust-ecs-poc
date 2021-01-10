use bidule::Stream;
// use serde_json::{Value,json};
use crate::model::ReactiveEntityInstance;
use std::sync::RwLock;
use crate::reactive::{OperatorPosition, Expression};
use indradb::VertexProperties;

// Generic implementation of an expression(LHS,RHS) using reactive streams
pub struct LogicalGate<'a> {

    pub lhs: RwLock<Stream<'a, (OperatorPosition, bool)>>,

    pub rhs: RwLock<Stream<'a, (OperatorPosition, bool)>>,

}

impl LogicalGate<'_> {

    fn new(e: ReactiveEntityInstance, f: F) -> LogicalGate<'static> {
        // let bit_1 = e.properties.get("bit_1").unwrap();
        // let bit_2 = e.properties.get("bit_2").unwrap();
        // let bit_3 = e.properties.get("bit_3").unwrap();
        // result_1.get();

        let lhs = e.properties
            .get("bit_1")
            .unwrap()
            .stream
            .read()
            .unwrap()
            .map(|v| match v.as_bool() {
                Some(b) => (OperatorPosition::LHS, b),
                None => (OperatorPosition::LHS, false)
            });
        let rhs = e.properties
            .get("bit_2")
            .unwrap()
            .stream
            .read()
            .unwrap()
            .map(|v| -> (OperatorPosition, bool) {
                match v.as_bool() {
                    Some(b) => (OperatorPosition::RHS, b),
                    None => (OperatorPosition::RHS, false)
                }
            });

        // let expression_b = lhs.merge(&rhs);
        // expression_b.observe(| (o, v) | println!("[{:?} {}] => expression", o, v));

        let expression = lhs.merge(&rhs)
            .fold(
                Expression::new(false, false),
                | old_state, (o, value) | {
                    // println!("expression [{:?} {}]", o, value);
                    match *o {
                        OperatorPosition::LHS => {
                            old_state.lhs(*value)
                            // Expression::new(*value, old_state.rhs)
                            // Expression { lhs: *value, rhs: old_state.rhs }
                        },
                        OperatorPosition::RHS => {
                            old_state.rhs(*value)
                            // Expression::new(old_state.lhs, *value)
                            // Expression { lhs: old_state.lhs, rhs: *value }
                        }
                    }
                }
            );
        let result = expression.map(f);
        result.observe(| v | {
            result.send(v);
        });

        let result_1 = e.properties.get("result_1").unwrap();

        // let result_1 = expression.map(move | e | e.lhs && e.rhs);
        LogicalGate {
            lhs: RwLock::new(lhs),
            rhs: RwLock::new(rhs),
        }

    }

}

struct AndGate {}
impl AndGate<'_> {

    pub fn from_vertex_properties(v: VertexProperties) -> LogicalGate<'static> {
        AndGate::from_entity_instance(ReactiveEntityInstance::from(v))
    }

    pub fn from_entity_instance(e: ReactiveEntityInstance) -> LogicalGate<'static> {
        LogicalGate::new(e, |e| e.lhs && e.rhs)
    }

}
