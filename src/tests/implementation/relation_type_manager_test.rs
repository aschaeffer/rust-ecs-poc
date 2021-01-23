use crate::application::Application;
use crate::di::di_container;
use crate::model::{EntityType, PropertyType};
use random_string::{Charset, Charsets, RandomString};
use std::env;
use waiter_di::{profiles, Provider};

#[test]
fn test_register_entity_type() {
    let mut container = di_container::get::<profiles::Default>();
    let container = &mut container;
    let application = Provider::<dyn Application>::create(container);

    let type_name =
        RandomString::generate(10, &Charset::from_charsets(Charsets::Letters)).to_string();

    application
        .entity_type_manager
        .register(crate::model::EntityType::new(
            type_name.clone(),
            vec![String::from("positionable")],
            vec![crate::model::PropertyType::new(
                String::from("x"),
                String::from("string"),
            )],
        ));
    assert!(application.entity_type_manager.has(type_name.clone()));

    let entity_type: Option<EntityType> = application.entity_type_manager.get(type_name.clone());
    assert_eq!(type_name, entity_type.unwrap().name);
}

#[test]
fn test_load_static_entity_types() {
    let mut container = di_container::get::<profiles::Default>();
    let container = &mut container;
    let application = Provider::<dyn Application>::create(container);
    application.entity_type_manager.load_static_entity_types();
    assert!(application.entity_type_manager.has(String::from("add")));
    assert!(application.entity_type_manager.has(String::from("and")));
    assert!(application.entity_type_manager.has(String::from("sin")));
    assert!(application.entity_type_manager.has(String::from("cos")));
    assert!(!application
        .entity_type_manager
        .has(RandomString::generate(10, &Charset::from_charsets(Charsets::Letters)).to_string()));
}

#[test]
fn test_create_and_delete_entity_type() {
    let mut container = di_container::get::<profiles::Default>();
    let container = &mut container;
    let application = Provider::<dyn Application>::create(container);

    let type_name =
        RandomString::generate(10, &Charset::from_charsets(Charsets::Letters)).to_string();

    application.entity_type_manager.create(
        type_name.clone(),
        vec![String::from("positionable")],
        vec![PropertyType::new(String::from("x"), String::from("string"))],
    );
    assert!(application.entity_type_manager.has(type_name.clone()));

    let entity_type: Option<EntityType> = application.entity_type_manager.get(type_name.clone());
    assert_eq!(type_name, entity_type.unwrap().name);

    application.entity_type_manager.delete(type_name.clone());

    assert!(!application.entity_type_manager.has(type_name.clone()));

    let entity_type: Option<EntityType> = application.entity_type_manager.get(type_name.clone());
    assert!(entity_type.is_none());
}

#[test]
fn test_get_entity_types() {
    let mut container = di_container::get::<profiles::Default>();
    let container = &mut container;
    let application = Provider::<dyn Application>::create(container);
    application.entity_type_manager.load_static_entity_types();
    let entity_types = application.entity_type_manager.get_entity_types();
    for entity_type in entity_types {
        assert!(application.entity_type_manager.has(entity_type.name));
    }
}

#[test]
fn test_register_entity_type_has_component() {
    let mut container = di_container::get::<profiles::Default>();
    let container = &mut container;
    let application = Provider::<dyn Application>::create(container);

    let component_name =
        RandomString::generate(10, &Charset::from_charsets(Charsets::Letters)).to_string();

    application
        .component_manager
        .register(crate::model::Component::new(
            component_name.clone(),
            vec![crate::model::PropertyType::new(
                String::from("x"),
                String::from("string"),
            )],
        ));

    let entity_type_name =
        RandomString::generate(10, &Charset::from_charsets(Charsets::Letters)).to_string();

    application
        .entity_type_manager
        .register(crate::model::EntityType::new(
            entity_type_name.clone(),
            vec![component_name.clone()],
            vec![crate::model::PropertyType::new(
                String::from("y"),
                String::from("string"),
            )],
        ));
    let entity_type: EntityType = application
        .entity_type_manager
        .get(entity_type_name.clone())
        .unwrap();
    assert!(entity_type.components.contains(&component_name.clone()));
    assert!(entity_type.is_a(component_name.clone()));
}

#[test]
fn test_register_entity_type_has_property() {
    let mut container = di_container::get::<profiles::Default>();
    let container = &mut container;
    let application = Provider::<dyn Application>::create(container);

    let property_name = String::from("x");
    let property_type = PropertyType::new(property_name.clone(), String::from("string"));

    let entity_type_name =
        RandomString::generate(10, &Charset::from_charsets(Charsets::Letters)).to_string();

    application.entity_type_manager.register(EntityType::new(
        entity_type_name.clone(),
        vec![],
        vec![property_type],
    ));
    let entity_type: Option<EntityType> = application
        .entity_type_manager
        .get(entity_type_name.clone());
    assert!(entity_type.unwrap().has_own_property(property_name.clone()));
}

#[test]
fn test_export_import_entity_type() {
    let mut container = di_container::get::<profiles::Default>();
    let container = &mut container;
    let application = Provider::<dyn Application>::create(container);

    let type_name =
        RandomString::generate(10, &Charset::from_charsets(Charsets::Letters)).to_string();

    let mut path = env::temp_dir();
    path.push(format!("{}.json", type_name));
    let path = path.into_os_string().into_string().unwrap();

    application.entity_type_manager.create(
        type_name.clone(),
        vec![String::from("positionable")],
        vec![PropertyType::new(String::from("x"), String::from("string"))],
    );
    info!("Exporting to: {}", path.clone());
    application
        .entity_type_manager
        .export(type_name.clone(), path.clone());
    assert!(application.entity_type_manager.has(type_name.clone()));
    application.entity_type_manager.delete(type_name.clone());
    assert!(!application.entity_type_manager.has(type_name.clone()));
    info!("Importing from: {}", path.clone());
    let result = application.entity_type_manager.import(path.clone());
    assert!(application.entity_type_manager.has(type_name.clone()));
    assert!(result.is_ok());
}
