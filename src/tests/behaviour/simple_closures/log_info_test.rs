use serde_json::json;

use crate::api::PropertyInstanceSetter;
use crate::behaviour::{LogInfo, EntityBehaviour, DefaultConnector, ConnectorBehaviour};
use crate::reactive::entity::simple_closure::SimpleClosureProperties;
use std::sync::Arc;
use crate::tests::{create_random_entity_instance, create_default_connector};
use crate::reactive::{ConstValue, Connector, ConstValueProperties};

#[test]
fn log_info_test () {

    let logger_result = log4rs::init_file("config/logging.yml", Default::default());
    match logger_result {
        Err(error) => {
            println!("Failed to configure logger: {}", error);
        },
        _ => {}
    }

    let log_info_1 = LogInfo::new().unwrap();
    let log_info_2 = LogInfo::new().unwrap();
    let log_info_3 = LogInfo::new().unwrap();
    log_info_1.entity.set(SimpleClosureProperties::INPUT.to_string(), json!("Hello Inexor!"));
    log_info_2.entity.set(SimpleClosureProperties::INPUT.to_string(), json!("(c) 2021"));
    log_info_3.entity.set(SimpleClosureProperties::INPUT.to_string(), json!("========"));

    let entity = Arc::new(create_random_entity_instance(ConstValueProperties::VALUE.to_string()));
    let const_value = ConstValue::from(entity.clone());

    {
        let r_const_value_to_log_info_1 = Arc::new(create_default_connector(
            entity.clone(),
            log_info_1.entity.clone(),
            ConstValueProperties::VALUE.to_string(),
            SimpleClosureProperties::INPUT.to_string()
        ));
        let c_const_value_to_log_info_1 = Connector::from_relation(r_const_value_to_log_info_1.clone(), DefaultConnector::OPERATION);
        assert_ne!(0, c_const_value_to_log_info_1.handle_id);

        let r_const_value_to_log_info_2 = Arc::new(create_default_connector(
            entity.clone(),
            log_info_2.entity.clone(),
            ConstValueProperties::VALUE.to_string(),
            SimpleClosureProperties::INPUT.to_string()
        ));
        let c_const_value_to_log_info_2 = Connector::from_relation(r_const_value_to_log_info_2.clone(), DefaultConnector::OPERATION);
        assert_ne!(0, c_const_value_to_log_info_2.handle_id);

        let r_const_value_to_log_info_3 = Arc::new(create_default_connector(
            entity.clone(),
            log_info_3.entity.clone(),
            ConstValueProperties::VALUE.to_string(),
            SimpleClosureProperties::INPUT.to_string()
        ));
        let c_const_value_to_log_info_3 = Connector::from_relation(r_const_value_to_log_info_3.clone(), DefaultConnector::OPERATION);
        assert_ne!(0, c_const_value_to_log_info_3.handle_id);

        // Now const_value is connected to all three instances of log_info
        const_value.set(&json!("This is logged three times :-)))"));
    }
    const_value.set(&json!("This is not logged because the connectors are gone :-("));
}
