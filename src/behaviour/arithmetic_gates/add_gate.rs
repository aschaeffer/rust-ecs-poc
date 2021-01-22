use crate::behaviour::{ReactiveEntityInstanceBehaviour, BehaviourCreationError};
use crate::model::ReactiveEntityInstance;
use crate::reactive::arithmetic_gate::{ArithmeticOperation, ArithmeticGate, create_arithmetic_gate_entity};
use indradb::VertexProperties;
use std::sync::Arc;

pub struct AddGate {}
impl AddGate {
    pub const TYPE_NAME: &'static str = "add";

    pub const OPERATION: ArithmeticOperation = |lhs, rhs| lhs + rhs;
}
impl<'a> ReactiveEntityInstanceBehaviour<'a, ArithmeticGate<'static>> for AddGate {
    fn new() -> Result<ArithmeticGate<'static>, BehaviourCreationError> {
        AddGate::from_entity_instance(AddGate::new_entity_instance())
    }

    fn from_vertex_properties(v: VertexProperties) -> Result<ArithmeticGate<'static>, BehaviourCreationError> {
        if v.vertex.t.0 != AddGate::TYPE_NAME {
            return Err(BehaviourCreationError.into())
        }
        AddGate::from_entity_instance(Arc::new(ReactiveEntityInstance::from(v)))
    }

    fn from_entity_instance(e: Arc<ReactiveEntityInstance<'static>>) -> Result<ArithmeticGate<'static>, BehaviourCreationError> {
        if e.type_name != AddGate::TYPE_NAME {
            return Err(BehaviourCreationError.into())
        }
        Ok(ArithmeticGate::new(e, AddGate::OPERATION))
    }

    fn new_entity_instance() -> Arc<ReactiveEntityInstance<'static>> {
        Arc::new(create_arithmetic_gate_entity(AddGate::TYPE_NAME.to_string()))
    }
}
