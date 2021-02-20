use std::sync::Arc;

use crate::behaviour::BehaviourCreationError;
use crate::behaviour::relation::RelationBehaviour;
use crate::model::{ReactiveRelationInstance, ReactiveEntityInstance, ReactiveRelationInstanceFactory};
use crate::reactive::relation::connector::{Connector, ConnectorFunction};
use crate::reactive::relation::connector::ConnectorReactiveRelationInstanceFactory;

pub trait ConnectorBehaviour: RelationBehaviour<Connector> {
    const TYPE_NAME_1: &'static str;

    const OPERATION: ConnectorFunction;
}
impl<T> RelationBehaviour<Connector> for T
    where
        T: ConnectorBehaviour,
{
    const TYPE_NAME: &'static str = Self::TYPE_NAME_1;

    fn from_relation_instance(r: Arc<ReactiveRelationInstance>) -> Result<Connector, BehaviourCreationError> {
        if !r.type_name.starts_with(Self::TYPE_NAME_1) {
            // TODO: check relation has properties: outbound_property_name, inbound_property_name
            // TODO: check type_name contains outbound_property_name and inbound_property_name
            // println!("Can't create connector from relation instance: Relation instance type {} does not match connector type {}", r.type_name, Self::TYPE_NAME_1);
            return Err(BehaviourCreationError.into())
        }
        Ok(Connector::from_relation(r, Self::OPERATION))
    }

    fn new_relation_instance(
        outbound: Arc<ReactiveEntityInstance>,
        inbound: Arc<ReactiveEntityInstance>
    ) -> Arc<ReactiveRelationInstance> {
        ConnectorReactiveRelationInstanceFactory::new(
            outbound.clone(),
            Self::TYPE_NAME,
            inbound.clone()
        )
        // Arc::new(create_connector_relation(outbound.clone(), inbound.clone(), Self::TYPE_NAME.to_string()))
    }
}
