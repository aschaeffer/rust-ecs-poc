use crate::behaviour::{ReactiveEntityInstanceBehaviour, BehaviourCreationError};
use crate::model::ReactiveEntityInstance;
use crate::reactive::{BinaryOperation, LogicalGate, create_logical_gate_entity};
use indradb::VertexProperties;
use std::sync::Arc;

pub struct NorGate {}
impl NorGate {
    pub const TYPE_NAME: &'static str = "nor";

    pub const OPERATION: BinaryOperation = |lhs, rhs| !(lhs || rhs);
}
impl<'a> ReactiveEntityInstanceBehaviour<'a, LogicalGate<'static>> for NorGate {
    fn new() -> Result<LogicalGate<'static>, BehaviourCreationError> {
        NorGate::from_entity_instance(NorGate::new_entity_instance())
    }

    fn from_vertex_properties(v: VertexProperties) -> Result<LogicalGate<'static>, BehaviourCreationError> {
        if v.vertex.t.0 != NorGate::TYPE_NAME {
            return Err(BehaviourCreationError.into())
        }
        NorGate::from_entity_instance(Arc::new(ReactiveEntityInstance::from(v)))
    }

    fn from_entity_instance(e: Arc<ReactiveEntityInstance<'static>>) -> Result<LogicalGate<'static>, BehaviourCreationError> {
        if e.type_name != NorGate::TYPE_NAME {
            return Err(BehaviourCreationError.into())
        }
        Ok(LogicalGate::new(e, NorGate::OPERATION))
    }

    fn new_entity_instance() -> Arc<ReactiveEntityInstance<'static>> {
        Arc::new(create_logical_gate_entity(NorGate::TYPE_NAME.to_string()))
    }
}
