use crate::behaviour::{ReactiveEntityInstanceBehaviour, BehaviourCreationError};
use crate::model::ReactiveEntityInstance;
use crate::reactive::arithmetic_gate::{ArithmeticOperation, ArithmeticGate, create_arithmetic_gate_entity};
use indradb::VertexProperties;
use std::sync::Arc;

pub struct SubGate {}
impl SubGate {
    pub const TYPE_NAME: &'static str = "sub";

    pub const OPERATION: ArithmeticOperation = |lhs, rhs| lhs - rhs;
}
impl<'a> ReactiveEntityInstanceBehaviour<'a, ArithmeticGate<'static>> for SubGate {
    fn new() -> Result<ArithmeticGate<'static>, BehaviourCreationError> {
        SubGate::from_entity_instance(SubGate::new_entity_instance())
    }

    fn from_vertex_properties(v: VertexProperties) -> Result<ArithmeticGate<'static>, BehaviourCreationError> {
        if v.vertex.t.0 != SubGate::TYPE_NAME {
            return Err(BehaviourCreationError.into())
        }
        SubGate::from_entity_instance(Arc::new(ReactiveEntityInstance::from(v)))
    }

    fn from_entity_instance(e: Arc<ReactiveEntityInstance<'static>>) -> Result<ArithmeticGate<'static>, BehaviourCreationError> {
        if e.type_name != SubGate::TYPE_NAME {
            return Err(BehaviourCreationError.into())
        }
        Ok(ArithmeticGate::new(e, SubGate::OPERATION))
    }

    fn new_entity_instance() -> Arc<ReactiveEntityInstance<'static>> {
        Arc::new(create_arithmetic_gate_entity(SubGate::TYPE_NAME.to_string()))
    }
}
