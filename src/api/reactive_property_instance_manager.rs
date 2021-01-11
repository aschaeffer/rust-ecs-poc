use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait ReactivePropertyInstanceManager: Send + Sync {

    fn connect(&self, outbound_uuid: Uuid, inbound_uuid: Uuid);

    fn disconnect(&self, outbound_uuid: Uuid, inbound_uuid: Uuid);

}
