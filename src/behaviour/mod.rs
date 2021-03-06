pub mod entity;
pub mod relation;

pub use entity::*;
pub use relation::*;

#[derive(Debug)]
pub struct BehaviourCreationError;
