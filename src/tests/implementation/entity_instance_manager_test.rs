use indradb::Transaction;
use uuid::Uuid;

use crate::builder::{EntityInstanceBuilder, EntityTypeBuilder};
use crate::tests::r_json_string;
use crate::tests::utils::{init_application, r_string};
use std::env;

#[test]
fn test_entity_instance_manager() {
    let application = init_application();
    let entity_type_manager = application.get_entity_type_manager();
    let entity_instance_manager = application.get_entity_instance_manager();
    let graph_database = application.get_graph_database();
    let transaction = graph_database.get_transaction().unwrap();

    let type_name = r_string();
    let property_name = r_string();
    let property_value = r_json_string();

    assert_eq!(0, transaction.get_vertex_count().unwrap());

    // Check that we cannot create an entity instance with a type which doesn't exist
    let result = EntityInstanceBuilder::new(type_name.clone())
        .property(property_name.clone(), property_value.clone())
        .create(entity_instance_manager.clone());
    assert!(result.is_err());
    assert_eq!(0, transaction.get_vertex_count().unwrap());

    EntityTypeBuilder::new(type_name.clone())
        .string_property(property_name.clone())
        .register(entity_type_manager.clone());

    let result = EntityInstanceBuilder::new(type_name.clone())
        .property(property_name.clone(), property_value.clone())
        .create(entity_instance_manager.clone());
    assert!(result.is_ok());
    assert_eq!(1, transaction.get_vertex_count().unwrap());
    let uuid = result.unwrap();

    // Check if has returns false for a non-existent uuid
    assert!(!entity_instance_manager.has(Uuid::new_v4()));

    // Check if has returns true for the created entity
    assert!(entity_instance_manager.has(uuid));

    // Check if get returns none for a non-existent uuid
    assert!(entity_instance_manager.get(Uuid::new_v4()).is_none());

    // Check if get returns the created entity
    let entity_instance = entity_instance_manager.get(uuid);
    assert!(entity_instance.is_some());
    let entity_instance = entity_instance.unwrap();
    assert_eq!(uuid, entity_instance.id);
    assert_eq!(type_name.clone(), entity_instance.type_name.clone());

    // Check if we cannot create an entity with the same uuid
    let result = EntityInstanceBuilder::new(type_name.clone())
        .id(uuid)
        .property(property_name.clone(), property_value.clone())
        .create(entity_instance_manager.clone());
    assert!(result.is_err());
    assert_eq!(1, transaction.get_vertex_count().unwrap());

    // Check if we can create an another entity with a different uuid
    let another_uuid = Uuid::new_v4();
    let result = EntityInstanceBuilder::new(type_name.clone())
        .id(another_uuid)
        .property(property_name.clone(), property_value.clone())
        .create(entity_instance_manager.clone());
    assert!(result.is_ok());
    assert!(entity_instance_manager.has(another_uuid));
    assert!(entity_instance_manager.get(another_uuid).is_some());
    assert_eq!(2, transaction.get_vertex_count().unwrap());

    entity_instance_manager.delete(another_uuid);
    assert!(!entity_instance_manager.has(another_uuid));
    assert!(entity_instance_manager.get(another_uuid).is_none());
    assert_eq!(1, transaction.get_vertex_count().unwrap());
}

#[test]
fn test_entity_instance_manager_import_export() {
    let application = init_application();
    let entity_type_manager = application.get_entity_type_manager();
    let entity_instance_manager = application.get_entity_instance_manager();

    let type_name = r_string();
    let type_name = type_name.as_str();

    let mut path = env::temp_dir();
    path.push(format!("{}.json", type_name));
    let path = path.into_os_string().into_string().unwrap();

    // let type_name = r_string();
    let property_name = r_string();
    let property_value = r_json_string();
    EntityTypeBuilder::new(type_name)
        .string_property(property_name.clone())
        .register(entity_type_manager.clone());

    let result = EntityInstanceBuilder::new(type_name)
        .property(property_name.clone(), property_value.clone())
        .create(entity_instance_manager.clone());

    let uuid = result.unwrap();

    entity_instance_manager.export(uuid, path.clone());
    assert!(entity_instance_manager.has(uuid));
    entity_instance_manager.delete(uuid);
    assert!(!entity_instance_manager.has(uuid));
    let result = entity_instance_manager.import(path.clone());
    assert!(result.is_ok());
    assert_eq!(uuid, result.unwrap());
    assert!(entity_instance_manager.has(uuid));
}
