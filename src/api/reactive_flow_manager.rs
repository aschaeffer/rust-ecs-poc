use crate::model::{ReactiveFlow, Flow};
use async_trait::async_trait;
use uuid::Uuid;
use std::sync::Arc;

#[derive(Debug)]
pub struct ReactiveFlowCreationError;

#[derive(Debug)]
pub struct ReactiveFlowImportError;

#[async_trait]
pub trait ReactiveFlowManager: Send + Sync {
    /// Returns true, if an flow exists with the given UUID.
    fn has(&self, id: Uuid) -> bool;

    /// Returns the flow with the given UUID or None.
    fn get(&self, id: Uuid) -> Option<Arc<ReactiveFlow>>;

    //  type_name: String, properties: HashMap<String, Value>
    fn create(&self, flow: Flow) -> Result<Arc<ReactiveFlow>, ReactiveFlowCreationError>;

    fn register(&self, reactive_flow: Arc<ReactiveFlow>);

    // TODO: return result
    fn commit(&self, id: Uuid);

    fn delete(&self, id: Uuid);

    fn import(&self, path: String) -> Result<Arc<ReactiveFlow>, ReactiveFlowImportError>;

    // TODO: return result
    fn export(&self, id: Uuid, path: String);
}
