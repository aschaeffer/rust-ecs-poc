use crate::model::PropertyType;
use async_trait::async_trait;
use crate::api::Lifecycle;

#[async_trait]
pub trait ComponentManager: Send + Sync + Lifecycle {
    fn register(&self, component: crate::model::Component);
    fn load_static_components(&self);
    fn get_components(&self) -> Vec<crate::model::Component>;

    fn has(&self, name: String) -> bool;
    fn get(&self, name: String) -> Option<crate::model::Component>;

    fn create(&self, name: String, properties: Vec<PropertyType>);
    fn delete(&self, name: String);

    fn import(&self, path: String);
    fn export(&self, name: String, path: String);
}
