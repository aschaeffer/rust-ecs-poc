use std::sync::Arc;

use crate::behaviour::{BehaviourCreationError, ReactiveEntityInstanceBehaviour};
use crate::model::ReactiveEntityInstance;
use crate::reactive::{create_numeric_operation_entity, NumericOperation, NumericOperationFunction};

pub trait NumericOperationBehaviour<'a>: ReactiveEntityInstanceBehaviour<'a, NumericOperation<'static>> {
    const TYPE_NAME_1: &'static str;

    const OPERATION: NumericOperationFunction;
}
impl<'a, T> ReactiveEntityInstanceBehaviour<'a, NumericOperation<'static>> for T
    where
        T: NumericOperationBehaviour<'a>,
{
    const TYPE_NAME: &'static str = Self::TYPE_NAME_1;

    fn from_entity_instance(e: Arc<ReactiveEntityInstance<'static>>) -> Result<NumericOperation<'static>, BehaviourCreationError> {
        if e.type_name != Self::TYPE_NAME_1 {
            return Err(BehaviourCreationError.into())
        }
        Ok(NumericOperation::new(e, Self::OPERATION))
    }

    fn new_entity_instance() -> Arc<ReactiveEntityInstance<'static>> {
        Arc::new(create_numeric_operation_entity(Self::TYPE_NAME.to_string()))
    }

}
