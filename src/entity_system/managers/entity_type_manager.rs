// extern crate waiter_di;
// extern crate serde;
//
// use waiter_di::*;
// use rust_embed::RustEmbed;
// use indradb::MemoryDatastore;
//
// use crate::entity_system::model::entity_type::EntityType;
// use crate::entity_system::managers::component_manager::ComponentManager;
//
// #[derive(RustEmbed)]
// #[folder = "static/types/entity"]
// struct EntityTypeAsset;
//
// pub trait EntityTypeManager {
//     fn register (&mut self, entity_type: EntityType);
//     fn load_static_entity_types (&mut self);
// }
//
// pub struct EntityTypeManager {
//
//     datastore: &'a MemoryDatastore,
//     component_manager: &'b ComponentManager,
//
//     pub entity_types: Vec<EntityType>,
//
// }
//
// impl EntityTypeManager<'_, '_> {
//
//     pub(crate) fn new<'a, 'b>(datastore: &'a MemoryDatastore, component_manager: &'b ComponentManager) -> EntityTypeManager<'a, 'b> {
//         EntityTypeManager {
//             datastore,
//             component_manager,
//             entity_types: Vec::new(),
//         }
//     }
//
//     pub(crate) fn register (&mut self, entity_type: EntityType) {
//         self.entity_types.push(entity_type);
//         // TODO: create vertex type in datastore
//     }
//
//     pub(crate) fn load_static_entity_types (&mut self) {
//         for file in EntityTypeAsset::iter() {
//             let filename = file.as_ref();
//             println!("Loading entity type from resource {}", filename);
//             let asset = EntityTypeAsset::get(filename).unwrap();
//             let result = std::str::from_utf8(asset.as_ref());
//             if result.is_ok() {
//                 let json_str = result.unwrap();
//                 println!("JSON {}", json_str);
//                 let entity_type: EntityType = serde_json::from_str(json_str).unwrap();
//                 self.register(entity_type);
//             } else {
//                 println!("Could not decode UTF-8 {}", filename)
//             }
//         }
//     }
//
// }
