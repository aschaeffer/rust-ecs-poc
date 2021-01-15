use random_string::{RandomString, Charset, Charsets};
use crate::model::ReactivePropertyInstance;
use uuid::v1::Timestamp;
use uuid::Uuid;
use std::sync::{RwLock, Mutex, Arc};
use bidule::Stream;
use serde_json::json;
use std::ops::DerefMut;
use std::borrow::{BorrowMut, Borrow};
use std::cell::RefCell;
use std::rc::Rc;

#[test]
fn reactive_property_instance_test() {
    let ts = Timestamp::from_rfc4122(1497624119, 0);
    let uuid = Uuid::new_v1(ts, &[1, 2, 3, 4, 5, 6], ).unwrap();

    let property_name =
        RandomString::generate(10, &Charset::from_charsets(Charsets::Letters)).to_string();

    let initial_property_value =
        RandomString::generate(10, &Charset::from_charsets(Charsets::Letters)).to_string();

    let initial_property_value_json = json!(initial_property_value);

    let reactive_property_instance = ReactivePropertyInstance {
        id: uuid,
        name: property_name.clone(),
        stream: RwLock::new(Stream::new()),
        value: RwLock::new(initial_property_value_json),
    };

    // Check that the meta data is correct
    assert_eq!(uuid, reactive_property_instance.id);
    assert_eq!(property_name.clone(), reactive_property_instance.name);
    assert_eq!(initial_property_value.as_str(), reactive_property_instance.value.read().unwrap().as_str().unwrap());

    // Set: Send to "stream", write inner "value"

    let new_property_value =
        RandomString::generate(10, &Charset::from_charsets(Charsets::Letters)).to_string();
    let new_property_value_json = json!(new_property_value);

    reactive_property_instance.set(new_property_value_json);

    // Check that the inner value has changed
    assert_eq!(new_property_value.as_str(), reactive_property_instance.value.read().unwrap().as_str().unwrap());
    assert_eq!(new_property_value.as_str(), reactive_property_instance.get().as_str().unwrap());

    // Send: Send to "stream", do not change the inner "value" (!)

    let send_property_value =
        RandomString::generate(10, &Charset::from_charsets(Charsets::Letters)).to_string();
    let send_property_value_json = json!(send_property_value);

    reactive_property_instance.send(&send_property_value_json);

    // Check that the inner value has not changed
    assert_eq!(new_property_value.as_str(), reactive_property_instance.value.read().unwrap().as_str().unwrap());
    assert_eq!(new_property_value.as_str(), reactive_property_instance.get().as_str().unwrap());

    // Check that the inner value is the same
    assert_ne!(send_property_value.as_str(), reactive_property_instance.value.read().unwrap().as_str().unwrap());
    assert_ne!(send_property_value.as_str(), reactive_property_instance.get().as_str().unwrap());

    // Create an observer which sinks on a variable

    let observed_value_json = Arc::new(RwLock::new(reactive_property_instance.get()));
    let mut inner_observed_value_json = Arc::clone(&observed_value_json);
    reactive_property_instance.stream.read().unwrap().observe(move |value| {
        let mut writer = inner_observed_value_json.write().unwrap();
        *writer.deref_mut() = value.clone();
    });

    reactive_property_instance.send(&send_property_value_json);

    // Check that the observer gets the sent value
    assert_eq!(send_property_value.as_str(), observed_value_json.read().unwrap().as_str().unwrap());
    // Check that the value hasn't changed
    assert_eq!(new_property_value.as_str(), reactive_property_instance.get().as_str().unwrap());

    // Resend the last value

    let tick_value_json = Arc::new(RwLock::new(json!("")));
    let mut i_tick_value_json = Arc::clone(&tick_value_json);
    reactive_property_instance.stream.read().unwrap().observe(move |value| {
        let mut writer = i_tick_value_json.write().unwrap();
        *writer.deref_mut() = value.clone();
    });

    reactive_property_instance.tick();

    // Check that the inner value has been sent to the observer
    assert_eq!(new_property_value.as_str(), tick_value_json.read().unwrap().as_str().unwrap());

}

#[test]
fn create_reactive_property_instance_test() {
    let ts = Timestamp::from_rfc4122(1497624119, 0);
    let uuid = Uuid::new_v1(ts, &[1, 2, 3, 4, 5, 6], ).unwrap();

    let property_name =
        RandomString::generate(10, &Charset::from_charsets(Charsets::Letters)).to_string();

    let initial_property_value =
        RandomString::generate(10, &Charset::from_charsets(Charsets::Letters)).to_string();

    let initial_property_value_json = json!(initial_property_value);

    let reactive_property_instance = ReactivePropertyInstance::new(
        uuid,
        property_name.clone(),
        initial_property_value_json,
    );

    assert_eq!(uuid, reactive_property_instance.id);
    assert_eq!(property_name.clone(), reactive_property_instance.name);
    assert_eq!(initial_property_value.as_str(), reactive_property_instance.value.read().unwrap().as_str().unwrap());

    // Set: Send to "stream", write "value"

    let new_property_value =
        RandomString::generate(10, &Charset::from_charsets(Charsets::Letters)).to_string();

    let new_property_value_json = json!(new_property_value);

    reactive_property_instance.set(new_property_value_json);

    assert_eq!(new_property_value.as_str(), reactive_property_instance.value.read().unwrap().as_str().unwrap());
    assert_eq!(new_property_value.as_str(), reactive_property_instance.get().as_str().unwrap());

    // Send: Send to "stream", do not change "value"

    let send_property_value =
        RandomString::generate(10, &Charset::from_charsets(Charsets::Letters)).to_string();

    let send_property_value_json = json!(send_property_value);

    reactive_property_instance.send(&send_property_value_json);

    assert_eq!(new_property_value.as_str(), reactive_property_instance.value.read().unwrap().as_str().unwrap());
    assert_eq!(new_property_value.as_str(), reactive_property_instance.get().as_str().unwrap());
    assert_ne!(send_property_value.as_str(), reactive_property_instance.value.read().unwrap().as_str().unwrap());
    assert_ne!(send_property_value.as_str(), reactive_property_instance.get().as_str().unwrap());

}

