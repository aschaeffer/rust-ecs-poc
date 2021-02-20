use std::sync::Arc;

use crate::behaviour::{BehaviourCreationError, EntityBehaviour};
use crate::model::{ReactiveEntityInstance, ReactiveEntityInstanceFactory};
use crate::reactive::entity::numeric_gate::{NumericGate, NumericGateFunction, NumericGateReactiveEntityInstanceFactory};

pub trait NumericGateBehaviour: EntityBehaviour<NumericGate<'static>> {
    const TYPE_NAME_1: &'static str;

    const OPERATION: NumericGateFunction<f64>;
}
impl<T> EntityBehaviour<NumericGate<'static>> for T
    where
        T: NumericGateBehaviour,
{
    const TYPE_NAME: &'static str = Self::TYPE_NAME_1;

    fn from_entity_instance(e: Arc<ReactiveEntityInstance>) -> Result<NumericGate<'static>, BehaviourCreationError> {
        if e.type_name != Self::TYPE_NAME_1 {
            return Err(BehaviourCreationError.into())
        }
        Ok(NumericGate::new(e, Self::OPERATION))
    }

    fn new_entity_instance() -> Arc<ReactiveEntityInstance> {
        NumericGateReactiveEntityInstanceFactory::new(Self::TYPE_NAME)
    }
}
