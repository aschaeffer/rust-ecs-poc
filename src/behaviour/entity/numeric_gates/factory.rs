use std::sync::Arc;
use crate::reactive::entity::numeric_gate::NumericGate;
use crate::behaviour::{BehaviourCreationError, EntityBehaviour};
use crate::model::ReactiveEntityInstance;
use crate::{entity_behaviour_factory,create_entity_behaviour_factory};

entity_behaviour_factory! { PowGateFactory, NumericGate<'static>, crate::behaviour::PowGate }

create_entity_behaviour_factory!(create_pow_gate_factory, PowGateFactory);
