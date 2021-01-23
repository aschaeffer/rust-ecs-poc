use crate::behaviour::{ReactiveEntityInstanceBehaviour, BehaviourCreationError};
use crate::model::ReactiveEntityInstance;
use crate::reactive::arithmetic_gate::{ArithmeticOperation, ArithmeticGate, create_arithmetic_gate_entity};
use indradb::VertexProperties;
use std::sync::Arc;

pub struct MinGate {}
impl MinGate {
    pub const TYPE_NAME: &'static str = "min";

    pub const OPERATION: ArithmeticOperation = |lhs, rhs| std::cmp::min(lhs,rhs);
}
impl<'a> ReactiveEntityInstanceBehaviour<'a, ArithmeticGate<'static>> for MinGate {
    fn new() -> Result<ArithmeticGate<'static>, BehaviourCreationError> {
        MinGate::from_entity_instance(MinGate::new_entity_instance())
    }

    fn from_vertex_properties(v: VertexProperties) -> Result<ArithmeticGate<'static>, BehaviourCreationError> {
        if v.vertex.t.0 != MinGate::TYPE_NAME {
            return Err(BehaviourCreationError.into())
        }
        MinGate::from_entity_instance(Arc::new(ReactiveEntityInstance::from(v)))
    }

    fn from_entity_instance(e: Arc<ReactiveEntityInstance<'static>>) -> Result<ArithmeticGate<'static>, BehaviourCreationError> {
        if e.type_name != MinGate::TYPE_NAME {
            return Err(BehaviourCreationError.into())
        }
        Ok(ArithmeticGate::new(e, MinGate::OPERATION))
    }

    fn new_entity_instance() -> Arc<ReactiveEntityInstance<'static>> {
        Arc::new(create_arithmetic_gate_entity(MinGate::TYPE_NAME.to_string()))
    }
}
