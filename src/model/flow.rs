use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::model::{RelationInstance, EntityInstance, ReactiveFlow};
use crate::builder::FlowBuilder;
use std::convert::TryFrom;
use std::sync::Arc;

#[derive(Debug)]
pub struct FlowCreationError;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Flow {
    /// The id of the flow corresponds to the id of the wrapper entity instance
    ///
    /// This means the vector of entity instances must contain an instance with
    /// the id of the flow.
    pub id: Uuid,

    /// The name of the flow.
    #[serde(default = "String::new")]
    pub name: String,

    /// The description of the flow.
    #[serde(default = "String::new")]
    pub description: String,

    /// The entity instances which are contained in this flow.
    ///
    /// It can't have a default because the wrapper entity instance must be
    /// present in the list of entities.
    // TODO: rename to entity_instances
    // TODO: #[serde(alias = "entities")]
    pub entity_instances: Vec<EntityInstance>,

    /// The relation instances which are contained in this flow.
    #[serde(default = "Vec::new")]
    // TODO: rename to relation_instances
    // TODO: #[serde(alias = "relations")]
    pub relation_instances: Vec<RelationInstance>,
}

impl Flow {
    /// Constructs a new flow from the wrapper entity instance.
    pub fn from_instance_with_name(wrapper_entity_instance: EntityInstance, name: String) -> Flow {
        let id = wrapper_entity_instance.id;
        let mut entity_instances: Vec<EntityInstance> = Vec::new();
        entity_instances.push(wrapper_entity_instance);
        Flow {
            id,
            name,
            description: String::from(""),
            entity_instances,
            relation_instances: Vec::new()
        }
    }
}

impl From<EntityInstance> for Flow {
    fn from(entity_instance: EntityInstance) -> Flow {
        let id = entity_instance.id;
        let mut entity_instances: Vec<EntityInstance> = Vec::new();
        entity_instances.push(entity_instance);
        Flow {
            id,
            name: String::from(""),
            description: String::from(""),
            entity_instances,
            relation_instances: Vec::new()
        }
    }
}

impl TryFrom<ReactiveFlow> for Flow {
    type Error = FlowCreationError;

    fn try_from(flow: ReactiveFlow) -> Result<Self, FlowCreationError> {
        let wrapper = flow.get_entity(flow.id);
        if wrapper.is_none() {
            return Err(FlowCreationError.into());
        }
        let wrapper = wrapper.unwrap();
        let mut flow_builder = FlowBuilder::new(wrapper.into());
        flow.entity_instances.read().unwrap().iter().for_each(|(_, entity)| {
            if entity.id != flow.id {
                flow_builder.entity(entity.clone().into());
            }
        });
        flow.relation_instances.read().unwrap().iter().for_each(|(_, relation)| {
            flow_builder.relation(relation.clone().into());
        });
        Ok(flow_builder.get())
    }
}

impl TryFrom<Arc<ReactiveFlow>> for Flow {
    type Error = FlowCreationError;

    fn try_from(flow: Arc<ReactiveFlow>) -> Result<Self, FlowCreationError> {
        let wrapper = flow.get_entity(flow.id);
        if wrapper.is_none() {
            return Err(FlowCreationError.into());
        }
        let wrapper = wrapper.unwrap();
        let mut flow_builder = FlowBuilder::new(wrapper.into());
        flow.entity_instances.read().unwrap().iter().for_each(|(_, entity)| {
            if entity.id != flow.id {
                flow_builder.entity(entity.clone().into());
            }
        });
        flow.relation_instances.read().unwrap().iter().for_each(|(_, relation_instance)| {
            flow_builder.relation(relation_instance.clone().into());
        });
        Ok(flow_builder.get())
    }
}
