use serde_json::Value;

pub trait PropertyInstanceGetter {
    /// Returns the boolean value of the given property by name
    fn as_bool(&self, property_name: String) -> Option<bool>;

    /// Returns the u64 value of the given property by name
    fn as_u64(&self, property_name: String) -> Option<u64>;

    /// Returns the i64 value of the given property by name
    fn as_i64(&self, property_name: String) -> Option<i64>;

    /// Returns the f64 value of the given property by name
    fn as_f64(&self, property_name: String) -> Option<f64>;

    /// Returns the string value of the given property by name
    fn as_string(&self, property_name: String) -> Option<String>;
}

pub trait MutablePropertyInstanceSetter: PropertyInstanceGetter {
    /// Sets the value of the given property by name
    fn set(&mut self, property_name: String, value: Value);
}

pub trait PropertyInstanceSetter: PropertyInstanceGetter {
    /// Sets the value of the given property by name
    fn set(&self, property_name: String, value: Value);
}
