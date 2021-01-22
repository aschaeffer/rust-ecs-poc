pub mod reactive_entity_instance_behaviour;
pub mod logical_gates;
pub mod arithmetic_gates;
pub mod simple_closures;

pub use reactive_entity_instance_behaviour::*;
pub use logical_gates::*;
pub use arithmetic_gates::*;
pub use simple_closures::*;

#[derive(Debug)]
pub struct BehaviourCreationError;
