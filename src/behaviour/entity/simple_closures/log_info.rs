// use crate::behaviour::{EntityBehaviour, BehaviourCreationError};
// use crate::model::ReactiveEntityInstance;
// use crate::reactive::entity::simple_closure::{SimpleClosure, create_simple_closure_entity};
// use std::sync::Arc;
// use serde_json::Value;
// use log::info;
//
// pub struct LogInfo {}
// impl LogInfo {}
// impl EntityBehaviour<SimpleClosure> for LogInfo {
//     const TYPE_NAME: &'static str = "log_info";
//
//     fn from_entity_instance(e: Arc<ReactiveEntityInstance>) -> Result<SimpleClosure, BehaviourCreationError> {
//         if e.type_name != LogInfo::TYPE_NAME {
//             return Err(BehaviourCreationError.into())
//         }
//         Ok(SimpleClosure::new(e, move | v: &Value | {
//             info!("{}", v.to_string());
//         }))
//     }
//
//     fn new_entity_instance() -> Arc<ReactiveEntityInstance> {
//         Arc::new(create_simple_closure_entity(LogInfo::TYPE_NAME.to_string()))
//     }
// }
