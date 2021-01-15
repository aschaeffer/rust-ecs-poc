use crate::model::ReactiveEntityInstance;
use uuid::v1::Timestamp;
use uuid::Uuid;
use random_string::{RandomString, Charset, Charsets};
use std::collections::HashMap;

#[test]
fn entity_instance_test() {
    let ts = Timestamp::from_rfc4122(1497624119, 0);
    let uuid = Uuid::new_v1(ts, &[1, 2, 3, 4, 5, 6], ).unwrap();
    let type_name =
        RandomString::generate(10, &Charset::from_charsets(Charsets::Letters)).to_string();
    let description =
        RandomString::generate(10, &Charset::from_charsets(Charsets::Letters)).to_string();

    let reactive_entity_instance = ReactiveEntityInstance {
        type_name: type_name.clone(),
        id: uuid.clone(),
        description: description.clone(),
        properties: HashMap::new(),
    };

    assert_eq!(type_name.clone(), reactive_entity_instance.type_name.clone());
    assert_eq!(uuid.clone(), reactive_entity_instance.id.clone());
    assert_eq!(description.clone(), reactive_entity_instance.description.clone());

}
