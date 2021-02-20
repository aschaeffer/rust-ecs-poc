use crate::entity_behaviour;
use log::{debug, info, warn, error};
use serde_json::Value;

macro_rules! simple_closure {
    ($behaviour_name:ident, $entity_type_name:expr, $f:expr) => {
        entity_behaviour! {
            $behaviour_name,
            crate::behaviour::SimpleClosureBehaviour,
            crate::reactive::SimpleClosureFunction,
            $entity_type_name,
            $f
        }
    }
}

simple_closure! { LogDebug, "log_debug", | v: &Value | debug!("{}", v.to_string()) }
simple_closure! { LogInfo, "log_info", | v: &Value | info!("{}", v.to_string()) }
simple_closure! { LogWarning, "log_warning", | v: &Value | warn!("{}", v.to_string()) }
simple_closure! { LogError, "log_error", | v: &Value | error!("{}", v.to_string()) }
simple_closure! { PrintValue, "print_value", | v: &Value | println!("{}", v.to_string()) }
