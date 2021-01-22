use crate::bidule::Stream;
use serde_json::Value;
use std::ops::{Deref, DerefMut};
use std::sync::RwLock;
use uuid::Uuid;

// This is not automatically persisted to graph database (yet)!
pub struct ReactivePropertyInstance<'a> {
    /// Vertex uuid
    pub id: Uuid,

    /// Property name
    pub name: String,

    /// The reactive stream
    pub stream: RwLock<Stream<'a, Value>>,

    /// Store the current value
    pub value: RwLock<Value>,
}

impl ReactivePropertyInstance<'_> {
    pub fn new(id: Uuid, name: String, value: Value) -> ReactivePropertyInstance<'static> {
        ReactivePropertyInstance {
            id,
            name,
            stream: RwLock::new(Stream::new()),
            value: RwLock::new(value),
        }
    }

    pub fn get(&self) -> Value {
        self.value.read().unwrap().clone()
    }

    pub fn set(&self, value: Value) {
        let mut writer = self.value.write().unwrap();
        *writer.deref_mut() = value.clone();
        self.stream.read().unwrap().send(&value);
    }

    /// Send a value down the stream, but does not change the current value
    pub fn send(&self, signal: &Value) {
        self.stream.read().unwrap().send(signal);
    }

    /// Resend the current value manually
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

    // TODO: Add method as_array()
    // pub fn as_array(&self) -> Option<&Vec<Value>> {
    //     self.get().as_array()
    // }

    // TODO: Add method as_object()
    // pub fn as_object(&self) -> Option<&Map<String, Value>> {
    //     self.get().as_object()
    // }
}
