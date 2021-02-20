// use crate::behaviour::{EntityBehaviour, BehaviourCreationError};
// use crate::model::ReactiveEntityInstance;
// use crate::reactive::entity::simple_closure::{SimpleClosure, create_simple_closure_entity};
// use std::sync::Arc;
// use serde_json::Value;
//
// pub struct PrintValue {}
// impl PrintValue {}
// impl EntityBehaviour<SimpleClosure> for PrintValue {
//     const TYPE_NAME: &'static str = "print";
//
//     fn from_entity_instance(e: Arc<ReactiveEntityInstance>) -> Result<SimpleClosure, BehaviourCreationError> {
//         if e.type_name != PrintValue::TYPE_NAME {
//             return Err(BehaviourCreationError.into())
//         }
//         Ok(SimpleClosure::new(e, move | v: &Value | {
//             println!("{}", v.to_string();
//         })))
//     }
//
//     fn new_entity_instance() -> Arc<ReactiveEntityInstance> {
//         Arc::new(create_simple_closure_entity(PrintValue::TYPE_NAME.to_string()))
//     }
// }
