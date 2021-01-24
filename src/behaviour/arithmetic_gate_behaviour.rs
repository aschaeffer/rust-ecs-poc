use std::sync::Arc;

use crate::behaviour::{BehaviourCreationError, ReactiveEntityInstanceBehaviour};
use crate::model::ReactiveEntityInstance;
use crate::reactive::arithmetic_gate::{ArithmeticGate, ArithmeticGateFunction, create_arithmetic_gate_entity};

pub trait ArithmeticGateBehaviour<'a>: ReactiveEntityInstanceBehaviour<'a, ArithmeticGate<'static>> {
    const TYPE_NAME_1: &'static str;

    const OPERATION: ArithmeticGateFunction<i64>;
}
impl<'a, T> ReactiveEntityInstanceBehaviour<'a, ArithmeticGate<'static>> for T
    where
        T: ArithmeticGateBehaviour<'a>,
{
    const TYPE_NAME: &'static str = Self::TYPE_NAME_1;

    fn from_entity_instance(e: Arc<ReactiveEntityInstance<'static>>) -> Result<ArithmeticGate<'static>, BehaviourCreationError> {
        if e.type_name != Self::TYPE_NAME_1 {
            return Err(BehaviourCreationError.into())
        }
        Ok(ArithmeticGate::new(e, Self::OPERATION))
    }

    fn new_entity_instance() -> Arc<ReactiveEntityInstance<'static>> {
        Arc::new(create_arithmetic_gate_entity::<i64>(Self::TYPE_NAME.to_string()))
    }

}
