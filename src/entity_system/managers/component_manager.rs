extern crate waiter_di;
extern crate serde;

use waiter_di::*;
// use waiter_di::{component,provides,module};
use rust_embed::RustEmbed;
use std::borrow::Borrow;
// use crate::entity_system::model::component::Component;

#[derive(RustEmbed)]
#[folder = "static/components"]
struct ComponentAsset;

#[wrapper]
pub struct Components(std::vec::Vec<crate::entity_system::model::component::Component>);

#[provides]
fn create_external_type_dependency() -> Components {
    Components(std::vec::Vec::new())
}

// wrap!(std::vec::Vec<Component> as VecComponent);

pub trait ComponentManager : Send { //  (thread boundaries)
    // fn new() -> Self;
    fn register (&mut self, component: crate::entity_system::model::component::Component);
    fn load_static_components (&mut self);
    fn get_components (&self) -> Vec<crate::entity_system::model::component::Component>;
    fn test (&mut self);
}

#[component]
pub struct ComponentManagerImpl {
    pub components: Components,
    // pub a: i32,
    // pub b: i32,

}

#[provides]
impl ComponentManager for ComponentManagerImpl {

    // #[provides]
    // fn new() -> Self {
    //     ComponentManagerImpl {
    //         components: Vec::new()
    //     }
    // }

    // fn new() -> Self {
    //     ComponentManagerImpl {
    //         components: Vec::new()
    //     }
    // }

    fn register (&mut self, component: crate::entity_system::model::component::Component) {
        // self.components.push(component);
        println!("Registered component {}", component.name);
        // let x = self.components.0.as_mut();
        // self.components.0.borrow()push(component);
        // self.components.0.get(0);
        // self.components.0.push(component);
    }

    fn load_static_components (&mut self) {
        for file in ComponentAsset::iter() {
            let filename = file.as_ref();
            println!("Loading component from resource {}", filename);
            let component_asset = ComponentAsset::get(filename).unwrap();
            let result = std::str::from_utf8(component_asset.as_ref());
            if result.is_ok() {
                let json_str = result.unwrap();
                println!("JSON {}", json_str);
                let component: crate::entity_system::model::component::Component = serde_json::from_str(json_str).unwrap();
                self.register(component);
            } else {
                println!("Could not decode UTF-8 {}", filename)
            }
        }
    }

    fn get_components (&self) -> Vec<crate::entity_system::model::component::Component> {
        for file in ComponentAsset::iter() {
            let filename = file.as_ref();
            println!("Loading component from resource {}", filename);
            let component_asset = ComponentAsset::get(filename).unwrap();
            let result = std::str::from_utf8(component_asset.as_ref());
            if result.is_ok() {
                let json_str = result.unwrap();
                println!("JSON {}", json_str);
                let component: crate::entity_system::model::component::Component = serde_json::from_str(json_str).unwrap();
                // self.components.0.push(component);
            } else {
                println!("Could not decode UTF-8 {}", filename)
            }
        }

        self.components.0.to_vec()
    }

    fn test (&mut self) {
        println!("Test");
    }

}


#[module]
struct ComponentManagerModule {
    component_manager: Box<dyn ComponentManager>
}


#[module]
struct RootModule {
    component_manager_module: ComponentManagerModule
}


// #[provides]
// fn create_component_manager(components: VecComponent) -> ComponentManagerImpl {
//     println!("create_component_manager");
//     ComponentManagerImpl {
//         components
//     }
// }

// #[provides]
// fn create_external_type_dependency() -> VecComponent {
//     std::vec::Vec::new()
// }

// #[component]
// impl ComponentManagerImpl {
//     #[provides]
//     fn new() -> Self {
//         ComponentManagerImpl {
//             components: Vec::new()
//         }
//     }
// }
