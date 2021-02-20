use std::sync::Arc;
use crate::behaviour::{BehaviourCreationError, EntityBehaviour};
use crate::model::ReactiveEntityInstance;
use crate::{entity_behaviour_factory,create_entity_behaviour_factory};

entity_behaviour_factory! { AndGateFactory, crate::reactive::LogicalGate<'static>, crate::behaviour::AndGate }
entity_behaviour_factory! { NandGateFactory, crate::reactive::LogicalGate<'static>, crate::behaviour::NandGate }
entity_behaviour_factory! { NorGateFactory, crate::reactive::LogicalGate<'static>, crate::behaviour::NorGate }
entity_behaviour_factory! { NotGateFactory, crate::reactive::LogicalGate<'static>, crate::behaviour::NotGate }
entity_behaviour_factory! { OrGateFactory, crate::reactive::LogicalGate<'static>, crate::behaviour::OrGate }
entity_behaviour_factory! { XorGateFactory, crate::reactive::LogicalGate<'static>, crate::behaviour::XorGate }
entity_behaviour_factory! { XnorGateFactory, crate::reactive::LogicalGate<'static>, crate::behaviour::XnorGate }

create_entity_behaviour_factory!(create_and_gate_factory, AndGateFactory);
create_entity_behaviour_factory!(create_nand_gate_factory, NandGateFactory);
create_entity_behaviour_factory!(create_nor_gate_factory, NorGateFactory);
create_entity_behaviour_factory!(create_not_gate_factory, NotGateFactory);
create_entity_behaviour_factory!(create_or_gate_factory, OrGateFactory);
create_entity_behaviour_factory!(create_xor_gate_factory, XorGateFactory);
create_entity_behaviour_factory!(create_xnor_gate_factory, XnorGateFactory);
