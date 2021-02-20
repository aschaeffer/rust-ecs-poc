use crate::api::{PropertyInstanceGetter, PropertyInstanceSetter};
use crate::behaviour::{SubGate, EntityBehaviour, DefaultConnector, ConnectorBehaviour};
use crate::reactive::{Connector};
use crate::tests::create_default_connector;
use std::sync::Arc;
use serde_json::json;
use crate::reactive::arithmetic_gate::ArithmeticGateProperties;

const LHS: ArithmeticGateProperties = ArithmeticGateProperties::LHS;
const RHS: ArithmeticGateProperties = ArithmeticGateProperties::RHS;
const RESULT: ArithmeticGateProperties = ArithmeticGateProperties::RESULT;

#[test]
fn sub_gates_test () {
    let sub_gate_1 = SubGate::new().unwrap();
    let sub_gate_2 = SubGate::new().unwrap();
    let sub_gate_3 = SubGate::new().unwrap();

    // You can get the inner ReactiveEntityInstance from a LogicalGate
    let sub_1 = sub_gate_1.entity.clone();
    let sub_2 = sub_gate_2.entity.clone();
    let sub_3 = sub_gate_3.entity.clone();
    // In real world, the and gate have to be registered in the registry (!)

    // Reset all states
    sub_1.set(LHS.to_string(), json!(0));
    sub_1.set(RHS.to_string(), json!(0));
    sub_1.set(RESULT.to_string(), json!(0));

    sub_2.set(LHS.to_string(), json!(0));
    sub_2.set(RHS.to_string(), json!(0));
    sub_2.set(RESULT.to_string(), json!(0));

    sub_3.set(LHS.to_string(), json!(0));
    sub_3.set(RHS.to_string(), json!(0));
    sub_3.set(RESULT.to_string(), json!(0));

    // Connect the results of the first two AND-Gates with the inputs of the third AND-Gate
    let r_sub_1_sub_3 = Arc::new(create_default_connector(
        sub_1.clone(),
        sub_3.clone(),
        RESULT.to_string(),
        LHS.to_string()
    ));
    let c_sub_1_sub_3 = Connector::from_relation(r_sub_1_sub_3.clone(), DefaultConnector::OPERATION);
    assert_ne!(0, c_sub_1_sub_3.handle_id);

    let r_sub_2_sub_3 = Arc::new(create_default_connector(
        sub_2.clone(),
        sub_3.clone(),
        RESULT.to_string(),
        RHS.to_string()
    ));
    let c_sub_2_sub_3 = Connector::from_relation(r_sub_2_sub_3.clone(), DefaultConnector::OPERATION);
    assert_ne!(0, c_sub_2_sub_3.handle_id);

    // Starting point
    assert_eq!(0, sub_1.as_i64(RESULT.to_string()).unwrap());
    assert_eq!(0, sub_2.as_i64(RESULT.to_string()).unwrap());
    assert_eq!(0, sub_3.as_i64(RESULT.to_string()).unwrap());

    sub_1.set(LHS.to_string(), json!(10));
    assert_eq!(10, sub_1.as_i64(RESULT.to_string()).unwrap());
    assert_eq!(0, sub_2.as_i64(RESULT.to_string()).unwrap());
    assert_eq!(10, sub_3.as_i64(RESULT.to_string()).unwrap());

    sub_1.set(RHS.to_string(), json!(5));
    assert_eq!(5, sub_1.as_i64(RESULT.to_string()).unwrap());
    assert_eq!(0, sub_2.as_i64(RESULT.to_string()).unwrap());
    assert_eq!(5, sub_3.as_i64(RESULT.to_string()).unwrap());

    sub_2.set(LHS.to_string(), json!(3));
    assert_eq!(5, sub_1.as_i64(RESULT.to_string()).unwrap());
    assert_eq!(3, sub_2.as_i64(RESULT.to_string()).unwrap());
    assert_eq!(2, sub_3.as_i64(RESULT.to_string()).unwrap());

    sub_2.set(RHS.to_string(), json!(22));
    assert_eq!(5, sub_1.as_i64(RESULT.to_string()).unwrap());
    assert_eq!(-19, sub_2.as_i64(RESULT.to_string()).unwrap());
    assert_eq!(24, sub_3.as_i64(RESULT.to_string()).unwrap());
}
