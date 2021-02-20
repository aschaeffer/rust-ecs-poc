use indradb::NamedProperty;
use serde_json::json;
use strum_macros::{AsRefStr, IntoStaticStr, ToString};

use crate::reactive::NamedProperties;

#[allow(non_camel_case_types)]
#[derive(AsRefStr, IntoStaticStr, ToString)]
pub enum SimpleClosureProperties {
    #[strum(serialize = "input")]
    INPUT,
}
impl SimpleClosureProperties {
    pub fn default_value(&self) -> f64 {
        match self {
            SimpleClosureProperties::INPUT => 0.0,
        }
    }
    pub fn properties() -> NamedProperties {
        vec![
            NamedProperty::from(SimpleClosureProperties::INPUT),
        ]
    }
}

impl From<SimpleClosureProperties> for NamedProperty {
    fn from(p: SimpleClosureProperties) -> Self {
        NamedProperty {
            name: p.to_string(),
            value: json!(p.default_value())
        }
    }
}
