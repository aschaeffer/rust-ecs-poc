use crate::bidule::Stream;
use serde_json::Value;
use std::ops::{Deref, DerefMut};
use std::sync::RwLock;
use uuid::Uuid;

// This is not automatically persisted to graph database (yet)!
pub struct ReactivePropertyInstance<'a> {
    // Vertex uuid
    pub id: Uuid,

    // Property name
    pub name: String,

    // pub property: NamedProperty,
    pub stream: RwLock<Stream<'a, Value>>,

    // Store the current value
    pub value: RwLock<Value>,
}

impl ReactivePropertyInstance<'_> {
    pub fn new(id: Uuid, name: String, value: Value) -> ReactivePropertyInstance<'static> {
        let mut i = ReactivePropertyInstance {
            id,
            name,
            stream: RwLock::new(Stream::new()),
            value: RwLock::new(value),
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

    // Resend the current value manually
    pub fn tick(&self) {
        let value = self.value.read().unwrap().deref().clone();
        self.stream.read().unwrap().send(&value);
    }

    pub fn as_bool(&self) -> Option<bool> {
        self.get().as_bool()
    }

    pub fn as_u64(&self) -> Option<u64> {
        self.get().as_u64()
    }

    pub fn as_i64(&self) -> Option<i64> {
        self.get().as_i64()
    }

    pub fn as_f64(&self) -> Option<f64> {
        self.get().as_f64()
    }

    pub fn as_string(&self) -> Option<String> {
        self.get().as_str().and_then(|s| Some(String::from(s)))
    }
}
