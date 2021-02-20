use std::sync::Arc;
use crate::reactive::entity::numeric_operation::NumericOperation;
use crate::behaviour::{BehaviourCreationError, EntityBehaviour};
use crate::model::ReactiveEntityInstance;
use crate::{entity_behaviour_factory,create_entity_behaviour_factory};

entity_behaviour_factory! { AcosGateFactory, NumericOperation<'static>, crate::behaviour::AcosGate }
entity_behaviour_factory! { AsinGateFactory, NumericOperation<'static>, crate::behaviour::AsinGate }
entity_behaviour_factory! { AtanGateFactory, NumericOperation<'static>, crate::behaviour::AtanGate }
entity_behaviour_factory! { CosGateFactory, NumericOperation<'static>, crate::behaviour::CosGate }
entity_behaviour_factory! { CoshGateFactory, NumericOperation<'static>, crate::behaviour::CoshGate }
entity_behaviour_factory! { SinGateFactory, NumericOperation<'static>, crate::behaviour::SinGate }
entity_behaviour_factory! { SinhGateFactory, NumericOperation<'static>, crate::behaviour::SinhGate }
entity_behaviour_factory! { TanGateFactory, NumericOperation<'static>, crate::behaviour::TanGate }
entity_behaviour_factory! { TanhGateFactory, NumericOperation<'static>, crate::behaviour::TanhGate }

create_entity_behaviour_factory!(create_acos_gate_factory, AcosGateFactory);
create_entity_behaviour_factory!(create_asin_gate_factory, AsinGateFactory);
create_entity_behaviour_factory!(create_atan_gate_factory, AtanGateFactory);
create_entity_behaviour_factory!(create_cos_gate_factory, CosGateFactory);
create_entity_behaviour_factory!(create_cosh_gate_factory, CoshGateFactory);
create_entity_behaviour_factory!(create_sin_gate_factory, SinGateFactory);
create_entity_behaviour_factory!(create_sinh_gate_factory, SinhGateFactory);
create_entity_behaviour_factory!(create_tan_gate_factory, TanGateFactory);
create_entity_behaviour_factory!(create_tanh_gate_factory, TanhGateFactory);
