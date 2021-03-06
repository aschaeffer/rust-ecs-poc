use std::str::FromStr;
use std::sync::Arc;

use indradb::{Type, Vertex, VertexProperties};
use uuid::Uuid;

use crate::model::{ReactiveEntityInstance, ReactiveEntityInstanceFactory};
use crate::reactive::entity::const_value::ConstValueProperties;
use crate::reactive::ConstValue;

pub struct ConstValueReactiveEntityInstanceFactory {}
impl ConstValueReactiveEntityInstanceFactory {
    pub fn new_default() -> Arc<ReactiveEntityInstance> {
        ConstValueReactiveEntityInstanceFactory::new(ConstValue::DEFAULT_TYPE_NAME)
    }
}
impl ReactiveEntityInstanceFactory for ConstValueReactiveEntityInstanceFactory {
    fn new<S: Into<String>>(type_name: S) -> Arc<ReactiveEntityInstance> {
        Arc::new(
            ReactiveEntityInstance::from(
                VertexProperties {
                    vertex: Vertex {
                        id: Uuid::new_v4(),
                        t: Type::from_str(type_name.into().as_str()).unwrap()
                    },
                    props: ConstValueProperties::properties()
                }
            )
        )
    }
}
