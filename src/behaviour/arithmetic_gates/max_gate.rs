use crate::behaviour::{ReactiveEntityInstanceBehaviour, BehaviourCreationError};
use crate::model::ReactiveEntityInstance;
use crate::reactive::arithmetic_gate::{ArithmeticOperation, ArithmeticGate, create_arithmetic_gate_entity};
use indradb::VertexProperties;
use std::sync::Arc;

pub struct MaxGate {}
impl MaxGate {
    pub const TYPE_NAME: &'static str = "max";

    pub const OPERATION: ArithmeticOperation = |lhs, rhs| std::cmp::max(lhs,rhs);
}
impl<'a> ReactiveEntityInstanceBehaviour<'a, ArithmeticGate<'static>> for MaxGate {
    fn new() -> Result<ArithmeticGate<'static>, BehaviourCreationError> {
        MaxGate::from_entity_instance(MaxGate::new_entity_instance())
    }

    fn from_vertex_properties(v: VertexProperties) -> Result<ArithmeticGate<'static>, BehaviourCreationError> {
        if v.vertex.t.0 != MaxGate::TYPE_NAME {
            return Err(BehaviourCreationError.into())
        }
        MaxGate::from_entity_instance(Arc::new(ReactiveEntityInstance::from(v)))
    }

    fn from_entity_instance(e: Arc<ReactiveEntityInstance<'static>>) -> Result<ArithmeticGate<'static>, BehaviourCreationError> {
        if e.type_name != MaxGate::TYPE_NAME {
            return Err(BehaviourCreationError.into())
        }
        Ok(ArithmeticGate::new(e, MaxGate::OPERATION))
    }

    fn new_entity_instance() -> Arc<ReactiveEntityInstance<'static>> {
        Arc::new(create_arithmetic_gate_entity(MaxGate::TYPE_NAME.to_string()))
    }
}
