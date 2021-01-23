use crate::api::{GraphDatabase, RelationTypeImportError};
use crate::api::{ComponentManager, RelationTypeManager};
use crate::model::{RelationType, PropertyType};
use async_trait::async_trait;
use indradb::Type;
use rust_embed::RustEmbed;
use std::fs::File;
use std::io::BufReader;
use std::sync::RwLock;
use waiter_di::*;

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
    graph_database: Wrc<dyn GraphDatabase>,

    component_manager: Wrc<dyn ComponentManager>,

    relation_types: RelationTypes,
}

#[async_trait]
#[provides]
impl RelationTypeManager for RelationTypeManagerImpl {
    fn register(&self, mut relation_type: crate::model::RelationType) {
        println!("Registered relation type {}", relation_type.name);
        // Construct the type
        relation_type.t = Type::new(relation_type.name.clone()).unwrap();
        for component_name in relation_type.components.to_vec() {
            let component = self.component_manager.get(component_name.clone());
            if component.is_some() {
                // println!("{} {:?}", component_name, component.unwrap());
                relation_type
                    .properties
                    .append(&mut component.unwrap().properties);
            } else {
                println!("No component named {}", component_name);
            }
        }
        self.relation_types.0.write().unwrap().push(relation_type);
        // let result = self.graph_database.get_transaction();
        // if result.is_ok() {
        //     let transaction = result.unwrap();
        //     let type_camera = Type::new("camera").unwrap();
        //     // transaction.c
        // }
    }

    fn load_static_relation_types(&self) {
        // self.component_manager.list_components();
        for file in RelationTypeAsset::iter() {
            let filename = file.as_ref();
            println!("Loading relation type from resource {}", filename);
            let asset = RelationTypeAsset::get(filename).unwrap();
            let result = std::str::from_utf8(asset.as_ref());
            if result.is_ok() {
                let json_str = result.unwrap();
                // println!("JSON {}", json_str);
                let relation_type: crate::model::RelationType = serde_json::from_str(json_str).unwrap();
                self.register(relation_type);
            } else {
                println!("Could not decode UTF-8 {}", filename)
            }
        }
    }

    fn get_relation_types(&self) -> Vec<crate::model::RelationType> {
        self.relation_types.0.read().unwrap().to_vec()
    }

    fn list_relation_types(&self) {
        for relation_type in self.relation_types.0.read().unwrap().iter() {
            println!("Relation Type {}: {:?}", relation_type.name, relation_type);
        }
    }

    fn has(&self, name: String) -> bool {
        self.get(name).is_some()
    }

    fn get(&self, name: String) -> Option<RelationType> {
        self.relation_types
            .0
            .read()
            .unwrap()
            .to_vec()
            .into_iter()
            .find(|relation_type| relation_type.name == name)
    }

    fn create(&self, name: String, outbound_type: String, inbound_type: String, components: Vec<String>, properties: Vec<PropertyType>) {
        self.register(RelationType::new(
            name.clone(),
            outbound_type,
            inbound_type,
            components.to_vec(),
            properties.to_vec(),
        ));
    }

    fn delete(&self, name: String) {
        self.relation_types
            .0
            .write()
            .unwrap()
            .retain(|relation_type| relation_type.name != name);
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

    fn export(&self, name: String, path: String) {
        let o_relation_type = self.get(name.clone());
        if o_relation_type.is_some() {
            let r_file = File::create(path.clone());
            match r_file {
                Ok(file) => {
                    let result = serde_json::to_writer_pretty(&file, &o_relation_type.unwrap());
                    if result.is_err() {
                        println!(
                            "Failed to export relation type {} to {}: {}",
                            name,
                            path,
                            result.err().unwrap()
                        );
                    }
                }
                Err(error) => {
                    println!(
                        "Failed to export relation type {} to {}: {}",
                        name,
                        path,
                        error.to_string()
                    );
                }
            }
        }
    }
}
