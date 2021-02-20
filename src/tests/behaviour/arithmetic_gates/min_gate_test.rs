use crate::api::{PropertyInstanceGetter, PropertyInstanceSetter};
use crate::behaviour::{MinGate, EntityBehaviour};
use serde_json::json;
use crate::reactive::arithmetic_gate::ArithmeticGateProperties;

const LHS: ArithmeticGateProperties = ArithmeticGateProperties::LHS;
const RHS: ArithmeticGateProperties = ArithmeticGateProperties::RHS;
const RESULT: ArithmeticGateProperties = ArithmeticGateProperties::RESULT;

#[test]
fn min_gate_test () {
    let min_gate = MinGate::new().unwrap();
    let min = min_gate.entity.clone();
    min.set(RESULT.to_string(), json!(0));
    min.set(LHS.to_string(), json!(0));
    min.set(RHS.to_string(), json!(0));
    assert_eq!(0, min.as_i64(RESULT.to_string()).unwrap());
    min.set(LHS.to_string(), json!(5));
    assert_eq!(0, min.as_i64(RESULT.to_string()).unwrap());
    min.set(RHS.to_string(), json!(15));
    assert_eq!(5, min.as_i64(RESULT.to_string()).unwrap());
    min.set(LHS.to_string(), json!(25));
    assert_eq!(15, min.as_i64(RESULT.to_string()).unwrap());
}
