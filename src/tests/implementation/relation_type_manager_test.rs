use std::env;

use crate::model::{PropertyType, RelationType, DataType};
use crate::tests::utils::{init_application, r_string};
use crate::builder::EntityTypeBuilder;

#[test]
fn test_register_relation_type() {
    let application = init_application();
    let entity_type_manager = application.get_entity_type_manager();
    let relation_type_manager = application.get_relation_type_manager();

    let type_name = r_string();
    let outbound_type_name = r_string();
    let inbound_type_name = r_string();

    EntityTypeBuilder::new(outbound_type_name.clone()).register(entity_type_manager.clone());
    EntityTypeBuilder::new(inbound_type_name.clone()).register(entity_type_manager.clone());
    relation_type_manager
        .register(crate::model::RelationType::new(
            outbound_type_name.clone(),
            type_name.clone(),
            inbound_type_name.clone(),
            vec![String::from("named")],
            Vec::new(),
            vec![crate::model::PropertyType::new(
                String::from("x"),
                DataType::String,
            )],
        ));
    assert!(relation_type_manager.has(type_name.clone()));

    let relation_type: Option<RelationType> = relation_type_manager.get(type_name.clone());
    assert_eq!(type_name, relation_type.unwrap().type_name);
}

#[test]
fn test_load_static_relation_types() {
    let application = init_application();
    let entity_type_manager = application.get_entity_type_manager();
    let relation_type_manager = application.get_relation_type_manager();
    entity_type_manager.load_static_entity_types();
    relation_type_manager.load_static_relation_types();
    assert!(relation_type_manager.has(String::from("default_connector")));
    assert!(relation_type_manager.has(String::from("commented_with")));
    assert!(relation_type_manager.has(String::from("tagged_with")));
    assert!(!relation_type_manager.has(r_string()));
}

#[test]
fn test_create_and_delete_relation_type() {
    let application = init_application();
    let entity_type_manager = application.get_entity_type_manager();
    let relation_type_manager = application.get_relation_type_manager();

    let type_name = r_string();
    let outbound_type_name = r_string();
    let inbound_type_name = r_string();

    EntityTypeBuilder::new(outbound_type_name.clone()).register(entity_type_manager.clone());
    EntityTypeBuilder::new(inbound_type_name.clone()).register(entity_type_manager.clone());
    relation_type_manager.create(
        outbound_type_name.clone(),
        type_name.clone(),
        inbound_type_name.clone(),
        vec![String::from("positionable")],
        Vec::new(),
        vec![PropertyType::new(String::from("x"), DataType::String)],
    );
    assert!(relation_type_manager.has(type_name.clone()));

    let relation_type: Option<RelationType> = relation_type_manager.get(type_name.clone());
    assert_eq!(type_name, relation_type.unwrap().type_name);

    relation_type_manager.delete(type_name.clone());

    assert!(!relation_type_manager.has(type_name.clone()));

    let relation_type: Option<RelationType> = relation_type_manager.get(type_name.clone());
    assert!(relation_type.is_none());
}

#[test]
fn test_get_relation_types() {
    let application = init_application();
    let relation_type_manager = application.get_relation_type_manager();
    relation_type_manager.load_static_relation_types();
    let relation_types = relation_type_manager.get_relation_types();
    for relation_type in relation_types {
        assert!(relation_type_manager.has(relation_type.type_name));
    }
}

#[test]
fn test_register_relation_type_has_component() {
    let application = init_application();
    let component_manager = application.get_component_manager();
    let entity_type_manager = application.get_entity_type_manager();
    let relation_type_manager = application.get_relation_type_manager();

    let component_name = r_string();

    component_manager
        .register(crate::model::Component::new(
            component_name.clone(),
            vec![crate::model::PropertyType::new(
                String::from("x"),
                DataType::String,
            )],
        ));

    let relation_type_name = r_string();
    let outbound_type_name = r_string();
    let inbound_type_name = r_string();

    EntityTypeBuilder::new(outbound_type_name.clone()).register(entity_type_manager.clone());
    EntityTypeBuilder::new(inbound_type_name.clone()).register(entity_type_manager.clone());
    relation_type_manager
        .register(crate::model::RelationType::new(
            outbound_type_name.clone(),
            relation_type_name.clone(),
            inbound_type_name.clone(),
            vec![component_name.clone()],
            Vec::new(),
            vec![crate::model::PropertyType::new(
                String::from("y"),
                DataType::String,
            )],
        ));
    let relation_type: RelationType = relation_type_manager.get(relation_type_name.clone()).unwrap();
    assert!(relation_type.components.contains(&component_name.clone()));
    assert!(relation_type.is_a(component_name.clone()));
}

#[test]
fn test_register_relation_type_has_property() {
    let application = init_application();
    let entity_type_manager = application.get_entity_type_manager();
    let relation_type_manager = application.get_relation_type_manager();

    let property_name = String::from("x");
    let property_type = PropertyType::new(property_name.clone(), DataType::String);

    let relation_type_name = r_string();
    let outbound_type_name = r_string();
    let inbound_type_name = r_string();

    EntityTypeBuilder::new(outbound_type_name.clone()).register(entity_type_manager.clone());
    EntityTypeBuilder::new(inbound_type_name.clone()).register(entity_type_manager.clone());
    relation_type_manager.register(RelationType::new(
        outbound_type_name.clone(),
        relation_type_name.clone(),
        inbound_type_name.clone(),
        Vec::new(),
        Vec::new(),
        vec![property_type],
    ));
    let relation_type: Option<RelationType> = relation_type_manager.get(relation_type_name.clone());
    assert!(relation_type.unwrap().has_own_property(property_name.clone()));
}

#[test]
fn test_export_import_relation_type() {
    let application = init_application();
    let entity_type_manager = application.get_entity_type_manager();
    let relation_type_manager = application.get_relation_type_manager();

    let type_name = r_string();
    let outbound_type_name = r_string();
    let inbound_type_name = r_string();

    let mut path = env::temp_dir();
    path.push(format!("{}.json", type_name));
    let path = path.into_os_string().into_string().unwrap();

    EntityTypeBuilder::new(outbound_type_name.clone()).register(entity_type_manager.clone());
    EntityTypeBuilder::new(inbound_type_name.clone()).register(entity_type_manager.clone());
    relation_type_manager.create(
        outbound_type_name.clone(),
        type_name.clone(),
        inbound_type_name.clone(),
        vec![String::from("positionable")],
        Vec::new(),
        vec![PropertyType::new(String::from("x"), DataType::String)],
    );
    relation_type_manager.export(type_name.clone(), path.clone());
    assert!(relation_type_manager.has(type_name.clone()));
    relation_type_manager.delete(type_name.clone());
    assert!(!relation_type_manager.has(type_name.clone()));
    let result = relation_type_manager.import(path.clone());
    assert!(relation_type_manager.has(type_name.clone()));
    assert!(result.is_ok());
}
