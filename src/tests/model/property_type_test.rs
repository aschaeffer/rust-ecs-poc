use crate::model::{PropertyType};
use random_string::{RandomString, Charset, Charsets};

#[test]
fn property_type_test() {
    let property_name =
        RandomString::generate(10, &Charset::from_charsets(Charsets::Letters)).to_string();

    let data_type = String::from("string");

    let property_type = PropertyType {
        name: property_name.clone(),
        data_type: data_type.clone()
    };

    assert_eq!(property_name.clone(), property_type.name);
    assert_eq!(data_type.clone(), property_type.data_type);
}

#[test]
fn property_type_serde_test() {
    let property_name =
        RandomString::generate(10, &Charset::from_charsets(Charsets::Letters)).to_string();

    let data_type = String::from("string");

    let property_type = PropertyType {
        name: property_name.clone(),
        data_type: data_type.clone()
    };

    let result = serde_json::to_string_pretty(&property_type.clone());
    assert!(result.is_ok());
    let result_2 = serde_json::from_str(result.unwrap().as_str());
    assert!(result_2.is_ok());
    let property_type_2: PropertyType = result_2.unwrap();

    assert_eq!(property_name.clone(), property_type_2.name);
    assert_eq!(data_type.clone(), property_type_2.data_type);
}
