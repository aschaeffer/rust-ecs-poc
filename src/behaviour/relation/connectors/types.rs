use crate::relation_behaviour;
use serde_json::json;

macro_rules! connector_behaviour {
    ($behaviour_name:ident, $entity_type_name:expr, $f:expr) => {
        relation_behaviour! {
            $behaviour_name,
            crate::behaviour::ConnectorBehaviour,
            crate::reactive::relation::connector::ConnectorFunction,
            $entity_type_name,
            $f
        }
    }
}

// connector_behaviour! { DefaultConnector, "default_connector", |v| v.clone() }
connector_behaviour! { DefaultConnector, "default_connector", |v| { println!("connector propagate {}", v.to_string()); return v.clone(); } }
// TODO: safety checks
connector_behaviour! { ParseIntConnector, "parse_int_connector", |v| json!(v.clone().as_str().unwrap().parse::<i64>().unwrap()) }
connector_behaviour! { ToStringConnector, "to_string_connector", |v| json!(v.clone().to_string()) }

connector_behaviour! { DebugConnector, "debug_connector", |v| { println!("connector propagate {}", v.to_string()); return v.clone(); } }
