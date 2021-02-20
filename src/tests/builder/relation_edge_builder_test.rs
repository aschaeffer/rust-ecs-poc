use crate::builder::RelationEdgeBuilder;
use crate::tests::{r_string, r_json_string};
use uuid::Uuid;
use indradb::{EdgeKey, Type};

#[test]
fn relation_edge_builder_test() {
    let type_name = r_string();
    let outbound_id = Uuid::new_v4();
    let inbound_id = Uuid::new_v4();
    let property_name = r_string();
    let property_value = r_json_string();
    RelationEdgeBuilder::new(outbound_id, type_name.clone(), inbound_id)
        .property(property_name.clone(), property_value.clone());
    let edge_key = EdgeKey::new(outbound_id, Type::new(type_name.clone()).unwrap(), inbound_id);
    RelationEdgeBuilder::from(edge_key)
        .property(property_name.clone(), property_value.clone());
}
