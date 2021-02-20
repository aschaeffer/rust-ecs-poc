use std::sync::Arc;

use crate::behaviour::{BehaviourCreationError, EntityBehaviour};
use crate::model::{ReactiveEntityInstance, ReactiveEntityInstanceFactory};
use crate::reactive::entity::numeric_operation::{NumericOperation, NumericOperationFunction, NumericOperationReactiveEntityInstanceFactory};

pub trait NumericOperationBehaviour: EntityBehaviour<NumericOperation<'static>> {
    const TYPE_NAME_1: &'static str;

    const OPERATION: NumericOperationFunction<f64>;
}
impl<T> EntityBehaviour<NumericOperation<'static>> for T
    where
        T: NumericOperationBehaviour,
{
    const TYPE_NAME: &'static str = Self::TYPE_NAME_1;

    fn from_entity_instance(e: Arc<ReactiveEntityInstance>) -> Result<NumericOperation<'static>, BehaviourCreationError> {
        if e.type_name != Self::TYPE_NAME_1 {
            return Err(BehaviourCreationError.into())
        }
        Ok(NumericOperation::new(e, Self::OPERATION))
    }

    fn new_entity_instance() -> Arc<ReactiveEntityInstance> {
        NumericOperationReactiveEntityInstanceFactory::new(Self::TYPE_NAME)
    }

}
