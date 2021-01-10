use crate::api::*;
use waiter_di::*;
use async_trait::async_trait;

#[async_trait]
pub trait Application {
    // fn new() -> Self where Self: Sized;
    async fn init(&mut self);
    async fn run(&mut self);
}

#[module]
pub struct ApplicationImpl {
    component_manager: Wrc<dyn ComponentManager>,
    entity_type_manager: Wrc<dyn EntityTypeManager>,
    system_constants_initializer: Wrc<dyn SystemConstantsInitializer>,
    graph_database: Wrc<dyn GraphDatabase>,
}

#[async_trait]
#[provides]
impl Application for ApplicationImpl {

    async fn init(&mut self) {
        println!("--- INIT ---");
        self.component_manager.load_static_components();
        self.component_manager.list_components();
        self.entity_type_manager.load_static_entity_types();
        self.entity_type_manager.list_entity_types();
        self.system_constants_initializer.activate();
    }

    async fn run(&mut self) {
        println!("--- entity_type_manager.register ---");
        self.entity_type_manager.register(
            crate::model::EntityType::new(
                String::from("test1"),
                vec![
                    String::from("positionable")
                ],
                vec![
                    crate::model::PropertyType::new(String::from("x"), String::from("string"))
                ]
            )
        );
        println!("--- entity_type_manager.create ---");
        self.entity_type_manager.create(
            String::from("test2"),
            vec![
                String::from("positionable"),
                String::from("movable")
            ],
            vec![

            ]
        );
        println!("--- entity_type_manager.list_entity_types ---");
        self.entity_type_manager.list_entity_types();
        println!("--- component_manager.list_components ---");
        self.component_manager.list_components();
        println!("--- component_manager.create ---");
        self.component_manager.create(
            String::from("dynamically_created"),
            vec![
                crate::model::PropertyType::new(String::from("x"), String::from("string")),
            ]
        );
        println!("--- component_manager.list_components ---");
        self.component_manager.list_components();
        println!("--- entity_type_manager.list_entity_types ---");
        self.entity_type_manager.list_entity_types();
        println!("--- entity_type_manager.delete ---");
        self.entity_type_manager.delete(String::from("camera"));
        println!("--- component_manager.list_components ---");
        self.component_manager.list_components();
        println!("--- list_components.delete ---");
        self.component_manager.delete(String::from("dynamically_created"));
        println!("--- component_manager.list_components ---");
        self.component_manager.list_components();
        println!("--- component_manager.get -> JSON ---");
        let component = self.component_manager.get(String::from("movable")).unwrap();
        println!("{}", serde_json::to_string(&component).unwrap());
        self.component_manager.export(String::from("movable"), String::from("C:\\movable.json"));
        self.component_manager.export(String::from("movable"), String::from("movable.json"));
        self.component_manager.list_components();
        self.component_manager.import(String::from("imported_component.json"));
        self.component_manager.list_components();
    }

}
