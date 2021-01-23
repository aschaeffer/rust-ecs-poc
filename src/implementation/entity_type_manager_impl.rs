use crate::api::{GraphDatabase, EntityTypeImportError};
use crate::api::{ComponentManager, EntityTypeManager};
use crate::model::{EntityType, PropertyType};
use async_trait::async_trait;
use indradb::Type;
use rust_embed::RustEmbed;
use std::fs::File;
use std::io::BufReader;
use std::sync::RwLock;
use waiter_di::*;

#[derive(RustEmbed)]
#[folder = "static/types/entity"]
struct EntityTypeAsset;

#[wrapper]
pub struct EntityTypes(RwLock<std::vec::Vec<crate::model::EntityType>>);

#[provides]
fn create_external_type_dependency() -> EntityTypes {
    EntityTypes(RwLock::new(std::vec::Vec::new()))
}

#[component]
pub struct EntityTypeManagerImpl {
    graph_database: Wrc<dyn GraphDatabase>,

    component_manager: Wrc<dyn ComponentManager>,

    entity_types: EntityTypes,
}

#[async_trait]
#[provides]
impl EntityTypeManager for EntityTypeManagerImpl {
    fn register(&self, mut entity_type: crate::model::EntityType) {
        println!("Registered entity type {}", entity_type.name);
        // Construct the type
        entity_type.t = Type::new(entity_type.name.clone()).unwrap();
        for component_name in entity_type.components.to_vec() {
            let component = self.component_manager.get(component_name.clone());
            if component.is_some() {
                // println!("{} {:?}", component_name, component.unwrap());
                entity_type
                    .properties
                    .append(&mut component.unwrap().properties);
            } else {
                println!("No component named {}", component_name);
            }
        }
        self.entity_types.0.write().unwrap().push(entity_type);
        // let result = self.graph_database.get_transaction();
        // if result.is_ok() {
        //     let transaction = result.unwrap();
        //     let type_camera = Type::new("camera").unwrap();
        //     // transaction.c
        // }
    }

    fn load_static_entity_types(&self) {
        for file in EntityTypeAsset::iter() {
            let filename = file.as_ref();
            println!("Loading entity type from resource {}", filename);
            let asset = EntityTypeAsset::get(filename).unwrap();
            let result = std::str::from_utf8(asset.as_ref());
            if result.is_ok() {
                let json_str = result.unwrap();
                // println!("JSON {}", json_str);
                let entity_type: crate::model::EntityType = serde_json::from_str(json_str).unwrap();
                self.register(entity_type);
            } else {
                println!("Could not decode UTF-8 {}", filename)
            }
        }
    }

    fn get_entity_types(&self) -> Vec<crate::model::EntityType> {
        self.entity_types.0.read().unwrap().to_vec()
    }

    fn has(&self, name: String) -> bool {
        self.get(name).is_some()
    }

    fn get(&self, name: String) -> Option<EntityType> {
        self.entity_types
            .0
            .read()
            .unwrap()
            .to_vec()
            .into_iter()
            .find(|entity_type| entity_type.name == name)
    }

    fn create(&self, name: String, components: Vec<String>, properties: Vec<PropertyType>) {
        self.register(EntityType::new(
            name.clone(),
            components.to_vec(),
            properties.to_vec(),
        ));
    }

    fn delete(&self, name: String) {
        self.entity_types
            .0
            .write()
            .unwrap()
            .retain(|entity_type| entity_type.name != name);
    }

    fn import(&self, path: String) -> Result<EntityType, EntityTypeImportError> {
        let file = File::open(path);
        if file.is_ok() {
            let file = file.unwrap();
            let reader = BufReader::new(file);
            let entity_type = serde_json::from_reader(reader);
            if entity_type.is_ok() {
                let entity_type: EntityType = entity_type.unwrap();
                self.register(entity_type.clone());
                return Ok(entity_type.clone());
            }
        }
        Err(EntityTypeImportError.into())
    }

    fn export(&self, name: String, path: String) {
        let o_entity_type = self.get(name.clone());
        if o_entity_type.is_some() {
            let r_file = File::create(path.clone());
            match r_file {
                Ok(file) => {
                    let result = serde_json::to_writer_pretty(&file, &o_entity_type.unwrap());
                    if result.is_err() {
                        println!(
                            "Failed to export entity type {} to {}: {}",
                            name,
                            path,
                            result.err().unwrap()
                        );
                    }
                }
                Err(error) => {
                    println!(
                        "Failed to export entity type {} to {}: {}",
                        name,
                        path,
                        error.to_string()
                    );
                }
            }
        }
    }
}
