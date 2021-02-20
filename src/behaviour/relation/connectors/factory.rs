use std::sync::Arc;
use crate::reactive::relation::connector::Connector;
use crate::behaviour::{BehaviourCreationError, RelationBehaviour};
use crate::model::ReactiveRelationInstance;
use crate::{relation_behaviour_factory,create_relation_behaviour_factory};

// TODO: Connector<'static>
relation_behaviour_factory! { DefaultConnectorFactory, Connector, crate::behaviour::DefaultConnector }
relation_behaviour_factory! { ParseIntConnectorFactory, Connector, crate::behaviour::ParseIntConnector }
relation_behaviour_factory! { ToStringConnectorFactory, Connector, crate::behaviour::ToStringConnector }

create_relation_behaviour_factory!(create_default_connector_factory, DefaultConnectorFactory);
create_relation_behaviour_factory!(create_parse_int_connector_factory, ParseIntConnectorFactory);
create_relation_behaviour_factory!(create_to_string_connector_factory, ToStringConnectorFactory);
