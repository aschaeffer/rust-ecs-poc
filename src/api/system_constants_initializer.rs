use async_trait::async_trait;
use crate::api::Lifecycle;

#[async_trait]
pub trait SystemConstantsInitializer: Send + Sync + Lifecycle {
    // fn activate(&self);
    // fn deactivate(&self);
}
