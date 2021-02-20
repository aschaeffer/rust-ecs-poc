use std::sync::Arc;
use crate::reactive::entity::arithmetic_gate::ArithmeticGate;
use crate::behaviour::{BehaviourCreationError, EntityBehaviour};
use crate::model::ReactiveEntityInstance;
use crate::{entity_behaviour_factory,create_entity_behaviour_factory};

entity_behaviour_factory! { AddGateFactory, ArithmeticGate<'static>, crate::behaviour::AddGate }
entity_behaviour_factory! { DivGateFactory, ArithmeticGate<'static>, crate::behaviour::DivGate }
entity_behaviour_factory! { MaxGateFactory, ArithmeticGate<'static>, crate::behaviour::MaxGate }
entity_behaviour_factory! { MinGateFactory, ArithmeticGate<'static>, crate::behaviour::MinGate }
entity_behaviour_factory! { ModGateFactory, ArithmeticGate<'static>, crate::behaviour::ModGate }
entity_behaviour_factory! { MulGateFactory, ArithmeticGate<'static>, crate::behaviour::MulGate }
entity_behaviour_factory! { SubGateFactory, ArithmeticGate<'static>, crate::behaviour::SubGate }

create_entity_behaviour_factory!(create_add_gate_factory, AddGateFactory);
create_entity_behaviour_factory!(create_div_gate_factory, DivGateFactory);
create_entity_behaviour_factory!(create_max_gate_factory, MaxGateFactory);
create_entity_behaviour_factory!(create_min_gate_factory, MinGateFactory);
create_entity_behaviour_factory!(create_mod_gate_factory, ModGateFactory);
create_entity_behaviour_factory!(create_mul_gate_factory, MulGateFactory);
create_entity_behaviour_factory!(create_sub_gate_factory, SubGateFactory);
