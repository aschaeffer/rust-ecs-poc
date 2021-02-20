use crate::api::{PropertyInstanceGetter, PropertyInstanceSetter};
use crate::behaviour::{EntityBehaviour, XorGate, DefaultConnector, ConnectorBehaviour};
use crate::reactive::{Connector, LogicalGateProperties};
use crate::tests::create_default_connector;
use std::sync::Arc;
use serde_json::json;

#[test]
fn xor_gates_test () {
    // Now it's very convenient to create AND-Gates
    let xor_gate_1 = XorGate::new().unwrap();
    let xor_gate_2 = XorGate::new().unwrap();
    let xor_gate_3 = XorGate::new().unwrap();

    // You can get the inner ReactiveEntityInstance from a LogicalGate
    let xor_1 = xor_gate_1.entity.clone();
    let xor_2 = xor_gate_2.entity.clone();
    let xor_3 = xor_gate_3.entity.clone();
    // In real world, the or gate have to be registered in the registry (!)

    // Reset all states
    xor_1.set(LogicalGateProperties::LHS.to_string(), json!(false));
    xor_1.set(LogicalGateProperties::RHS.to_string(), json!(false));
    xor_1.set(LogicalGateProperties::RESULT.to_string(), json!(false));

    xor_2.set(LogicalGateProperties::LHS.to_string(), json!(false));
    xor_2.set(LogicalGateProperties::RHS.to_string(), json!(false));
    xor_2.set(LogicalGateProperties::RESULT.to_string(), json!(false));

    xor_3.set(LogicalGateProperties::LHS.to_string(), json!(false));
    xor_3.set(LogicalGateProperties::RHS.to_string(), json!(false));
    xor_3.set(LogicalGateProperties::RESULT.to_string(), json!(false));


    // Connect the results of the first two AND-Gates with the inputs of the third AND-Gate
    let r_xor_1_xor_3 = Arc::new(create_default_connector(
        xor_1.clone(),
        xor_3.clone(),
        LogicalGateProperties::RESULT.to_string(),
        LogicalGateProperties::LHS.to_string()
    ));
    let c_xor_1_xor_3 = Connector::from_relation(r_xor_1_xor_3.clone(), DefaultConnector::OPERATION);
    assert_ne!(0, c_xor_1_xor_3.handle_id);

    let r_xor_2_xor_3 = Arc::new(create_default_connector(
        xor_2.clone(),
        xor_3.clone(),
        LogicalGateProperties::RESULT.to_string(),
        LogicalGateProperties::RHS.to_string()
    ));
    let c_xor_2_xor_3 = Connector::from_relation(r_xor_2_xor_3.clone(), DefaultConnector::OPERATION);
    assert_ne!(0, c_xor_2_xor_3.handle_id);

    // Initial
    assert_eq!(false, xor_1.as_bool(LogicalGateProperties::RESULT.to_string()).unwrap());
    assert_eq!(false, xor_2.as_bool(LogicalGateProperties::RESULT.to_string()).unwrap());
    assert_eq!(false, xor_3.as_bool(LogicalGateProperties::RESULT.to_string()).unwrap());

    // Bit 1 true
    xor_1.set(LogicalGateProperties::LHS.to_string(), json!(true));
    assert_eq!(true, xor_1.as_bool(LogicalGateProperties::RESULT.to_string()).unwrap());
    assert_eq!(false, xor_2.as_bool(LogicalGateProperties::RESULT.to_string()).unwrap());
    assert_eq!(true, xor_3.as_bool(LogicalGateProperties::RESULT.to_string()).unwrap());

    // Bit 2 true
    xor_1.set(LogicalGateProperties::LHS.to_string(), json!(false));
    xor_1.set(LogicalGateProperties::RHS.to_string(), json!(true));
    assert_eq!(true, xor_1.as_bool(LogicalGateProperties::RESULT.to_string()).unwrap());
    assert_eq!(false, xor_2.as_bool(LogicalGateProperties::RESULT.to_string()).unwrap());
    assert_eq!(true, xor_3.as_bool(LogicalGateProperties::RESULT.to_string()).unwrap());

    // Bit 1 true, Bit 2 true
    xor_1.set(LogicalGateProperties::LHS.to_string(), json!(true));
    assert_eq!(false, xor_1.as_bool(LogicalGateProperties::RESULT.to_string()).unwrap());
    assert_eq!(false, xor_2.as_bool(LogicalGateProperties::RESULT.to_string()).unwrap());
    assert_eq!(false, xor_3.as_bool(LogicalGateProperties::RESULT.to_string()).unwrap());

    // Bit 3 true
    xor_1.set(LogicalGateProperties::LHS.to_string(), json!(false));
    xor_1.set(LogicalGateProperties::RHS.to_string(), json!(false));
    xor_2.set(LogicalGateProperties::LHS.to_string(), json!(true));
    assert_eq!(false, xor_1.as_bool(LogicalGateProperties::RESULT.to_string()).unwrap());
    assert_eq!(true, xor_2.as_bool(LogicalGateProperties::RESULT.to_string()).unwrap());
    assert_eq!(true, xor_3.as_bool(LogicalGateProperties::RESULT.to_string()).unwrap());

    // Bit 4 true
    xor_2.set(LogicalGateProperties::LHS.to_string(), json!(false));
    xor_2.set(LogicalGateProperties::RHS.to_string(), json!(true));
    assert_eq!(false, xor_1.as_bool(LogicalGateProperties::RESULT.to_string()).unwrap());
    assert_eq!(true, xor_2.as_bool(LogicalGateProperties::RESULT.to_string()).unwrap());
    assert_eq!(true, xor_3.as_bool(LogicalGateProperties::RESULT.to_string()).unwrap());

    // Bit 3 true, Bit 4 true
    xor_2.set(LogicalGateProperties::LHS.to_string(), json!(true));
    assert_eq!(false, xor_1.as_bool(LogicalGateProperties::RESULT.to_string()).unwrap());
    assert_eq!(false, xor_2.as_bool(LogicalGateProperties::RESULT.to_string()).unwrap());
    assert_eq!(false, xor_3.as_bool(LogicalGateProperties::RESULT.to_string()).unwrap());

    // All true
    xor_1.set(LogicalGateProperties::LHS.to_string(), json!(true));
    xor_1.set(LogicalGateProperties::RHS.to_string(), json!(true));
    assert_eq!(false, xor_1.as_bool(LogicalGateProperties::RESULT.to_string()).unwrap());
    assert_eq!(false, xor_2.as_bool(LogicalGateProperties::RESULT.to_string()).unwrap());
    assert_eq!(false, xor_3.as_bool(LogicalGateProperties::RESULT.to_string()).unwrap());

    // Bit 4 false
    xor_2.set(LogicalGateProperties::RHS.to_string(), json!(false));
    assert_eq!(false, xor_1.as_bool(LogicalGateProperties::RESULT.to_string()).unwrap());
    assert_eq!(true, xor_2.as_bool(LogicalGateProperties::RESULT.to_string()).unwrap());
    assert_eq!(true, xor_3.as_bool(LogicalGateProperties::RESULT.to_string()).unwrap());

    // Bit 2 false
    xor_1.set(LogicalGateProperties::RHS.to_string(), json!(false));
    assert_eq!(true, xor_1.as_bool(LogicalGateProperties::RESULT.to_string()).unwrap());
    assert_eq!(true, xor_2.as_bool(LogicalGateProperties::RESULT.to_string()).unwrap());
    assert_eq!(false, xor_3.as_bool(LogicalGateProperties::RESULT.to_string()).unwrap());
}
