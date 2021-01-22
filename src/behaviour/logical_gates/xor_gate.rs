use crate::behaviour::{ReactiveEntityInstanceBehaviour, BehaviourCreationError};
use crate::model::ReactiveEntityInstance;
use crate::reactive::{BinaryOperation, LogicalGate, create_logical_gate_entity};
use indradb::VertexProperties;
use std::sync::Arc;

pub struct XorGate {}
impl XorGate {
    pub const TYPE_NAME: &'static str = "xor";

    pub const OPERATION: BinaryOperation = |lhs, rhs| lhs ^ rhs;
}
impl<'a> ReactiveEntityInstanceBehaviour<'a, LogicalGate<'static>> for XorGate {
    fn new() -> Result<LogicalGate<'static>, BehaviourCreationError> {
        XorGate::from_entity_instance(XorGate::new_entity_instance())
    }

    fn from_vertex_properties(v: VertexProperties) -> Result<LogicalGate<'static>, BehaviourCreationError> {
        if v.vertex.t.0 != XorGate::TYPE_NAME {
            return Err(BehaviourCreationError.into())
        }
        XorGate::from_entity_instance(Arc::new(ReactiveEntityInstance::from(v)))
    }

    fn from_entity_instance(e: Arc<ReactiveEntityInstance<'static>>) -> Result<LogicalGate<'static>, BehaviourCreationError> {
        if e.type_name != XorGate::TYPE_NAME {
            return Err(BehaviourCreationError.into())
        }
        Ok(LogicalGate::new(e, XorGate::OPERATION))
    }

    fn new_entity_instance() -> Arc<ReactiveEntityInstance<'static>> {
        Arc::new(create_logical_gate_entity(XorGate::TYPE_NAME.to_string()))
    }
}
