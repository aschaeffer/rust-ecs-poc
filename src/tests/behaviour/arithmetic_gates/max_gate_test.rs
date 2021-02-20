use crate::api::{PropertyInstanceGetter, PropertyInstanceSetter};
use crate::behaviour::{MaxGate, EntityBehaviour};
use serde_json::json;
use crate::reactive::arithmetic_gate::ArithmeticGateProperties;

const LHS: ArithmeticGateProperties = ArithmeticGateProperties::LHS;
const RHS: ArithmeticGateProperties = ArithmeticGateProperties::RHS;
const RESULT: ArithmeticGateProperties = ArithmeticGateProperties::RESULT;

#[test]
fn max_gate_test () {
    let max_gate = MaxGate::new().unwrap();
    let max = max_gate.entity.clone();
    max.set(RESULT.to_string(), json!(0));
    max.set(LHS.to_string(), json!(0));
    max.set(RHS.to_string(), json!(0));
    assert_eq!(0, max.as_i64(RESULT.to_string()).unwrap());
    max.set(LHS.to_string(), json!(5));
    assert_eq!(5, max.as_i64(RESULT.to_string()).unwrap());
    max.set(RHS.to_string(), json!(15));
    assert_eq!(15, max.as_i64(RESULT.to_string()).unwrap());
    max.set(LHS.to_string(), json!(10));
    assert_eq!(15, max.as_i64(RESULT.to_string()).unwrap());
}
