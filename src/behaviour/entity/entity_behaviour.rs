use crate::model::ReactiveEntityInstance;
use crate::behaviour::BehaviourCreationError;
use indradb::VertexProperties;
use std::sync::Arc;

pub trait EntityBehaviour<T> {
    const TYPE_NAME: &'static str;

    fn new() -> Result<T, BehaviourCreationError> {
        Self::from_entity_instance(Self::new_entity_instance())
    }

    fn from_vertex_properties(v: VertexProperties) -> Result<T, BehaviourCreationError> {

// TODO: How to handle type checks? Do we require on the type(s) or behaviour(s) // remove this? EntityType name != Behaviour name
//         if v.vertex.t.0 != Self::TYPE_NAME {
//             return Err(BehaviourCreationError.into())
//         }

        Self::from_entity_instance(Arc::new(ReactiveEntityInstance::from(v)))
    }

    fn from_entity_instance(e: Arc<ReactiveEntityInstance>) -> Result<T, BehaviourCreationError>;

    fn new_entity_instance() -> Arc<ReactiveEntityInstance>;
}

#[macro_export]
macro_rules! entity_behaviour {
    ($behaviour_name:ident, $behaviour_type:ty, $behaviour_operation_type:ty, $entity_type_name:expr, $f:expr) => {
        pub struct $behaviour_name {}
        impl $behaviour_name {}
        impl $behaviour_type for $behaviour_name {
            const TYPE_NAME_1: &'static str = $entity_type_name;
            const OPERATION: $behaviour_operation_type = $f;
        }
    }
}
