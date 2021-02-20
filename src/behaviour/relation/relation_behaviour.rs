use crate::model::{ReactiveEntityInstance, ReactiveRelationInstance};
use crate::behaviour::BehaviourCreationError;
use indradb::EdgeProperties;
use std::sync::Arc;

pub trait RelationBehaviour<T> {
    const TYPE_NAME: &'static str;

    fn new(outbound: Arc<ReactiveEntityInstance>, inbound: Arc<ReactiveEntityInstance>)
        -> Result<T, BehaviourCreationError>
    {
        Self::from_relation_instance(Self::new_relation_instance(outbound, inbound))
    }

    fn from_edge_properties(outbound: Arc<ReactiveEntityInstance>, inbound: Arc<ReactiveEntityInstance>, v: EdgeProperties)
        -> Result<T, BehaviourCreationError> {

// TODO: remove this? RelationType name != Behaviour name
//         if v.edge.key.t.0 != Self::TYPE_NAME {
//             return Err(BehaviourCreationError.into())
//         }

        Self::from_relation_instance(Arc::new(ReactiveRelationInstance::from(outbound, inbound, v)))
    }

    fn from_relation_instance(e: Arc<ReactiveRelationInstance>) -> Result<T, BehaviourCreationError>;

    fn new_relation_instance(outbound: Arc<ReactiveEntityInstance>, inbound: Arc<ReactiveEntityInstance>)
        -> Arc<ReactiveRelationInstance>;
}

#[macro_export]
macro_rules! relation_behaviour {
    ($behaviour_name:ident, $behaviour_type:ty, $behaviour_operation_type:ty, $relation_type_name:expr, $f:expr) => {
        pub struct $behaviour_name {}
        impl $behaviour_name {}
        impl $behaviour_type for $behaviour_name {
            const TYPE_NAME_1: &'static str = $relation_type_name;
            const OPERATION: $behaviour_operation_type = $f;
        }
    }
}
