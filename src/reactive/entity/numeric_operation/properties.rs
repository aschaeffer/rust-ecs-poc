use indradb::NamedProperty;
use serde_json::json;
use strum_macros::{AsRefStr, IntoStaticStr, ToString};

use crate::reactive::NamedProperties;

#[allow(non_camel_case_types)]
#[derive(AsRefStr, IntoStaticStr, ToString)]
pub enum NumericOperationProperties {
    #[strum(serialize = "lhs")]
    LHS,
    #[strum(serialize = "result")]
    RESULT
}

impl NumericOperationProperties {
    pub fn default_value(&self) -> f64 {
        match self {
            NumericOperationProperties::LHS => 0.0,
            NumericOperationProperties::RESULT => 0.0,
        }
    }
    pub fn properties() -> NamedProperties {
        vec![
            NamedProperty::from(NumericOperationProperties::LHS),
            NamedProperty::from(NumericOperationProperties::RESULT)
        ]
    }
}

impl From<NumericOperationProperties> for NamedProperty {
    fn from(p: NumericOperationProperties) -> Self {
        NamedProperty {
            name: p.to_string(),
            value: json!(p.default_value())
        }
    }
}
