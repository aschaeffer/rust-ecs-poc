// use rust_embed::RustEmbed;
// use crate::entity_system::model::relation_type::RelationType;
// use crate::entity_system::managers::component_manager::ComponentManager;
// use indradb::MemoryDatastore;
//
// #[derive(RustEmbed)]
// #[folder = "static/types/relation"]
// struct RelationTypeAsset;
//
// pub struct RelationTypeManager<'a, 'b> {
//
//     datastore: &'a MemoryDatastore,
//     component_manager: &'b ComponentManager,
//
//     pub relation_types: Vec<RelationType>
//
// }
//
// impl RelationTypeManager<'_, '_> {
//
//     pub(crate) fn new<'a, 'b>(datastore: &'a MemoryDatastore, component_manager: &'b ComponentManager) -> RelationTypeManager<'a, 'b> {
//         RelationTypeManager {
//             datastore,
//             component_manager,
//             relation_types: Vec::new(),
//         }
//     }
//
//     pub(crate) fn register (&mut self, relation_type: RelationType) {
//         self.relation_types.push(relation_type);
//         // TODO: create edge type in datastore
//     }
//
//     pub(crate) fn load_static_relation_types (&mut self) {
//         for file in RelationTypeAsset::iter() {
//             let filename = file.as_ref();
//             println!("Loading relation type from resource {}", filename);
//             let asset = RelationTypeAsset::get(filename).unwrap();
//             let result = std::str::from_utf8(asset.as_ref());
//             if result.is_ok() {
//                 let json_str = result.unwrap();
//                 println!("JSON {}", json_str);
//                 let relation_type: RelationType = serde_json::from_str(json_str).unwrap();
//                 self.register(relation_type);
//             } else {
//                 println!("Could not decode UTF-8 {}", filename)
//             }
//         }
//     }
//
// }
