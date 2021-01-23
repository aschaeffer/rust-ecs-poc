use crate::behaviour::{ReactiveEntityInstanceBehaviour, BehaviourCreationError};
use crate::model::ReactiveEntityInstance;
use crate::reactive::arithmetic_gate::{ArithmeticOperation, ArithmeticGate, create_arithmetic_gate_entity};
use indradb::VertexProperties;
use std::sync::Arc;

pub struct DivGate {}
impl DivGate {
    pub const TYPE_NAME: &'static str = "div";

    pub const OPERATION: ArithmeticOperation = |lhs, rhs| if rhs != 0 { lhs / rhs } else { 0 };
}
impl<'a> ReactiveEntityInstanceBehaviour<'a, ArithmeticGate<'static>> for DivGate {
    fn new() -> Result<ArithmeticGate<'static>, BehaviourCreationError> {
        DivGate::from_entity_instance(DivGate::new_entity_instance())
    }

    fn from_vertex_properties(v: VertexProperties) -> Result<ArithmeticGate<'static>, BehaviourCreationError> {
        if v.vertex.t.0 != DivGate::TYPE_NAME {
            return Err(BehaviourCreationError.into())
        }
        DivGate::from_entity_instance(Arc::new(ReactiveEntityInstance::from(v)))
    }

    fn from_entity_instance(e: Arc<ReactiveEntityInstance<'static>>) -> Result<ArithmeticGate<'static>, BehaviourCreationError> {
        if e.type_name != DivGate::TYPE_NAME {
            return Err(BehaviourCreationError.into())
        }
        Ok(ArithmeticGate::new(e, DivGate::OPERATION))
    }

    fn new_entity_instance() -> Arc<ReactiveEntityInstance<'static>> {
        Arc::new(create_arithmetic_gate_entity(DivGate::TYPE_NAME.to_string()))
    }
}
