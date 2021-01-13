use crate::api::GraphDatabase;
use crate::api::{EntityTypeManager, SystemConstantsInitializer};
use async_trait::async_trait;
use waiter_di::*;

#[component]
pub struct SystemConstantsInitializerImpl {
    graph_database: Wrc<dyn GraphDatabase>,

    entity_type_manager: Wrc<dyn EntityTypeManager>,
}

#[async_trait]
#[provides]
impl SystemConstantsInitializer for SystemConstantsInitializerImpl {
    fn activate(&self) {
        println!("SystemConstantsInitializer::activate")
        // let entity_type = self.entity_type_manager.entity_type_manager.get("value_number");
        // entity_instance_manager.create()
    }

    fn deactivate(&self) {
        println!("SystemConstantsInitializer::deactivate")
    }
}
