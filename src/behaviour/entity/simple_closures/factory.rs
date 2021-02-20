use std::sync::Arc;
use crate::reactive::entity::simple_closure::SimpleClosure;
use crate::behaviour::{BehaviourCreationError, EntityBehaviour};
use crate::model::ReactiveEntityInstance;
use crate::{entity_behaviour_factory,create_entity_behaviour_factory};

entity_behaviour_factory! { LogDebugFactory, SimpleClosure, crate::behaviour::LogDebug }
entity_behaviour_factory! { LogInfoFactory, SimpleClosure, crate::behaviour::LogInfo }

create_entity_behaviour_factory!(create_log_debug_factory, LogDebugFactory);
create_entity_behaviour_factory!(create_log_info_factory, LogInfoFactory);
