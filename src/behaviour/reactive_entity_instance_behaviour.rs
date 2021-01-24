use crate::model::ReactiveEntityInstance;
use crate::behaviour::BehaviourCreationError;
use indradb::VertexProperties;
use std::sync::Arc;

pub trait ReactiveEntityInstanceBehaviour<'a, T> {
    const TYPE_NAME: &'static str;

    fn new() -> Result<T, BehaviourCreationError> {
        Self::from_entity_instance(Self::new_entity_instance())
    }

    fn from_vertex_properties(v: VertexProperties) -> Result<T, BehaviourCreationError> {
        if v.vertex.t.0 != Self::TYPE_NAME {
            return Err(BehaviourCreationError.into())
        }
        Self::from_entity_instance(Arc::new(ReactiveEntityInstance::from(v)))
    }

    fn from_entity_instance(e: Arc<ReactiveEntityInstance<'static>>) -> Result<T, BehaviourCreationError>;

    fn new_entity_instance() -> Arc<ReactiveEntityInstance<'static>>;
}
