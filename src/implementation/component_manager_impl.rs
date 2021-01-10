use crate::api::ComponentManager;
use async_trait::async_trait;
use rust_embed::RustEmbed;
use waiter_di::*;
use std::sync::RwLock;
use crate::model::PropertyType;
use std::fs::File;
use std::io::BufReader;

#[derive(RustEmbed)]
#[folder = "static/components"]
struct ComponentAsset;

#[wrapper]
pub struct Components(RwLock<std::vec::Vec<crate::model::Component>>);

#[provides]
fn create_external_type_dependency() -> Components {
    Components(RwLock::new(std::vec::Vec::new()))
}

#[component]
pub struct ComponentManagerImpl {
    components: Components,
    // pub test_string: String,
}

#[async_trait]
#[provides]
impl ComponentManager for ComponentManagerImpl {

    fn register (&self, component: crate::model::Component) {
        if !self.has(component.name.clone()) {
            println!("Registered component {}", component.name);
            self.components.0
                .write()
                .unwrap()
                .push(component);
        }
    }

    fn load_static_components (&self) {
        for file in ComponentAsset::iter() {
            let filename = file.as_ref();
            println!("Loading component from resource {}", filename);
            let asset = ComponentAsset::get(filename).unwrap();
            let result = std::str::from_utf8(asset.as_ref());
            if result.is_ok() {
                let json_str = result.unwrap();
                let component: crate::model::Component = serde_json::from_str(json_str).unwrap();
                self.register(component);
            } else {
                println!("Could not decode UTF-8 {}", filename)
            }
        }
    }

    // Returns a copy
    fn get_components (&self) -> Vec<crate::model::Component> {
        self.components.0.read().unwrap().to_vec()
    }

    fn list_components(&self) {
        for component in self.components.0.read().unwrap().to_vec().iter() {
            println!("Component {}: {:?}", component.name.clone(), component);
        }
    }

    fn has(&self, name: String) -> bool {
        self.get(name).is_some()
    }

    fn get(&self, name: String) -> Option<crate::model::Component> {
        self.components.0
            .read()
            .unwrap()
            .to_vec()
            .into_iter()
            .find(| component | component.name == name)
    }

    fn create(&self, name: String, properties: Vec<PropertyType>) {
        self.register(crate::model::Component::new(
            name.clone(),
            properties.to_vec()
        ));
    }

    fn delete(&self, name: String) {
        self.components.0
            .write()
            .unwrap()
            .retain(| component | component.name != name);
    }

    fn import(&self, path: String) {
        let file = File::open(path);
        if file.is_ok() {
            let file = file.unwrap();
            let reader = BufReader::new(file);
            let component = serde_json::from_reader(reader);
            if component.is_ok() {
                self.register(component.unwrap());
            }
        }
    }

    fn export(&self, name: String, path: String) {
        let o_component = self.get(name.clone());
        if o_component.is_some() {
            let r_file = File::create(path.clone());
            match r_file {
                Ok(file) => {
                    let result = serde_json::to_writer_pretty(
                        &file,
                        &o_component.unwrap()
                    );
                    if result.is_err() {
                        println!("Failed to export component {} to {}: {}", name, path, result.err().unwrap());
                    }
                },
                Err(error) => {
                    println!("Failed to export component {} to {}: {}", name, path, error.to_string());
                }
            }
        }
    }

}
