use serde_json::Value;

use crate::reactive::Operation;

pub trait Gate: Operation {
    fn rhs(&self, value: Value);
}
