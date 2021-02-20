use crate::api::{PropertyInstanceGetter, PropertyInstanceSetter};
use crate::behaviour::{DivGate, EntityBehaviour};
use serde_json::json;
use crate::reactive::arithmetic_gate::ArithmeticGateProperties;

const LHS: ArithmeticGateProperties = ArithmeticGateProperties::LHS;
const RHS: ArithmeticGateProperties = ArithmeticGateProperties::RHS;
const RESULT: ArithmeticGateProperties = ArithmeticGateProperties::RESULT;

#[test]
fn div_gate_test () {
    let div_gate = DivGate::new().unwrap();

    // You can get the inner ReactiveEntityInstance from a LogicalGate
    let div = div_gate.entity.clone();

    div.set(LHS.to_string(), json!(10));
    div.set(RHS.to_string(), json!(5));
    assert_eq!(2, div.as_i64(RESULT.to_string()).unwrap());
}
