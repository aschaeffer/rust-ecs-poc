use std::sync::Arc;

use serde_json::json;

use crate::api::{PropertyInstanceGetter, PropertyInstanceSetter};
use crate::behaviour::{AndGate, ConnectorBehaviour, DefaultConnector, EntityBehaviour, LogicalGateBehaviour, OrGate};
use crate::model::ReactiveEntityInstanceFactory;
use crate::reactive::Connector;
use crate::reactive::entity::logical_gate::{LogicalGate, LogicalGateProperties};
use crate::reactive::entity::logical_gate::LogicalGateReactiveEntityInstanceFactory as Factory;
use crate::tests::create_default_connector;

const LHS: LogicalGateProperties = LogicalGateProperties::LHS;
const RHS: LogicalGateProperties = LogicalGateProperties::RHS;
const RESULT: LogicalGateProperties = LogicalGateProperties::RESULT;

#[test]
fn and_gate_type_test () {
    let and = Factory::new(AndGate::TYPE_NAME);
    // let and = Arc::new(create_logical_gate_entity(AndGate::TYPE_NAME.to_string()));
    let and_gate = LogicalGate::new(and.clone(), AndGate::OPERATION);
    assert_eq!(AndGate::TYPE_NAME.to_string(), and_gate.type_name());
}

#[test]
fn and_gate_test () {
    let and = Factory::new(AndGate::TYPE_NAME);
    {
        // Create the AND-Gate in scope
        let and_gate = LogicalGate::new(and.clone(), AndGate::OPERATION);
        assert_ne!(0, and_gate.handle_id);

        and.set(LHS.to_string(), json!(false));
        and.set(RHS.to_string(), json!(false));
        assert_eq!(false, and.as_bool(RESULT.to_string()).unwrap());
        and.set(RHS.to_string(), json!(true));
        assert_eq!(false, and.as_bool(RESULT.to_string()).unwrap());
        and.set(RHS.to_string(), json!(false));
        and.set(LHS.to_string(), json!(true));
        assert_eq!(false, and.as_bool(RESULT.to_string()).unwrap());
        and.set(RHS.to_string(), json!(true));
        assert_eq!(true, and.as_bool(RESULT.to_string()).unwrap());
    } // The LogicalGate doesn't live no more
    // Setting the inputs ...
    and.set(LHS.to_string(), json!(false));
    and.set(RHS.to_string(), json!(false));
    // ... doesn't affect the result anymore (result should have the same value as before)
    assert_eq!(true, and.as_bool(RESULT.to_string()).unwrap());
}

#[test]
fn or_gate_test () {
    let or = Factory::new(OrGate::TYPE_NAME);
    {
        let and_gate = LogicalGate::new(or.clone(), OrGate::OPERATION);
        assert_ne!(0, and_gate.handle_id);

        or.set(LHS.to_string(), json!(false));
        or.set(RHS.to_string(), json!(false));
        assert_eq!(false, or.as_bool(RESULT.to_string()).unwrap());

        or.set(LHS.to_string(), json!(true));
        assert_eq!(true, or.as_bool(RESULT.to_string()).unwrap());
        or.set(LHS.to_string(), json!(false));
        assert_eq!(false, or.as_bool(RESULT.to_string()).unwrap());

        or.set(RHS.to_string(), json!(true));
        assert_eq!(true, or.as_bool(RESULT.to_string()).unwrap());
        or.set(RHS.to_string(), json!(false));
        assert_eq!(false, or.as_bool(RESULT.to_string()).unwrap());

        or.set(LHS.to_string(), json!(true));
        or.set(RHS.to_string(), json!(true));
        assert_eq!(true, or.as_bool(RESULT.to_string()).unwrap());
    } // The LogicalGate doesn't live no more
    // Setting the inputs ...
    or.set(LHS.to_string(), json!(false));
    or.set(RHS.to_string(), json!(false));
    // ... doesn't affect the result anymore (result should have the same value as before)
    assert_eq!(true, or.as_bool(RESULT.to_string()).unwrap());
}

/// The results of two AND-Gates are the inputs of the third AND-Gate
#[test]
fn three_and_gates_test () {
    let and_1 = Factory::new(AndGate::TYPE_NAME);
    let and_2 = Factory::new(AndGate::TYPE_NAME);
    let and_3 = Factory::new(AndGate::TYPE_NAME);

    // Reset all states
    and_1.set(LHS.to_string(), json!(false));
    and_1.set(RHS.to_string(), json!(false));
    and_1.set(RESULT.to_string(), json!(false));

    and_2.set(LHS.to_string(), json!(false));
    and_2.set(RHS.to_string(), json!(false));
    and_2.set(RESULT.to_string(), json!(false));

    and_3.set(LHS.to_string(), json!(false));
    and_3.set(RHS.to_string(), json!(false));
    and_3.set(RESULT.to_string(), json!(false));

    // Make the entity instances act as AND-Gates
    let and_gate_1 = LogicalGate::new(and_1.clone(), AndGate::OPERATION);
    assert_ne!(0, and_gate_1.handle_id);

    let and_gate_2 = LogicalGate::new(and_2.clone(), AndGate::OPERATION);
    assert_ne!(0, and_gate_2.handle_id);

    let and_gate_3 = LogicalGate::new(and_3.clone(), AndGate::OPERATION);
    assert_ne!(0, and_gate_3.handle_id);

    // Connect the results of the first two AND-Gates with the inputs of the third AND-Gate
    let r_and_1_and_3 = Arc::new(create_default_connector(
        and_1.clone(),
        and_3.clone(),
        RESULT.to_string(),
        LHS.to_string()
    ));
    let c_and_1_and_3 = Connector::from_relation(r_and_1_and_3.clone(), DefaultConnector::OPERATION);
    assert_ne!(0, c_and_1_and_3.handle_id);

    let r_and_2_and_3 = Arc::new(create_default_connector(
        and_2.clone(),
        and_3.clone(),
        RESULT.to_string(),
        RHS.to_string()
    ));
    let c_and_2_and_3 = Connector::from_relation(r_and_2_and_3.clone(), DefaultConnector::OPERATION);
    assert_ne!(0, c_and_2_and_3.handle_id);

    and_1.set(LHS.to_string(), json!(true));
    assert_eq!(false, and_1.as_bool(RESULT.to_string()).unwrap());
    assert_eq!(false, and_3.as_bool(RESULT.to_string()).unwrap());
    and_1.set(RHS.to_string(), json!(true));
    assert_eq!(true, and_1.as_bool(RESULT.to_string()).unwrap());
    assert_eq!(false, and_3.as_bool(RESULT.to_string()).unwrap());

    and_2.set(LHS.to_string(), json!(true));
    assert_eq!(false, and_2.as_bool(RESULT.to_string()).unwrap());
    assert_eq!(false, and_3.as_bool(RESULT.to_string()).unwrap());
    and_2.set(RHS.to_string(), json!(true));
    assert_eq!(true, and_2.as_bool(RESULT.to_string()).unwrap());
    assert_eq!(true, and_3.as_bool(RESULT.to_string()).unwrap());
}
