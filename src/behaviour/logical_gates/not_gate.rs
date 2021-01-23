use crate::behaviour::{ReactiveEntityInstanceBehaviour, BehaviourCreationError};
use crate::model::ReactiveEntityInstance;
use crate::reactive::{BinaryOperation, LogicalGate, create_logical_gate_entity};
use indradb::VertexProperties;
use std::sync::Arc;

pub struct NotGate {}
impl NotGate {
    pub const TYPE_NAME: &'static str = "xor";

    // TODO: not optimal: ignores the second input
    pub const OPERATION: BinaryOperation = |lhs, _rhs| !lhs;
}
impl<'a> ReactiveEntityInstanceBehaviour<'a, LogicalGate<'static>> for NotGate {
    fn new() -> Result<LogicalGate<'static>, BehaviourCreationError> {
        NotGate::from_entity_instance(NotGate::new_entity_instance())
    }

    fn from_vertex_properties(v: VertexProperties) -> Result<LogicalGate<'static>, BehaviourCreationError> {
        if v.vertex.t.0 != NotGate::TYPE_NAME {
            return Err(BehaviourCreationError.into())
        }
        NotGate::from_entity_instance(Arc::new(ReactiveEntityInstance::from(v)))
    }

    fn from_entity_instance(e: Arc<ReactiveEntityInstance<'static>>) -> Result<LogicalGate<'static>, BehaviourCreationError> {
        if e.type_name != NotGate::TYPE_NAME {
            return Err(BehaviourCreationError.into())
        }
        Ok(LogicalGate::new(e, NotGate::OPERATION))
    }

    fn new_entity_instance() -> Arc<ReactiveEntityInstance<'static>> {
        Arc::new(create_logical_gate_entity(NotGate::TYPE_NAME.to_string()))
    }
}
