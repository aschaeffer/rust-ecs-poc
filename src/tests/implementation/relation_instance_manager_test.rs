use std::env;

use uuid::Uuid;

use crate::builder::{EntityInstanceBuilder, EntityTypeBuilder, RelationInstanceBuilder, RelationTypeBuilder};
use crate::tests::r_json_string;
use crate::tests::utils::{init_application, r_string};
use indradb::{EdgeKey, Type};

#[test]
fn test_relation_instance_manager() {
    let application = init_application();
    let entity_type_manager = application.get_entity_type_manager();
    let entity_instance_manager = application.get_entity_instance_manager();
    let relation_type_manager = application.get_relation_type_manager();
    let relation_instance_manager = application.get_relation_instance_manager();

    let outbound_type = r_string();
    let outbound_id = Uuid::new_v4();
    let type_name = r_string();
    let inbound_type = r_string();
    let inbound_id = Uuid::new_v4();
    let property_name = r_string();
    let property_value = r_json_string();

    let edge_key = EdgeKey::new(outbound_id, Type::new(type_name.clone()).unwrap(), inbound_id);

    EntityTypeBuilder::new(outbound_type.clone())
        .string_property(property_name.clone())
        .register(entity_type_manager.clone());
    EntityTypeBuilder::new(inbound_type.clone())
        .string_property(property_name.clone())
        .register(entity_type_manager.clone());

    let result = EntityInstanceBuilder::new(outbound_type.clone())
        .id(outbound_id)
        .property(property_name.clone(), property_value.clone())
        .create(entity_instance_manager.clone());
    assert!(result.is_ok());

    let result = EntityInstanceBuilder::new(inbound_type.clone())
        .id(inbound_id)
        .property(property_name.clone(), property_value.clone())
        .create(entity_instance_manager.clone());
    assert!(result.is_ok());

    // Check that we cannot create an relation instance with a type which doesn't exist
    let result = RelationInstanceBuilder::new(outbound_id, type_name.clone(), inbound_id)
        .property(property_name.clone(), property_value.clone())
        .create(relation_instance_manager.clone());
    assert!(result.is_err());

    RelationTypeBuilder::new(outbound_type.clone(), type_name.clone(), inbound_type.clone())
        .string_property(property_name.clone())
        .register(relation_type_manager.clone());

    // Check that we cannot create the a relation instance with a non-existent outbound
    let result = RelationInstanceBuilder::new(Uuid::new_v4(), type_name.clone(), inbound_id)
        .property(property_name.clone(), property_value.clone())
        .create(relation_instance_manager.clone());
    assert!(result.is_err());

    // Check that we cannot create the a relation instance with a non-existent inbound
    let result = RelationInstanceBuilder::new(outbound_id, type_name.clone(), Uuid::new_v4())
        .property(property_name.clone(), property_value.clone())
        .create(relation_instance_manager.clone());
    assert!(result.is_err());

    let result = RelationInstanceBuilder::new(outbound_id, type_name.clone(), inbound_id.clone())
        .property(property_name.clone(), property_value.clone())
        .create(relation_instance_manager.clone());
    assert!(result.is_ok());
    let actual_edge_key = result.unwrap();
    assert_eq!(outbound_id, actual_edge_key.outbound_id);
    assert_eq!(type_name.clone(), actual_edge_key.t.0);
    assert_eq!(inbound_id, actual_edge_key.inbound_id);

    // Check if has returns false for a non-existent uuid
    let wrong_outbound_id_edge_key = EdgeKey::new(Uuid::new_v4(), Type::new(type_name.clone()).unwrap(), inbound_id);
    let wrong_inbound_id_edge_key = EdgeKey::new(outbound_id, Type::new(type_name.clone()).unwrap(), Uuid::new_v4());
    // let wrong_outbound_id_edge_key = EdgeKey::new(outbound_id, Type::new(type_name.clone()).unwrap(), inbound_id);

    assert!(!relation_instance_manager.has(wrong_outbound_id_edge_key.clone()));
    assert!(!relation_instance_manager.has(wrong_inbound_id_edge_key.clone()));

    // Check if has returns true for the created relation
    assert!(relation_instance_manager.has(edge_key.clone()));

    // Check if get returns none for a non-existent uuid
    assert!(relation_instance_manager.get(wrong_outbound_id_edge_key.clone()).is_none());
    assert!(relation_instance_manager.get(wrong_inbound_id_edge_key.clone()).is_none());

    // Check if get returns the created relation
    let relation_instance = relation_instance_manager.get(edge_key.clone());
    assert!(relation_instance.is_some());
    let relation_instance = relation_instance.unwrap();
    assert_eq!(outbound_id, relation_instance.outbound_id);
    assert_eq!(inbound_id, relation_instance.inbound_id);
    assert_eq!(type_name.clone(), relation_instance.type_name.clone());

    // Check that we cannot create the same relation instance twice
    let result = RelationInstanceBuilder::new(outbound_id, type_name.clone(), inbound_id)
        .property(property_name.clone(), property_value.clone())
        .create(relation_instance_manager.clone());
    assert!(result.is_err());
    let result = RelationInstanceBuilder::from(edge_key.clone())
        .property(property_name.clone(), property_value.clone())
        .create(relation_instance_manager.clone());
    assert!(result.is_err());


    relation_instance_manager.delete(edge_key.clone());
    assert!(!relation_instance_manager.has(edge_key.clone()));
    assert!(relation_instance_manager.get(edge_key.clone()).is_none());
}

#[test]
fn test_relation_instance_manager_import_export() {
    let application = init_application();
    let entity_type_manager = application.get_entity_type_manager();
    let entity_instance_manager = application.get_entity_instance_manager();
    let relation_type_manager = application.get_relation_type_manager();
    let relation_instance_manager = application.get_relation_instance_manager();

    let outbound_type = r_string();
    let outbound_id = Uuid::new_v4();
    let type_name = r_string();
    let inbound_type = r_string();
    let inbound_id = Uuid::new_v4();
    let property_name = r_string();
    let property_value = r_json_string();

    let edge_key = EdgeKey::new(outbound_id, Type::new(type_name.clone()).unwrap(), inbound_id);

    EntityTypeBuilder::new(outbound_type.clone())
        .string_property(property_name.clone())
        .register(entity_type_manager.clone());
    EntityTypeBuilder::new(inbound_type.clone())
        .string_property(property_name.clone())
        .register(entity_type_manager.clone());

    let result = EntityInstanceBuilder::new(outbound_type.clone())
        .id(outbound_id)
        .property(property_name.clone(), property_value.clone())
        .create(entity_instance_manager.clone());
    assert!(result.is_ok());

    let result = EntityInstanceBuilder::new(inbound_type.clone())
        .id(inbound_id)
        .property(property_name.clone(), property_value.clone())
        .create(entity_instance_manager.clone());
    assert!(result.is_ok());

    let mut path = env::temp_dir();
    path.push(format!("{}-{}-{}.json", outbound_id.to_string().as_str(), type_name.clone(), inbound_id.to_string().as_str()));
    let path = path.into_os_string().into_string().unwrap();

    RelationTypeBuilder::new(outbound_type, type_name.clone(), inbound_type)
        .string_property(property_name.clone())
        .register(relation_type_manager.clone());

    let result = RelationInstanceBuilder::new(outbound_id, type_name.clone(), inbound_id)
        .property(property_name.clone(), property_value.clone())
        .create(relation_instance_manager.clone());

    let actual_edge_key = result.unwrap();
    assert_eq!(outbound_id, actual_edge_key.outbound_id);
    assert_eq!(type_name.clone(), actual_edge_key.t.0);
    assert_eq!(inbound_id, actual_edge_key.inbound_id);

    relation_instance_manager.export(edge_key.clone(), path.clone());
    assert!(relation_instance_manager.has(edge_key.clone()));
    relation_instance_manager.delete(edge_key.clone());
    assert!(!relation_instance_manager.has(edge_key.clone()));
    let result = relation_instance_manager.import(path.clone());
    assert!(result.is_ok());
    let relation_instance = result.unwrap();
    assert_eq!(outbound_id, relation_instance.outbound_id);
    assert_eq!(type_name.clone(), relation_instance.type_name.clone());
    assert_eq!(inbound_id, relation_instance.inbound_id);
    assert!(relation_instance_manager.has(edge_key.clone()));
}
