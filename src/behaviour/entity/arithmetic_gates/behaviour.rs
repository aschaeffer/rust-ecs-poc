use std::sync::Arc;

use crate::behaviour::{BehaviourCreationError, EntityBehaviour};
use crate::model::{ReactiveEntityInstance, ReactiveEntityInstanceFactory};
use crate::reactive::entity::arithmetic_gate::{ArithmeticGate, ArithmeticGateFunction, ArithmeticGateReactiveEntityInstanceFactory};

pub trait ArithmeticGateBehaviour: EntityBehaviour<ArithmeticGate<'static>> {
    const TYPE_NAME_1: &'static str;

    const OPERATION: ArithmeticGateFunction<i64>;
}
impl<T> EntityBehaviour<ArithmeticGate<'static>> for T
    where
        T: ArithmeticGateBehaviour,
{
    const TYPE_NAME: &'static str = Self::TYPE_NAME_1;

    fn from_entity_instance(e: Arc<ReactiveEntityInstance>) -> Result<ArithmeticGate<'static>, BehaviourCreationError> {
        if e.type_name != Self::TYPE_NAME_1 {
            return Err(BehaviourCreationError.into())
        }
        Ok(ArithmeticGate::new(e, Self::OPERATION))
    }

    fn new_entity_instance() -> Arc<ReactiveEntityInstance> {
        ArithmeticGateReactiveEntityInstanceFactory::new(Self::TYPE_NAME)
        // Arc::new(create_arithmetic_gate_entity::<i64>(Self::TYPE_NAME.to_string()))
    }
}
