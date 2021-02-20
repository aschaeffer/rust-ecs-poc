use std::sync::Arc;

use crate::behaviour::{BehaviourCreationError, EntityBehaviour};
use crate::model::{ReactiveEntityInstance, ReactiveEntityInstanceFactory};
use crate::reactive::{LogicalGate, LogicalGateFunction, LogicalGateReactiveEntityInstanceFactory};

pub trait LogicalGateBehaviour: EntityBehaviour<LogicalGate<'static>> {
    const TYPE_NAME_1: &'static str;

    const OPERATION: LogicalGateFunction;
}
// TODO: Factory Type: impl<T, F: ReactiveEntityInstanceFactory>
impl<T> EntityBehaviour<LogicalGate<'static>> for T
    where
        T: LogicalGateBehaviour,
{
    const TYPE_NAME: &'static str = Self::TYPE_NAME_1;

    fn from_entity_instance(e: Arc<ReactiveEntityInstance>) -> Result<LogicalGate<'static>, BehaviourCreationError> {
        if e.type_name != Self::TYPE_NAME_1 {
            return Err(BehaviourCreationError.into())
        }
        Ok(LogicalGate::new(e, Self::OPERATION))
    }

    fn new_entity_instance() -> Arc<ReactiveEntityInstance> {
        LogicalGateReactiveEntityInstanceFactory::new(Self::TYPE_NAME)
        // Arc::new(create_logical_gate_entity(Self::TYPE_NAME.to_string()))
    }

}
