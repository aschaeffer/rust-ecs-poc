use crate::model::ReactiveEntityInstance;
use indradb::{VertexProperties, Type, NamedProperty, Vertex};
use serde_json::{json, Value};
use std::sync::Arc;
use uuid::Uuid;
use std::str::FromStr;

// TODO: move into: impl SimpleClosure {}
pub static PROPERTY_NAME_INPUT: &'static str = "input";

/// The simple closure is meant to be used as a sink.
///
/// The entity has only one property named "value". The
pub struct SimpleClosure<'a> {
    pub entity: Arc<ReactiveEntityInstance<'a>>,

    pub handle_id: u128,
}

impl SimpleClosure<'_> {
    pub fn new<'a, F: 'static>(e: Arc<ReactiveEntityInstance<'static>>, f: F) -> SimpleClosure<'static>
    where
        F: 'a + FnMut(&Value) -> ()
    {
        let handle_id = e.properties.get(PROPERTY_NAME_INPUT).unwrap().id.as_u128();
        e.properties.get(PROPERTY_NAME_INPUT).unwrap()
            .stream.read().unwrap()
            .observe_with_handle(f, handle_id);
        SimpleClosure {
            entity: e.clone(),
            handle_id
        }
    }

    pub fn disconnect(&self) {
        println!("Disconnect SimpleClosure {}", self.handle_id);
        self.entity.properties.get(PROPERTY_NAME_INPUT).unwrap()
            .stream.read().unwrap().remove(self.handle_id);
    }

    pub fn type_name(&self) -> String {
        self.entity.type_name.clone()
    }
}

/// Automatically disconnect streams on destruction
impl Drop for SimpleClosure<'_> {
    fn drop(&mut self) {
        println!("Drop simple closure");
        self.disconnect();
    }
}

// TODO: replace these static constructor functions with builder patterns
// EntityBuilder::new().type(type_name).property("a").build()
/// Creates a entity with one input and no outputs
pub fn create_simple_closure_entity(type_name: String) -> ReactiveEntityInstance<'static> {
    let uuid = Uuid::new_v4();
    let t = Type::from_str(type_name.as_str()).unwrap();
    let property_input = NamedProperty {
        name: PROPERTY_NAME_INPUT.to_string(),
        value: json!(false)
    };
    let properties = vec![
        property_input
    ];
    let vertex_properties = VertexProperties {
        vertex: Vertex { id: uuid, t: t.clone() },
        props: properties.clone()
    };
    ReactiveEntityInstance::from(vertex_properties)
}

// TODO: pub fn create_simple_closure_entity_with_id(type_name: String, id: String) -> ReactiveEntityInstance<'static> {
