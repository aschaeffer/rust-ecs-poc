use crate::model::{PropertyType, Component};
use crate::di::di_container;
use waiter_di::{profiles, Provider};
use crate::application::Application;
use std::env;
use random_string::{RandomString, Charsets, Charset};

#[test]
fn test_register_component () {
    let mut container = di_container::get::<profiles::Default>();
    let container = &mut container;
    let application = Provider::<dyn Application>::create(container);

    let component_name = RandomString::generate(
        10,
        &Charset::from_charsets(Charsets::Letters)
    ).to_string();

    let property_name = RandomString::generate(
        10,
        &Charset::from_charsets(Charsets::Letters)
    ).to_string();

    application.component_manager.register(
        Component::new(
            component_name.clone(),
            vec![
                PropertyType::new(
                    property_name.clone(),
                    String::from("string")
                )
            ]
        )
    );
    assert!(application.component_manager.has(component_name.clone()));

    let component: Component = application.component_manager.get(component_name.clone()).unwrap();
    assert_eq!(component_name, component.name);
    assert!(component.has_property(property_name.clone()));
    assert!(!component.has_property(
        RandomString::generate(10, &Charset::from_charsets(Charsets::Letters)).to_string()
    ));
}

#[test]
fn test_load_static_components () {
    let mut container = di_container::get::<profiles::Default>();
    let container = &mut container;
    let application = Provider::<dyn Application>::create(container);
    application.component_manager.load_static_components();
    assert!(application.component_manager.has(String::from("named")));
    assert!(application.component_manager.has(String::from("positionable")));
    assert!(application.component_manager.has(String::from("movable")));
    assert!(!application.component_manager.has(
        RandomString::generate(10, &Charset::from_charsets(Charsets::Letters)).to_string()
    ));
}


#[test]
fn test_get_components () {
    let mut container = di_container::get::<profiles::Default>();
    let container = &mut container;
    let application = Provider::<dyn Application>::create(container);
    application.component_manager.load_static_components();
    let components = application.component_manager.get_components();
    for component in components {
        assert!(application.component_manager.has(component.name));
    }
    assert!(!application.component_manager.has(
        RandomString::generate(10, &Charset::from_charsets(Charsets::Letters)).to_string()
    ));
}

#[test]
fn test_export_import_component () {
    let mut container = di_container::get::<profiles::Default>();
    let container = &mut container;
    let application = Provider::<dyn Application>::create(container);

    let component_name = RandomString::generate(
        10,
        &Charset::from_charsets(Charsets::Letters)
    ).to_string();

    let mut path = env::temp_dir();
    path.push(format!("{}.json", component_name));
    let path = path.into_os_string().into_string().unwrap();

    application.component_manager.create(
        component_name.clone(),
        vec![ PropertyType::new(String::from("x"), String::from("string")) ]
    );
    println!("Exporting to: {}", path.clone());
    application.component_manager.export(component_name.clone(), path.clone());
    assert!(application.component_manager.has(component_name.clone()));
    application.component_manager.delete(component_name.clone());
    assert!(!application.component_manager.has(component_name.clone()));
    println!("Importing from: {}", path.clone());
    application.component_manager.import(path.clone());
    assert!(application.component_manager.has(component_name.clone()));
}
