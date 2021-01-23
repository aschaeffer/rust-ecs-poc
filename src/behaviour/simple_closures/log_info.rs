use crate::behaviour::{ReactiveEntityInstanceBehaviour, BehaviourCreationError};
use crate::model::ReactiveEntityInstance;
use crate::reactive::simple_closure::{SimpleClosure, create_simple_closure_entity};
use indradb::VertexProperties;
use std::sync::Arc;
use serde_json::Value;
use log::info;

pub struct LogInfo {}
impl LogInfo {
    pub const TYPE_NAME: &'static str = "log_info";
}
impl<'a> ReactiveEntityInstanceBehaviour<'a, SimpleClosure<'static>> for LogInfo {
    fn new() -> Result<SimpleClosure<'static>, BehaviourCreationError> {
        LogInfo::from_entity_instance(LogInfo::new_entity_instance())
    }

    fn from_vertex_properties(v: VertexProperties) -> Result<SimpleClosure<'static>, BehaviourCreationError> {
        if v.vertex.t.0 != LogInfo::TYPE_NAME {
            return Err(BehaviourCreationError.into())
        }
        LogInfo::from_entity_instance(Arc::new(ReactiveEntityInstance::from(v)))
    }

    fn from_entity_instance(e: Arc<ReactiveEntityInstance<'static>>) -> Result<SimpleClosure<'static>, BehaviourCreationError> {
        if e.type_name != LogInfo::TYPE_NAME {
            return Err(BehaviourCreationError.into())
        }
        Ok(SimpleClosure::new(e, move | v: &Value | {
            info!("{}", v.to_string());
        }))
    }

    fn new_entity_instance() -> Arc<ReactiveEntityInstance<'static>> {
        Arc::new(create_simple_closure_entity(LogInfo::TYPE_NAME.to_string()))
    }
}
