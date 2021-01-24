pub mod reactive_entity_instance_behaviour;
pub mod numeric_operation_behaviour;
pub mod logical_gates;
pub mod logical_gate_behaviour;
pub mod arithmetic_gates;
pub mod arithmetic_gate_behaviour;
pub mod simple_closures;
pub mod trigonometric_gates;

pub use reactive_entity_instance_behaviour::*;
pub use numeric_operation_behaviour::*;
pub use logical_gates::*;
pub use logical_gate_behaviour::*;
pub use arithmetic_gates::*;
pub use arithmetic_gate_behaviour::*;
pub use simple_closures::*;
pub use trigonometric_gates::*;

#[derive(Debug)]
pub struct BehaviourCreationError;
