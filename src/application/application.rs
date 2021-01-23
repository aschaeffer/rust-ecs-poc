use crate::api::*;
use async_trait::async_trait;
use std::sync::RwLock;
use std::thread;
use std::time::Duration;
use waiter_di::*;

#[wrapper]
pub struct RunningState(RwLock<bool>);

#[provides]
fn create_external_type_dependency() -> RunningState {
    RunningState(RwLock::new(false))
}

#[async_trait]
pub trait Application: Send + Sync {
    fn init(&self);

    async fn run(&mut self);

    fn stop(&self);
}

#[module]
pub struct ApplicationImpl {
    running: RunningState,

    pub component_manager: Wrc<dyn ComponentManager>,
    pub entity_type_manager: Wrc<dyn EntityTypeManager>,
    pub entity_instance_vertex_manager: Wrc<dyn EntityInstanceVertexManager>,
    pub entity_instance_manager: Wrc<dyn EntityInstanceManager>,
    pub relation_type_manager: Wrc<dyn RelationTypeManager>,
    pub system_constants_initializer: Wrc<dyn SystemConstantsInitializer>,
    pub graph_database: Wrc<dyn GraphDatabase>,
}

#[async_trait]
#[provides]
impl Application for ApplicationImpl {
    fn init(&self) {
        self.component_manager.load_static_components();
        self.entity_type_manager.load_static_entity_types();
        self.relation_type_manager.load_static_relation_types();
        self.system_constants_initializer.activate();
    }

    async fn run(&mut self) {
        {
            let mut running = self.running.0.write().unwrap();
            *running = true;
        }
        let running = self.running.0.read().unwrap();
        while *running {
            thread::sleep(Duration::from_millis(1));
        }
    }

    fn stop(&self) {
        {
            let mut running = self.running.0.write().unwrap();
            *running = false;
        }
    }
}
