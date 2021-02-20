// RAW and Dirty

use async_trait::async_trait;

#[async_trait]
pub trait JsEngine: Send + Sync {
    fn init(&self);

    fn run(&self);
}
