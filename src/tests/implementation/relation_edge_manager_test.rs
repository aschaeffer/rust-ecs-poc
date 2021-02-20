use serde_json::json;

use crate::builder::{RelationTypeBuilder, RelationEdgeBuilder, EntityTypeBuilder, EntityVertexBuilder};
use crate::tests::init_application;
use crate::tests::utils::r_string;
use indradb::{EdgeKey, Type};

#[test]
fn test_relation_edge_manager() {
    let application = init_application();
    let entity_type_manager = application.get_entity_type_manager();
    let relation_type_manager = application.get_relation_type_manager();
    let entity_vertex_manager = application.get_entity_vertex_manager();
    let relation_edge_manager = application.get_relation_edge_manager();

    let outbound_type_name = r_string();
    let relation_type_name = r_string();
    let inbound_type_name = r_string();
    let property_name = r_string();
    let property_value = json!(r_string());

    // Create entity types
    EntityTypeBuilder::new(outbound_type_name.clone()).register(entity_type_manager.clone());
    EntityTypeBuilder::new(inbound_type_name.clone()).register(entity_type_manager.clone());

    // Create relation type
    RelationTypeBuilder::new(outbound_type_name.clone(), relation_type_name.clone(), inbound_type_name.clone())
        .component(String::from("positionable"))
        .string_property(property_name.clone())
        .register(relation_type_manager.clone());

    let outbound_entity = EntityVertexBuilder::new(outbound_type_name.clone()).create(entity_vertex_manager.clone());
    let outbound_id = outbound_entity.unwrap();
    let inbound_entity = EntityVertexBuilder::new(inbound_type_name.clone()).create(entity_vertex_manager.clone());
    let inbound_id = inbound_entity.unwrap();

    let edge_key = EdgeKey::new(outbound_id, Type::new(relation_type_name.clone()).unwrap(), inbound_id);


    let result = RelationEdgeBuilder::new(
        outbound_id,
        relation_type_name.clone(),
        inbound_id
    )
        .property(property_name.clone(), property_value.clone())
        .create(relation_edge_manager.clone());
    assert!(result.is_ok());

    // Check if the relation exists by edge key
    let actual_edge_key = result.unwrap();
    assert_eq!(edge_key, actual_edge_key);
    assert!(relation_edge_manager.has(edge_key.clone()));

    let edge = relation_edge_manager.get(edge_key.clone());
    assert!(edge.is_some());
    assert_eq!(edge_key, edge.unwrap().key);

    let properties = relation_edge_manager.get_properties(edge_key.clone());
    assert!(properties.is_some());
    let properties = properties.unwrap();
    assert_eq!(edge_key, properties.edge.key);
    assert_eq!(relation_type_name.clone(), properties.edge.key.t.0);
    assert_eq!(1, properties.props.len());
    let property = properties.props.get(0);
    assert!(property.is_some());
    let property = property.unwrap();
    assert_eq!(property_name.clone(), property.name);
    assert_eq!(property_value.clone(), property.value);

    // Delete edge
    relation_edge_manager.delete(edge_key.clone());
    // Check if edge is gone
    assert!(!relation_edge_manager.has(edge_key.clone()));
    let edge = relation_edge_manager.get(edge_key.clone());
    assert!(!edge.is_some());
    let properties = relation_edge_manager.get_properties(edge_key.clone());
    assert!(!properties.is_some());
}
