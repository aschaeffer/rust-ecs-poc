use bidule::Stream;
// use indradb::NamedProperty;
use serde_json::Value;
use std::sync::RwLock;
use uuid::Uuid;
use std::ops::Deref;

// This is not automatically persisted to graph database (yet)!
pub struct ReactivePropertyInstance<'a> {

    // Vertex uuid
    pub id: Uuid,

    // Property name
    pub name: String,

    // pub property: NamedProperty,

    pub stream: RwLock<Stream<'a, Value>>,

    // Store the current value
    value: RwLock<Value>,

    // Reference to the inbound stream ?
    // pub inbound: ReactivePropertyInstance,

    // References to the outbound streams ?
    // pub outbound: Vec<ReactivePropertyInstance>,

}

impl ReactivePropertyInstance<'_> {

    pub fn new(id: Uuid, name: String, value: Value) -> ReactivePropertyInstance<'static> {
        let mut i = ReactivePropertyInstance {
            id,
            name,
            stream: RwLock::new(Stream::new()),
            value: RwLock::new(value)
        };
        i.init();
        i
    }

    // pub fn observe<F>(&self, subscriber: F) where F: 'static + FnMut(&Value) {
    //     self.stream.observe(subscriber);
    // }


    pub fn init(&mut self) {
        // self.stream.write().unwrap().observe(| value | self.value = value.clone());
        // | json_value | self.property.value = json_value.clone()
        // self.observe(| json_value | self.property.value = *json_value);
    }

    pub fn get(&self) -> Value {
        self.value.read().unwrap().clone()
    }

    pub fn set(&self, value: Value) {
        let mut writer = self.value.write().unwrap();
        *writer.deref_mut() = value.clone();
        self.stream.read().unwrap().send(&value);
    }

    // Send but not set
    pub fn send(&self, signal: &Value) {
        self.stream.read().unwrap().send(signal);
    }

    // Resend the last value
    pub fn tick(&self) {
        let value = self.value.read().unwrap().deref().clone();
        self.stream.read().unwrap().send(&value);
    }

    // pub fn save(&mut self, value: Value) {
    //
    // }

    // pub fn create_gate(&mut self) {
    //     let stream2 = self.stream.write().unwrap();
    //     stream2.filter(| v | v % 2 == 0);
    // }

}
