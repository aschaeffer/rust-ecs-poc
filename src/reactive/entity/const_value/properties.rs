use indradb::NamedProperty;
use serde_json::json;
use strum_macros::{AsRefStr, IntoStaticStr, ToString};

use crate::reactive::NamedProperties;

#[allow(non_camel_case_types)]
#[derive(AsRefStr, IntoStaticStr, ToString)]
pub enum ConstValueProperties {
    #[strum(serialize = "value")]
    VALUE,
}

impl ConstValueProperties {
    pub fn default_value(&self) -> i64 {
        match self {
            ConstValueProperties::VALUE => 0
        }
    }
    pub fn properties() -> NamedProperties {
        vec![
            NamedProperty::from(ConstValueProperties::VALUE),
        ]
    }
}

impl From<ConstValueProperties> for NamedProperty {
    fn from(p: ConstValueProperties) -> Self {
        NamedProperty {
            name: p.to_string(),
            value: json!(p.default_value())
        }
    }
}
