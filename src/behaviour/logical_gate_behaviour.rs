use std::sync::Arc;

use crate::behaviour::{BehaviourCreationError, ReactiveEntityInstanceBehaviour};
use crate::model::ReactiveEntityInstance;
use crate::reactive::{create_logical_gate_entity, LogicalGate, LogicalGateFunction};

pub trait LogicalGateBehaviour<'a>: ReactiveEntityInstanceBehaviour<'a, LogicalGate<'static>> {
    const TYPE_NAME_1: &'static str;

    const OPERATION: LogicalGateFunction;
}
impl<'a, T> ReactiveEntityInstanceBehaviour<'a, LogicalGate<'static>> for T
    where
        T: LogicalGateBehaviour<'a>,
{
    const TYPE_NAME: &'static str = Self::TYPE_NAME_1;

    fn from_entity_instance(e: Arc<ReactiveEntityInstance<'static>>) -> Result<LogicalGate<'static>, BehaviourCreationError> {
        if e.type_name != Self::TYPE_NAME_1 {
            return Err(BehaviourCreationError.into())
        }
        Ok(LogicalGate::new(e, Self::OPERATION))
    }

    fn new_entity_instance() -> Arc<ReactiveEntityInstance<'static>> {
        Arc::new(create_logical_gate_entity(Self::TYPE_NAME.to_string()))
    }

}
