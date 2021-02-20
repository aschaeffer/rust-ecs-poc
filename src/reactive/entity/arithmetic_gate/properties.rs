use indradb::NamedProperty;
use serde_json::json;
use strum_macros::{AsRefStr, IntoStaticStr, ToString};

use crate::reactive::NamedProperties;

#[allow(non_camel_case_types)]
#[derive(AsRefStr, IntoStaticStr, ToString)]
pub enum ArithmeticGateProperties {
    #[strum(serialize = "lhs")]
    LHS,
    #[strum(serialize = "rhs")]
    RHS,
    #[strum(serialize = "result")]
    RESULT
}

impl ArithmeticGateProperties {
    pub fn default_value(&self) -> i64 {
        match self {
            ArithmeticGateProperties::LHS => 0,
            ArithmeticGateProperties::RHS => 0,
            ArithmeticGateProperties::RESULT => 0,
        }
    }
    pub fn properties() -> NamedProperties {
        vec![
            NamedProperty::from(ArithmeticGateProperties::LHS),
            NamedProperty::from(ArithmeticGateProperties::RHS),
            NamedProperty::from(ArithmeticGateProperties::RESULT)
        ]
    }
}

impl From<ArithmeticGateProperties> for NamedProperty {
    fn from(p: ArithmeticGateProperties) -> Self {
        NamedProperty {
            name: p.to_string(),
            value: json!(p.default_value())
        }
    }
}
