use crate::api::{PropertyInstanceGetter, PropertyInstanceSetter};
use crate::behaviour::{MulGate, EntityBehaviour};
use serde_json::json;
use crate::reactive::arithmetic_gate::ArithmeticGateProperties;

const LHS: ArithmeticGateProperties = ArithmeticGateProperties::LHS;
const RHS: ArithmeticGateProperties = ArithmeticGateProperties::RHS;
const RESULT: ArithmeticGateProperties = ArithmeticGateProperties::RESULT;

#[test]
fn mul_gate_test () {
    let mul_gate = MulGate::new().unwrap();

    // You can get the inner ReactiveEntityInstance from a LogicalGate
    let mul = mul_gate.entity.clone();

    mul.set(RESULT.to_string(), json!(0));
    mul.set(LHS.to_string(), json!(0));
    mul.set(RHS.to_string(), json!(0));
    assert_eq!(0, mul.as_i64(RESULT.to_string()).unwrap());
    mul.set(LHS.to_string(), json!(5));
    mul.set(RHS.to_string(), json!(5));
    assert_eq!(25, mul.as_i64(RESULT.to_string()).unwrap());
}
