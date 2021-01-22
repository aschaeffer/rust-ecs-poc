use crate::behaviour::{ReactiveEntityInstanceBehaviour, BehaviourCreationError};
use crate::model::ReactiveEntityInstance;
use crate::reactive::logical_gate::{BinaryOperation, LogicalGate, create_logical_gate_entity};
use indradb::VertexProperties;
use std::sync::Arc;

pub struct AndGate {}
impl AndGate {
    pub const TYPE_NAME: &'static str = "and";

    pub const OPERATION: BinaryOperation = |lhs, rhs| lhs && rhs;
}
impl<'a> ReactiveEntityInstanceBehaviour<'a, LogicalGate<'static>> for AndGate {
    fn new() -> Result<LogicalGate<'static>, BehaviourCreationError> {
        AndGate::from_entity_instance(AndGate::new_entity_instance())
    }

    fn from_vertex_properties(v: VertexProperties) -> Result<LogicalGate<'static>, BehaviourCreationError> {
        if v.vertex.t.0 != AndGate::TYPE_NAME {
            return Err(BehaviourCreationError.into())
        }
        AndGate::from_entity_instance(Arc::new(ReactiveEntityInstance::from(v)))
    }

    fn from_entity_instance(e: Arc<ReactiveEntityInstance<'static>>) -> Result<LogicalGate<'static>, BehaviourCreationError> {
        if e.type_name != AndGate::TYPE_NAME {
            return Err(BehaviourCreationError.into())
        }
        Ok(LogicalGate::new(e, AndGate::OPERATION))
    }

    fn new_entity_instance() -> Arc<ReactiveEntityInstance<'static>> {
        Arc::new(create_logical_gate_entity(AndGate::TYPE_NAME.to_string()))
    }
}
