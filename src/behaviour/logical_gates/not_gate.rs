use crate::behaviour::{ReactiveEntityInstanceBehaviour, BehaviourCreationError};
use crate::model::ReactiveEntityInstance;
use crate::reactive::{LogicalGateFunction, LogicalGate, create_logical_gate_entity};
use std::sync::Arc;

pub struct NotGate {}
impl NotGate {
    // TODO: not optimal: ignores the second input
    pub const OPERATION: LogicalGateFunction = |lhs, _rhs| !lhs;
}
impl<'a> ReactiveEntityInstanceBehaviour<'a, LogicalGate<'static>> for NotGate {
    const TYPE_NAME: &'static str = "xor";

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
