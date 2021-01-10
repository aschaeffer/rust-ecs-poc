use async_trait::async_trait;

#[async_trait]
pub trait SystemConstantsInitializer : Send + Sync {

    fn activate (&self);
    fn deactivate (&self);

}
