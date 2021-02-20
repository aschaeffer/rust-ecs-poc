use indradb::Transaction;

use crate::builder::{EntityTypeBuilder, ReactiveEntityInstanceBuilder, EntityInstanceBuilder};
use crate::tests::r_json_string;
use crate::tests::utils::{init_application, r_string};
use uuid::Uuid;
use crate::api::{PropertyInstanceSetter, PropertyInstanceGetter};
use std::sync::Arc;
use std::env;
use crate::reactive::{LogicalGateProperties, NumericOperationProperties};
use serde_json::json;
use crate::behaviour::{AndGate, EntityBehaviour, AddGate, SinGate};
use crate::reactive::arithmetic_gate::ArithmeticGateProperties;
use std::f64::consts::PI;

#[test]
fn test_reactive_entity_instance_manager() {
    let application = init_application();
    let entity_type_manager = application.get_entity_type_manager();
    let entity_instance_manager = application.get_entity_instance_manager();
    let reactive_entity_instance_manager = application.get_reactive_entity_instance_manager();
    let graph_database = application.get_graph_database();
    let transaction = graph_database.get_transaction().unwrap();

    let type_name = r_string();
    let property_name = r_string();
    let property_value = r_json_string();

    // Check that we cannot create an entity instance with a type which doesn't exist
    let result = ReactiveEntityInstanceBuilder::new(type_name.clone())
        .property(property_name.clone(), property_value.clone())
        .create(reactive_entity_instance_manager.clone());
    assert!(result.is_err());
    assert_eq!(0, transaction.get_vertex_count().unwrap());

    EntityTypeBuilder::new(type_name.clone())
        .string_property(property_name.clone())
        .register(entity_type_manager.clone());

    let result = ReactiveEntityInstanceBuilder::new(type_name.clone())
        .property(property_name.clone(), property_value.clone())
        .create(reactive_entity_instance_manager.clone());
    let reactive_entity_instance = result.unwrap();
    let id = reactive_entity_instance.id;

    assert!(reactive_entity_instance_manager.has(id));
    assert!(entity_instance_manager.has(id));

    let reactive_entity_instance_2 = reactive_entity_instance_manager.get(id);
    assert!(reactive_entity_instance_2.is_some());
    assert_eq!(reactive_entity_instance_2.unwrap().id, id);

    let entity_instance = entity_instance_manager.get(id);
    assert!(entity_instance.is_some());
    let entity_instance = entity_instance.unwrap();
    assert_eq!(id, entity_instance.id);
    assert!(entity_instance.properties.contains_key(property_name.as_str()));
    assert_eq!(property_value.clone(), *entity_instance.properties.get(property_name.as_str()).unwrap());

    assert!(!reactive_entity_instance_manager.get(Uuid::new_v4()).is_some());

    assert_eq!(property_value.clone(), reactive_entity_instance.as_string(property_name.clone()).unwrap());
    let property_value_new = r_json_string();
    reactive_entity_instance.set(property_name.clone(), property_value_new.clone());
    assert_eq!(property_value_new.clone(), reactive_entity_instance.as_string(property_name.clone()).unwrap());

    // assert_eq!(property_value.clone(), reactive_entity_instance.as_string(property_name.clone()).unwrap());

    assert_eq!(property_value.clone(), *entity_instance_manager.get(id).unwrap().properties.get(property_name.as_str()).unwrap());
    reactive_entity_instance_manager.commit(id);
    assert_eq!(property_value_new.clone(), entity_instance_manager.get(id).unwrap().as_string(property_name.clone()).unwrap());

    assert_eq!(2, Arc::strong_count(&reactive_entity_instance));
    reactive_entity_instance_manager.delete(id);
    assert_eq!(1, Arc::strong_count(&reactive_entity_instance));
    assert!(!reactive_entity_instance_manager.has(id));
    assert!(!reactive_entity_instance_manager.get(id).is_some());
    assert!(!entity_instance_manager.has(id));
    assert!(!entity_instance_manager.get(id).is_some());

    let id = Uuid::new_v4();
    let result = ReactiveEntityInstanceBuilder::new(type_name.clone())
        .id(id)
        .property(property_name.clone(), property_value.clone())
        .create(reactive_entity_instance_manager.clone());
    let reactive_entity_instance = result.unwrap();
    assert!(reactive_entity_instance_manager.has(id));
    assert!(entity_instance_manager.has(id));
    assert_eq!(id, reactive_entity_instance.id);

    let reactive_entity_instance_2 = reactive_entity_instance_manager.get(id).unwrap();
    assert_eq!(id, reactive_entity_instance_2.id);

    let id = Uuid::new_v4();
    let result = EntityInstanceBuilder::new(type_name.clone())
        .id(id)
        .property(property_name.clone(), property_value.clone())
        .create(entity_instance_manager.clone());
    assert!(result.is_ok());
    assert!(entity_instance_manager.has(id));
    let result = ReactiveEntityInstanceBuilder::new(type_name.clone())
        .id(id)
        .property(property_name.clone(), property_value.clone())
        .create(reactive_entity_instance_manager.clone());
    let reactive_entity_instance = result.unwrap();
    assert!(reactive_entity_instance_manager.has(id));
    assert_eq!(id, reactive_entity_instance.id);
    assert_eq!(property_value.clone(), reactive_entity_instance.as_string(property_name.clone()).unwrap());
    // assert_eq!(property_value.clone(), *entity_instance_manager.get(id).unwrap().properties.get(property_name.as_str()).unwrap());


    // reactive_entity_instance.set(property_name, json!());
    // reactive_entity_instance_manager.commit(id);
}

#[test]
fn test_reactive_entity_instance_manager_import_export() {
    let application = init_application();
    let entity_type_manager = application.get_entity_type_manager();
    let reactive_entity_instance_manager = application.get_reactive_entity_instance_manager();

    let type_name = r_string();

    let mut path = env::temp_dir();
    path.push(format!("{}.json", type_name));
    let path = path.into_os_string().into_string().unwrap();

    let type_name = r_string();
    let property_name = r_string();
    let property_value = r_json_string();
    EntityTypeBuilder::new(type_name.clone())
        .string_property(property_name.clone())
        .register(entity_type_manager.clone());

    let result = ReactiveEntityInstanceBuilder::new(type_name.clone())
        .property(property_name.clone(), property_value.clone())
        .create(reactive_entity_instance_manager.clone());

    let reactive_entity_instance = result.unwrap();
    let uuid = reactive_entity_instance.id;

    reactive_entity_instance_manager.export(uuid, path.clone());
    assert!(reactive_entity_instance_manager.has(uuid));
    reactive_entity_instance_manager.delete(uuid);
    assert!(!reactive_entity_instance_manager.has(uuid));
    let result = reactive_entity_instance_manager.import(path.clone());
    assert!(result.is_ok());
    assert_eq!(uuid, result.unwrap().id);
    assert!(reactive_entity_instance_manager.has(uuid));
}

#[test]
fn test_reactive_entity_instance_manager_behaviour_logical_gate() {
    let application = init_application();
    let entity_type_manager = application.get_entity_type_manager();
    let reactive_entity_instance_manager = application.get_reactive_entity_instance_manager();

    let type_name = String::from(AndGate::TYPE_NAME);

    EntityTypeBuilder::new(type_name.clone())
        .bool_property(LogicalGateProperties::LHS.to_string())
        .bool_property(LogicalGateProperties::RHS.to_string())
        .bool_property(LogicalGateProperties::RESULT.to_string())
        .register(entity_type_manager.clone());

    let result = ReactiveEntityInstanceBuilder::new(type_name.clone())
        .property(LogicalGateProperties::LHS.to_string(), json!(false))
        .property(LogicalGateProperties::RHS.to_string(), json!(false))
        .property(LogicalGateProperties::RESULT.to_string(), json!(false))
        .create(reactive_entity_instance_manager.clone());

    let reactive_entity_instance = result.unwrap();

    assert!(!reactive_entity_instance.as_bool(LogicalGateProperties::LHS.to_string()).unwrap());
    assert!(!reactive_entity_instance.as_bool(LogicalGateProperties::RHS.to_string()).unwrap());
    assert!(!reactive_entity_instance.as_bool(LogicalGateProperties::RESULT.to_string()).unwrap());

    reactive_entity_instance.set(LogicalGateProperties::LHS.to_string(), json!(true));
    assert!(reactive_entity_instance.as_bool(LogicalGateProperties::LHS.to_string()).unwrap());
    assert!(!reactive_entity_instance.as_bool(LogicalGateProperties::RHS.to_string()).unwrap());
    assert!(!reactive_entity_instance.as_bool(LogicalGateProperties::RESULT.to_string()).unwrap());

    // Note: The result should change now because of the behaviour "and"
    reactive_entity_instance.set(LogicalGateProperties::RHS.to_string(), json!(true));
    assert!(reactive_entity_instance.as_bool(LogicalGateProperties::LHS.to_string()).unwrap());
    assert!(reactive_entity_instance.as_bool(LogicalGateProperties::RHS.to_string()).unwrap());
    assert!(reactive_entity_instance.as_bool(LogicalGateProperties::RESULT.to_string()).unwrap());
}

#[test]
fn test_reactive_entity_instance_manager_behaviour_arithmetic_gate() {
    let application = init_application();
    let entity_type_manager = application.get_entity_type_manager();
    let reactive_entity_instance_manager = application.get_reactive_entity_instance_manager();

    let type_name = String::from(AddGate::TYPE_NAME);

    let NUMBER_1: &'static str = ArithmeticGateProperties::LHS.as_ref();
    let NUMBER_2: &'static str = ArithmeticGateProperties::RHS.as_ref();
    let RESULT_1: &'static str = ArithmeticGateProperties::RESULT.as_ref();

    EntityTypeBuilder::new(type_name.clone())
        .number_property(NUMBER_1)
        .number_property(NUMBER_2)
        .number_property(RESULT_1)
        .register(entity_type_manager.clone());

    let result = ReactiveEntityInstanceBuilder::new(type_name.clone())
        .property(NUMBER_1, json!(0))
        .property(NUMBER_2, json!(0))
        .property(RESULT_1, json!(0))
        .create(reactive_entity_instance_manager.clone());

    let reactive_entity_instance = result.unwrap();

    assert_eq!(0, reactive_entity_instance.as_i64(NUMBER_1).unwrap());
    assert_eq!(0, reactive_entity_instance.as_i64(NUMBER_2).unwrap());
    assert_eq!(0, reactive_entity_instance.as_i64(RESULT_1).unwrap());

    // Note: The result should change now because of the behaviour "add"
    reactive_entity_instance.set(NUMBER_1, json!(1));
    assert_eq!(1, reactive_entity_instance.as_i64(NUMBER_1).unwrap());
    assert_eq!(0, reactive_entity_instance.as_i64(NUMBER_2).unwrap());
    assert_eq!(1, reactive_entity_instance.as_i64(RESULT_1).unwrap());

    reactive_entity_instance.set(NUMBER_2, json!(2));
    assert_eq!(1, reactive_entity_instance.as_i64(NUMBER_1).unwrap());
    assert_eq!(2, reactive_entity_instance.as_i64(NUMBER_2).unwrap());
    assert_eq!(3, reactive_entity_instance.as_i64(RESULT_1).unwrap());
}

#[test]
fn test_reactive_entity_instance_manager_behaviour_numeric_operation() {
    let application = init_application();
    let entity_type_manager = application.get_entity_type_manager();
    let reactive_entity_instance_manager = application.get_reactive_entity_instance_manager();

    let type_name = String::from(SinGate::TYPE_NAME);

    EntityTypeBuilder::new(type_name.clone())
        .number_property(NumericOperationProperties::LHS.to_string())
        .number_property(NumericOperationProperties::RESULT.to_string())
        .register(entity_type_manager.clone());

    let result = ReactiveEntityInstanceBuilder::new(type_name.clone())
        .property(NumericOperationProperties::LHS.to_string(), json!(0.0))
        .property(NumericOperationProperties::RESULT.to_string(), json!(0.0))
        .create(reactive_entity_instance_manager.clone());

    let reactive_entity_instance = result.unwrap();

    assert_eq!(0.0, reactive_entity_instance.as_f64(NumericOperationProperties::LHS.to_string()).unwrap());
    assert_eq!(0.0, reactive_entity_instance.as_f64(NumericOperationProperties::RESULT.to_string()).unwrap());

    // Note: The result should change now because of the behaviour "add"
    reactive_entity_instance.set(NumericOperationProperties::LHS.to_string(), json!(PI / 2.0));
    assert_eq!(PI / 2.0, reactive_entity_instance.as_f64(NumericOperationProperties::LHS.to_string()).unwrap());
    assert_eq!(1.0, reactive_entity_instance.as_f64(NumericOperationProperties::RESULT.to_string()).unwrap());

    reactive_entity_instance.set(NumericOperationProperties::LHS.to_string(), json!(PI / 6.0));
    assert_eq!(PI / 6.0, reactive_entity_instance.as_f64(NumericOperationProperties::LHS.to_string()).unwrap());
    assert!(assert_approx(0.5, reactive_entity_instance.as_f64(NumericOperationProperties::RESULT.to_string()).unwrap()));

}

fn assert_approx(expected: f64, value: f64) -> bool {
    value > expected - 0.00000001 && value < expected + 0.00000001
}
