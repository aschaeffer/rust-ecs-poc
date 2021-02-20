use crate::api::{PropertyInstanceGetter, PropertyInstanceSetter};
use crate::behaviour::{ModGate, EntityBehaviour};
use serde_json::json;
use crate::reactive::arithmetic_gate::ArithmeticGateProperties;

const LHS: ArithmeticGateProperties = ArithmeticGateProperties::LHS;
const RHS: ArithmeticGateProperties = ArithmeticGateProperties::RHS;
const RESULT: ArithmeticGateProperties = ArithmeticGateProperties::RESULT;

#[test]
fn modulo_gate_test() {
    let modulo_gate = ModGate::new().unwrap();

    // You can get the inner ReactiveEntityInstance from a LogicalGate
    let modulo = modulo_gate.entity.clone();

    modulo.set(LHS.to_string(), json!(12));
    modulo.set(RHS.to_string(), json!(5));
    assert_eq!(2, modulo.as_i64(RESULT.to_string()).unwrap());

    modulo.set(LHS.to_string(), json!(2));
    modulo.set(RHS.to_string(), json!(2));
    assert_eq!(0, modulo.as_i64(RESULT.to_string()).unwrap());
}
