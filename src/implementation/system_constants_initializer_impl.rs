use crate::api::GraphDatabase;
use crate::api::{EntityTypeManager, SystemConstantsInitializer};
use async_trait::async_trait;
use waiter_di::*;
use log::debug;

#[component]
pub struct SystemConstantsInitializerImpl {
    graph_database: Wrc<dyn GraphDatabase>,

    entity_type_manager: Wrc<dyn EntityTypeManager>,
}

#[async_trait]
#[provides]
impl SystemConstantsInitializer for SystemConstantsInitializerImpl {
    fn activate(&self) {
        debug!("SystemConstantsInitializer::activate")
        // let entity_type = self.entity_type_manager.entity_type_manager.get("value_number");
        // entity_instance_manager.create()
    }

    fn deactivate(&self) {
        debug!("SystemConstantsInitializer::deactivate")
    }
}
