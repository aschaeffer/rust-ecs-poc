use serde_json::Value;
use indradb::NamedProperty;

pub trait Disconnectable {
    fn disconnect(&self);
}

pub trait Operation: Disconnectable {
    fn lhs(&self, value: Value);

    fn result(&self) -> Value;
}

pub type NamedProperties = Vec<NamedProperty>;
