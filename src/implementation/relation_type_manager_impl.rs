use crate::api::{RelationTypeImportError, EntityTypeManager, Lifecycle};
use crate::api::{ComponentManager, RelationTypeManager};
use crate::model::{RelationType, PropertyType};
use async_trait::async_trait;
use indradb::Type;
use rust_embed::RustEmbed;
use std::fs::File;
use std::io::BufReader;
use std::sync::RwLock;
use waiter_di::*;
use log::{warn, debug, error};

#[derive(RustEmbed)]
#[folder = "static/types/relation"]
struct RelationTypeAsset;

#[wrapper]
pub struct RelationTypes(RwLock<std::vec::Vec<crate::model::RelationType>>);

#[provides]
fn create_external_type_dependency() -> RelationTypes {
    RelationTypes(RwLock::new(std::vec::Vec::new()))
}

#[component]
pub struct RelationTypeManagerImpl {
    component_manager: Wrc<dyn ComponentManager>,

    entity_type_manager: Wrc<dyn EntityTypeManager>,

    relation_types: RelationTypes,
}

#[async_trait]
#[provides]
impl RelationTypeManager for RelationTypeManagerImpl {
    fn register(&self, mut relation_type: crate::model::RelationType) {
        debug!("Registered relation type {}", relation_type.type_name.clone());
        // Construct the type
        relation_type.t = Type::new(relation_type.type_name.clone()).unwrap();
        if relation_type.outbound_type != "*" &&!self.entity_type_manager.has(relation_type.outbound_type.clone()) {
            warn!("Relation type {} not initialized: Outbound entity type does not exist {}", relation_type.type_name.clone(), relation_type.outbound_type.clone());
            // TODO: Result
            return
        }
        if relation_type.inbound_type != "*" &&!self.entity_type_manager.has(relation_type.inbound_type.clone()) {
            warn!("Relation type {} not initialized: Inbound entity type does not exist {}", relation_type.type_name.clone(), relation_type.inbound_type.clone());
            // TODO: Result
            return
        }
        for component_name in relation_type.components.to_vec() {
            let component = self.component_manager.get(component_name.clone());
            if component.is_some() {
                relation_type.properties
                    .append(&mut component.unwrap().properties);
            } else {
                warn!("Relation type {} not fully initialized: No component named {}", relation_type.type_name.clone(), component_name);
            }
        }

        self.relation_types.0.write().unwrap().push(relation_type);
        // TODO: Result
    }

    fn load_static_relation_types(&self) {
        for file in RelationTypeAsset::iter() {
            let filename = file.as_ref();
            debug!("Loading relation type from resource {}", filename);
            let asset = RelationTypeAsset::get(filename).unwrap();
            let result = std::str::from_utf8(asset.as_ref());
            if result.is_ok() {
                let json_str = result.unwrap();
                let relation_type: crate::model::RelationType = serde_json::from_str(json_str).unwrap();
                self.register(relation_type);
            } else {
                warn!("Could not decode UTF-8 {}", filename)
            }
        }
    }

    fn get_relation_types(&self) -> Vec<crate::model::RelationType> {
        self.relation_types.0.read().unwrap().to_vec()
    }

    fn has(&self, type_name: String) -> bool {
        self.get(type_name).is_some()
    }

    fn get(&self, type_name: String) -> Option<RelationType> {
        self.relation_types.0.read().unwrap()
            .to_vec().into_iter()
            .find(|relation_type| relation_type.type_name == type_name)
    }

    fn get_starts_with(&self, type_name_starts_with: String) -> Option<RelationType> {
        self.relation_types.0.read().unwrap()
            .to_vec().into_iter()
            .find(|relation_type| type_name_starts_with.starts_with(relation_type.type_name.as_str()))
    }

    fn create(&self, outbound_type: String, type_name: String, inbound_type: String, components: Vec<String>, behaviours: Vec<String>, properties: Vec<PropertyType>) {
        self.register(RelationType::new(
            outbound_type.clone(),
            type_name.clone(),
            inbound_type.clone(),
            components.to_vec(),
            behaviours.to_vec(),
            properties.to_vec(),
        ));
    }

    fn delete(&self, type_name: String) {
        self.relation_types.0.write().unwrap()
            .retain(|relation_type| relation_type.type_name != type_name);
    }

    fn import(&self, path: String) -> Result<RelationType, RelationTypeImportError> {
        let file = File::open(path);
        if file.is_ok() {
            let file = file.unwrap();
            let reader = BufReader::new(file);
            let relation_type = serde_json::from_reader(reader);
            if relation_type.is_ok() {
                let relation_type: RelationType = relation_type.unwrap();
                self.register(relation_type.clone());
                return Ok(relation_type.clone());
            }
        }
        Err(RelationTypeImportError.into())
    }

    fn export(&self, type_name: String, path: String) {
        let o_relation_type = self.get(type_name.clone());
        if o_relation_type.is_some() {
            let r_file = File::create(path.clone());
            match r_file {
                Ok(file) => {
                    let result = serde_json::to_writer_pretty(&file, &o_relation_type.unwrap());
                    if result.is_err() {
                        error!("Failed to export relation type {} to {}: {}",
                               type_name, path, result.err().unwrap()
                        );
                    }
                }
                Err(error) => {
                    error!("Failed to export relation type {} to {}: {}",
                           type_name, path, error.to_string());
                }
            }
        }
    }
}

impl Lifecycle for RelationTypeManagerImpl {
    fn init(&self) {
        self.load_static_relation_types();
    }

    fn shutdown(&self) {
        // TODO: self.clear_relation_types();
    }
}
