use std::sync::Arc;

use crate::behaviour::{BehaviourCreationError, EntityBehaviour};
use crate::model::{ReactiveEntityInstance, ReactiveEntityInstanceFactory};
use crate::reactive::entity::simple_closure::{SimpleClosure, SimpleClosureFunction, SimpleClosureReactiveEntityInstanceFactory};

pub trait SimpleClosureBehaviour: EntityBehaviour<SimpleClosure> {
    const TYPE_NAME_1: &'static str;

    const OPERATION: SimpleClosureFunction;
}
impl<T> EntityBehaviour<SimpleClosure> for T
    where
        T: SimpleClosureBehaviour,
{
    const TYPE_NAME: &'static str = Self::TYPE_NAME_1;

    fn from_entity_instance(e: Arc<ReactiveEntityInstance>) -> Result<SimpleClosure, BehaviourCreationError> {
        if e.type_name != Self::TYPE_NAME_1 {
            return Err(BehaviourCreationError.into())
        }
        Ok(SimpleClosure::new(e, Box::new(Self::OPERATION)))
    }

    fn new_entity_instance() -> Arc<ReactiveEntityInstance> {
        SimpleClosureReactiveEntityInstanceFactory::new(Self::TYPE_NAME)
    }
}
