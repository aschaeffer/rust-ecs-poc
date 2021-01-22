use crate::model::ReactiveEntityInstance;
use crate::behaviour::BehaviourCreationError;
use indradb::VertexProperties;
use std::sync::Arc;

pub trait ReactiveEntityInstanceBehaviour<'a, T> {
    fn new() -> Result<T, BehaviourCreationError>;

    // TODO: change return type to Result<T, Err>
    fn from_vertex_properties(v: VertexProperties) -> Result<T, BehaviourCreationError>;

    // TODO: change return type to Result<T, Err>
    fn from_entity_instance(e: Arc<ReactiveEntityInstance<'static>>) -> Result<T, BehaviourCreationError>;

    fn new_entity_instance() -> Arc<ReactiveEntityInstance<'static>>;
}
