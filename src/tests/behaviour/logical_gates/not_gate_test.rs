use crate::api::{PropertyInstanceGetter, PropertyInstanceSetter};
use crate::behaviour::{EntityBehaviour, NotGate, DefaultConnector, ConnectorBehaviour};
use crate::reactive::{Connector, LogicalGateProperties};
use crate::tests::create_default_connector;
use std::sync::Arc;
use serde_json::json;

#[test]
fn not_gates_test () {
    // Now it's very convenient to create AND-Gates
    let not_gate_1 = NotGate::new().unwrap();
    let not_gate_2 = NotGate::new().unwrap();

    // You can get the inner ReactiveEntityInstance from a LogicalGate
    let not_1 = not_gate_1.entity.clone();
    let not_2 = not_gate_2.entity.clone();

    // Reset all states
    not_1.set(LogicalGateProperties::LHS.to_string(), json!(false));
    not_1.set(LogicalGateProperties::RESULT.to_string(), json!(true));

    not_2.set(LogicalGateProperties::LHS.to_string(), json!(true));
    not_2.set(LogicalGateProperties::RESULT.to_string(), json!(false));


    // Connect the results of the first two AND-Gates with the inputs of the third AND-Gate
    let r_not_1_not_2 = Arc::new(create_default_connector(
        not_1.clone(),
        not_2.clone(),
        LogicalGateProperties::RESULT.to_string(),
        LogicalGateProperties::LHS.to_string()
    ));
    let c_not_1_not_2 = Connector::from_relation(r_not_1_not_2.clone(), DefaultConnector::OPERATION);
    assert_ne!(0, c_not_1_not_2.handle_id);

    // Initial
    assert_eq!(true, not_1.as_bool(LogicalGateProperties::RESULT.to_string()).unwrap());
    assert_eq!(false, not_2.as_bool(LogicalGateProperties::RESULT.to_string()).unwrap());

    not_1.set(LogicalGateProperties::LHS.to_string(), json!(true));
    assert_eq!(false, not_1.as_bool(LogicalGateProperties::RESULT.to_string()).unwrap());
    assert_eq!(true, not_2.as_bool(LogicalGateProperties::RESULT.to_string()).unwrap());

    not_1.set(LogicalGateProperties::LHS.to_string(), json!(false));
    assert_eq!(true, not_1.as_bool(LogicalGateProperties::RESULT.to_string()).unwrap());
    assert_eq!(false, not_2.as_bool(LogicalGateProperties::RESULT.to_string()).unwrap());
}
