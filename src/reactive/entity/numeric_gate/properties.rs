use indradb::NamedProperty;
use serde_json::json;
use strum_macros::{AsRefStr, IntoStaticStr, ToString};

use crate::reactive::NamedProperties;

#[allow(non_camel_case_types)]
#[derive(AsRefStr, IntoStaticStr, ToString)]
pub enum NumericGateProperties {
    #[strum(serialize = "lhs")]
    LHS,
    #[strum(serialize = "rhs")]
    RHS,
    #[strum(serialize = "result")]
    RESULT
}

impl NumericGateProperties {
    pub fn default_value(&self) -> f64 {
        match self {
            NumericGateProperties::LHS => 0.0,
            NumericGateProperties::RHS => 0.0,
            NumericGateProperties::RESULT => 0.0,
        }
    }
    pub fn properties() -> NamedProperties {
        vec![
            NamedProperty::from(NumericGateProperties::LHS),
            NamedProperty::from(NumericGateProperties::RHS),
            NamedProperty::from(NumericGateProperties::RESULT)
        ]
    }
}

impl From<NumericGateProperties> for NamedProperty {
    fn from(p: NumericGateProperties) -> Self {
        NamedProperty {
            name: p.to_string(),
            value: json!(p.default_value())
        }
    }
}
