use indradb::NamedProperty;
use serde_json::json;
use strum_macros::{AsRefStr, IntoStaticStr, ToString};

use crate::reactive::NamedProperties;

#[allow(non_camel_case_types)]
#[derive(AsRefStr, IntoStaticStr, ToString)]
pub enum LogicalGateProperties {
    #[strum(serialize = "lhs")]
    LHS,
    #[strum(serialize = "rhs")]
    RHS,
    #[strum(serialize = "result")]
    RESULT
}

impl LogicalGateProperties {
    pub fn default_value(&self) -> bool {
        match self {
            LogicalGateProperties::LHS => false,
            LogicalGateProperties::RHS => false,
            LogicalGateProperties::RESULT => false,
        }
    }
    pub fn properties() -> NamedProperties {
        vec![
            NamedProperty::from(LogicalGateProperties::LHS),
            NamedProperty::from(LogicalGateProperties::RHS),
            NamedProperty::from(LogicalGateProperties::RESULT)
        ]
    }
}

impl From<LogicalGateProperties> for NamedProperty {
    fn from(p: LogicalGateProperties) -> Self {
        NamedProperty {
            name: p.to_string(),
            value: json!(p.default_value())
        }
    }
}
