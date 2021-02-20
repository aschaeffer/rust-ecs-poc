use serde_json::json;

use crate::api::PropertyInstanceSetter;
use crate::behaviour::{PrintValue, EntityBehaviour, DefaultConnector, ConnectorBehaviour};
use crate::reactive::entity::simple_closure::SimpleClosureProperties;
use std::sync::Arc;
use crate::tests::{create_random_entity_instance, create_default_connector};
use crate::reactive::{ConstValue, Connector, ConstValueProperties};

#[test]
fn print_value_test () {
    let print_value_1 = PrintValue::new().unwrap();
    let print_value_2 = PrintValue::new().unwrap();
    let print_value_3 = PrintValue::new().unwrap();
    print_value_1.entity.set(SimpleClosureProperties::INPUT.to_string(), json!("Hello Inexor!"));
    print_value_2.entity.set(SimpleClosureProperties::INPUT.to_string(), json!("(c) 2021"));
    print_value_3.entity.set(SimpleClosureProperties::INPUT.to_string(), json!("========"));
    // Don't know yet how to unit test if the string has been written to stdout

    let entity = Arc::new(create_random_entity_instance(ConstValueProperties::VALUE.to_string()));
    let const_value = ConstValue::from(entity.clone());

    {
        let r_const_value_to_print_value_1 = Arc::new(create_default_connector(
            entity.clone(),
            print_value_1.entity.clone(),
            ConstValueProperties::VALUE.to_string(),
            SimpleClosureProperties::INPUT.to_string()
        ));
        let c_const_value_to_print_value_1 = Connector::from_relation(r_const_value_to_print_value_1.clone(), DefaultConnector::OPERATION);
        assert_ne!(0, c_const_value_to_print_value_1.handle_id);

        let r_const_value_to_print_value_2 = Arc::new(create_default_connector(
            entity.clone(),
            print_value_2.entity.clone(),
            ConstValueProperties::VALUE.to_string(),
            SimpleClosureProperties::INPUT.to_string()
        ));
        let c_const_value_to_print_value_2 = Connector::from_relation(r_const_value_to_print_value_2.clone(), DefaultConnector::OPERATION);
        assert_ne!(0, c_const_value_to_print_value_2.handle_id);

        let r_const_value_to_print_value_3 = Arc::new(create_default_connector(
            entity.clone(),
            print_value_3.entity.clone(),
            ConstValueProperties::VALUE.to_string(),
            SimpleClosureProperties::INPUT.to_string()
        ));
        let c_const_value_to_print_value_3 = Connector::from_relation(r_const_value_to_print_value_3.clone(), DefaultConnector::OPERATION);
        assert_ne!(0, c_const_value_to_print_value_3.handle_id);

        // Now const_value is connected to all three instances of print_value
        const_value.set(&json!("This is printed three times :-)))"));
    }
    const_value.set(&json!("This is not printed because the connectors are gone :-("));
}
