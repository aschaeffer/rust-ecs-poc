use crate::api::{EntityTypeImportError, Lifecycle};
use crate::api::{ComponentManager, EntityTypeManager};
use crate::model::{EntityType, PropertyType};
use async_trait::async_trait;
use indradb::Type;
use rust_embed::RustEmbed;
use std::fs::File;
use std::io::BufReader;
use std::sync::RwLock;
use waiter_di::*;
use log::{warn, debug, error};

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
    component_manager: Wrc<dyn ComponentManager>,

    entity_types: EntityTypes,
}

#[async_trait]
#[provides]
impl EntityTypeManager for EntityTypeManagerImpl {
    fn register(&self, mut entity_type: crate::model::EntityType) {
        debug!("Registered entity type {}", entity_type.name);
        // Construct the type
        entity_type.t = Type::new(entity_type.name.clone()).unwrap();
        for component_name in entity_type.components.to_vec() {
            let component = self.component_manager.get(component_name.clone());
            if component.is_some() {
                entity_type.properties
                    .append(&mut component.unwrap().properties);
            } else {
                warn!("Entity type {} not fully initialized: No component named {}", entity_type.name.clone(), component_name);
            }
        }
        self.entity_types.0.write().unwrap().push(entity_type);
    }

    fn load_static_entity_types(&self) {
        for file in EntityTypeAsset::iter() {
            let filename = file.as_ref();
            debug!("Loading entity type from resource {}", filename);
            let asset = EntityTypeAsset::get(filename).unwrap();
            let result = std::str::from_utf8(asset.as_ref());
            if result.is_ok() {
                let json_str = result.unwrap();
                let entity_type: crate::model::EntityType = serde_json::from_str(json_str).unwrap();
                self.register(entity_type);
            } else {
                error!("Could not decode UTF-8 {}", filename)
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
        self.entity_types.0.read().unwrap()
            .to_vec().into_iter()
            .find(|entity_type| entity_type.name == name)
    }

    fn create(&self, name: String, components: Vec<String>, behaviours: Vec<String>, properties: Vec<PropertyType>) {
        self.register(EntityType::new(
            name.clone(),
            components.to_vec(),
            behaviours.to_vec(),
            properties.to_vec(),
        ));
    }

    fn delete(&self, name: String) {
        self.entity_types.0.write().unwrap()
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
                        error!("Failed to export entity type {} to {}: {}",
                               name, path, result.err().unwrap());
                    }
                }
                Err(error) => {
                    error!("Failed to export entity type {} to {}: {}",
                           name, path, error.to_string());
                }
            }
        }
    }
}

impl Lifecycle for EntityTypeManagerImpl {
    fn init(&self) {
        self.load_static_entity_types();
    }

    fn shutdown(&self) {
        // TODO: self.clear_entity_types();
    }
}
