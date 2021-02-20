use async_trait::async_trait;
use log::debug;
use waiter_di::*;

use crate::api::{SystemConstantsInitializer, ReactiveEntityInstanceManager, ReactiveFlowManager, Lifecycle};
use crate::builder::{FlowBuilder, EntityInstanceBuilder, DefaultConnectorBuilder};
use crate::model::Flow;
use crate::reactive::{ConstValue, SimpleClosureProperties, ConstValueProperties};
use std::convert::TryFrom;
use serde_json::json;
use crate::behaviour::{LogInfo, EntityBehaviour};

#[wrapper]
pub struct SystemConstantsFlow(std::sync::RwLock<Option<std::sync::Arc<crate::model::ReactiveFlow>>>);

#[provides]
fn create_external_type_dependency() -> SystemConstantsFlow {
    SystemConstantsFlow(std::sync::RwLock::new(None))
}

#[component]
pub struct SystemConstantsInitializerImpl {
    reactive_entity_instance_manager: Wrc<dyn ReactiveEntityInstanceManager>,
    reactive_flow_manager: Wrc<dyn ReactiveFlowManager>,

    system_constants: SystemConstantsFlow,
}

#[async_trait]
#[provides]
impl SystemConstantsInitializer for SystemConstantsInitializerImpl {
}

impl Lifecycle for SystemConstantsInitializerImpl {
    fn init(&self) {
        debug!("SystemConstantsInitializer::activate");

        let e_num_cpus = EntityInstanceBuilder::new(ConstValue::DEFAULT_TYPE_NAME)
            .property(ConstValueProperties::VALUE.to_string(), json!(num_cpus::get()))
            .get();
        let e_flow = EntityInstanceBuilder::new(ConstValue::DEFAULT_TYPE_NAME)
            .property("num_cpus".to_string(), json!(0))
            .get();
        let e_log_info = EntityInstanceBuilder::new(LogInfo::TYPE_NAME.to_string())
            .property(SimpleClosureProperties::INPUT.to_string(), json!(0))
            .get();
        let r_num_cpus = DefaultConnectorBuilder::new()
            .outbound(e_num_cpus.clone(), ConstValueProperties::VALUE.to_string())
            .inbound(e_flow.clone(), "num_cpus".to_string())
            .get();
        let r_log_num_cpus = DefaultConnectorBuilder::new()
            .outbound(e_num_cpus.clone(), ConstValueProperties::VALUE.to_string())
            .inbound(e_log_info.clone(), SimpleClosureProperties::INPUT.to_string())
            .get();
        let system_constants = FlowBuilder::new(e_flow.clone())
            .entity(e_num_cpus.clone())
            .entity(e_log_info.clone())
            .relation(r_num_cpus.clone())
            .relation(r_log_num_cpus.clone())
            .get();
        let reactive_flow = self.reactive_flow_manager.create(system_constants);
        if reactive_flow.is_ok() {
            let reactive_flow = reactive_flow.unwrap();
            reactive_flow.tick();
            let flow = Flow::try_from(reactive_flow.clone());
            if flow.is_ok() {
                println!("{}", serde_json::to_string_pretty(&flow.unwrap()).unwrap());
            }
            let mut writer = self.system_constants.0.write().unwrap();
            *writer = Some(reactive_flow);
        }
    }

    fn shutdown(&self) {
        debug!("SystemConstantsInitializer::deactivate");
        let mut writer = self.system_constants.0.write().unwrap();
        *writer = None;
    }
}
