use std::str::FromStr;
use std::sync::Arc;

use indradb::{Type, Vertex, VertexProperties};
use uuid::Uuid;

use crate::model::{ReactiveEntityInstance, ReactiveEntityInstanceFactory};
use crate::reactive::entity::logical_gate::LogicalGateProperties;

pub struct LogicalGateReactiveEntityInstanceFactory {}
impl ReactiveEntityInstanceFactory for LogicalGateReactiveEntityInstanceFactory {
    fn new<S: Into<String>>(type_name: S) -> Arc<ReactiveEntityInstance> {
        Arc::new(
            ReactiveEntityInstance::from(
                VertexProperties {
                    vertex: Vertex {
                        id: Uuid::new_v4(),
                        t: Type::from_str(type_name.into().as_str()).unwrap()
                    },
                    props: LogicalGateProperties::properties()
                }
            )
        )
    }
}
