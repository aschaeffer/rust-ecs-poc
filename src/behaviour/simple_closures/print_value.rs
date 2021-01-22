use crate::behaviour::{ReactiveEntityInstanceBehaviour, BehaviourCreationError};
use crate::model::ReactiveEntityInstance;
use crate::reactive::simple_closure::{SimpleClosure, create_simple_closure_entity};
use indradb::VertexProperties;
use std::sync::Arc;
use serde_json::Value;

pub struct PrintValue {}
impl PrintValue {
    pub const TYPE_NAME: &'static str = "print";
}
impl<'a> ReactiveEntityInstanceBehaviour<'a, SimpleClosure<'static>> for PrintValue {
    fn new() -> Result<SimpleClosure<'static>, BehaviourCreationError> {
        PrintValue::from_entity_instance(PrintValue::new_entity_instance())
    }

    fn from_vertex_properties(v: VertexProperties) -> Result<SimpleClosure<'static>, BehaviourCreationError> {
        if v.vertex.t.0 != PrintValue::TYPE_NAME {
            return Err(BehaviourCreationError.into())
        }
        PrintValue::from_entity_instance(Arc::new(ReactiveEntityInstance::from(v)))
    }

    fn from_entity_instance(e: Arc<ReactiveEntityInstance<'static>>) -> Result<SimpleClosure<'static>, BehaviourCreationError> {
        if e.type_name != PrintValue::TYPE_NAME {
            return Err(BehaviourCreationError.into())
        }
        Ok(SimpleClosure::new(e, move | v: &Value | {
            println!("{}", v.to_string());
        }))
    }

    fn new_entity_instance() -> Arc<ReactiveEntityInstance<'static>> {
        Arc::new(create_simple_closure_entity(PrintValue::TYPE_NAME.to_string()))
    }
}
