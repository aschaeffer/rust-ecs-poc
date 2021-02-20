use crate::api::*;
use async_trait::async_trait;
use std::sync::{RwLock, Arc};
use std::thread;
use std::time::Duration;
use waiter_di::*;
use std::ops::Deref;

#[wrapper]
pub struct RunningState(RwLock<bool>);

#[provides]
fn create_external_type_dependency() -> RunningState {
    RunningState(RwLock::new(false))
}

#[async_trait]
pub trait Application: Send + Sync { //  + Lifecycle
    fn init(&self);

    fn shutdown(&self);

    async fn run(&mut self);

    fn stop(&self);

    fn is_running(&self) -> bool;

    fn get_graph_database(&self) -> Arc<dyn GraphDatabase>;

    fn get_component_manager(&self) -> Arc<dyn ComponentManager>;

    fn get_entity_type_manager(&self) -> Arc<dyn EntityTypeManager>;

    fn get_relation_type_manager(&self) -> Arc<dyn RelationTypeManager>;

    fn get_entity_vertex_manager(&self) -> Arc<dyn EntityVertexManager>;

    fn get_relation_edge_manager(&self) -> Arc<dyn RelationEdgeManager>;

    fn get_entity_instance_manager(&self) -> Arc<dyn EntityInstanceManager>;

    fn get_relation_instance_manager(&self) -> Arc<dyn RelationInstanceManager>;

    fn get_flow_manager(&self) -> Arc<dyn FlowManager>;

    fn get_reactive_entity_instance_manager(&self) -> Arc<dyn ReactiveEntityInstanceManager>;

    fn get_reactive_relation_instance_manager(&self) -> Arc<dyn ReactiveRelationInstanceManager>;

    fn get_reactive_flow_manager(&self) -> Arc<dyn ReactiveFlowManager>;

    fn get_entity_behaviour_manager(&self) -> Arc<dyn EntityBehaviourManager>;
}

#[module]
pub struct ApplicationImpl {
    running: RunningState,

    graph_database: Wrc<dyn GraphDatabase>,
    component_manager: Wrc<dyn ComponentManager>,
    entity_behaviour_manager: Wrc<dyn EntityBehaviourManager>,
    entity_instance_manager: Wrc<dyn EntityInstanceManager>,
    entity_type_manager: Wrc<dyn EntityTypeManager>,
    entity_vertex_manager: Wrc<dyn EntityVertexManager>,
    flow_manager: Wrc<dyn FlowManager>,
    reactive_entity_instance_manager: Wrc<dyn ReactiveEntityInstanceManager>,
    reactive_relation_instance_manager: Wrc<dyn ReactiveRelationInstanceManager>,
    reactive_flow_manager: Wrc<dyn ReactiveFlowManager>,
    relation_edge_manager: Wrc<dyn RelationEdgeManager>,
    relation_instance_manager: Wrc<dyn RelationInstanceManager>,
    relation_type_manager: Wrc<dyn RelationTypeManager>,

    system_constants_initializer: Wrc<dyn SystemConstantsInitializer>,
}

#[async_trait]
#[provides]
impl Application for ApplicationImpl {
    fn init(&self) {
        self.component_manager.init();
        self.entity_type_manager.init();
        self.relation_type_manager.init();
        self.system_constants_initializer.init();
    }

    fn shutdown(&self) {
        self.system_constants_initializer.shutdown();
        self.relation_type_manager.shutdown();
        self.entity_type_manager.shutdown();
        self.component_manager.shutdown();
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

    fn is_running(&self) -> bool {
        *self.running.0.read().unwrap().deref()
    }

    fn get_graph_database(&self) -> Arc<dyn GraphDatabase> {
        self.graph_database.clone()
    }

    fn get_component_manager(&self) -> Arc<dyn ComponentManager> {
        self.component_manager.clone()
    }

    fn get_entity_type_manager(&self) -> Arc<dyn EntityTypeManager> {
        self.entity_type_manager.clone()
    }

    fn get_relation_type_manager(&self) -> Arc<dyn RelationTypeManager> {
        self.relation_type_manager.clone()
    }

    fn get_entity_vertex_manager(&self) -> Arc<dyn EntityVertexManager> {
        self.entity_vertex_manager.clone()
    }

    fn get_relation_edge_manager(&self) -> Arc<dyn RelationEdgeManager> {
        self.relation_edge_manager.clone()
    }

    fn get_entity_instance_manager(&self) -> Arc<dyn EntityInstanceManager> {
        self.entity_instance_manager.clone()
    }

    fn get_relation_instance_manager(&self) -> Arc<dyn RelationInstanceManager> {
        self.relation_instance_manager.clone()
    }

    fn get_flow_manager(&self) -> Arc<dyn FlowManager> {
        self.flow_manager.clone()
    }

    fn get_reactive_entity_instance_manager(&self) -> Arc<dyn ReactiveEntityInstanceManager> {
        self.reactive_entity_instance_manager.clone()
    }

    fn get_reactive_relation_instance_manager(&self) -> Arc<dyn ReactiveRelationInstanceManager> {
        self.reactive_relation_instance_manager.clone()
    }

    fn get_reactive_flow_manager(&self) -> Arc<dyn ReactiveFlowManager> {
        self.reactive_flow_manager.clone()
    }

    fn get_entity_behaviour_manager(&self) -> Arc<dyn EntityBehaviourManager> {
        self.entity_behaviour_manager.clone()
    }
}

// #[provides]
// impl Lifecycle for ApplicationImpl {
// }
