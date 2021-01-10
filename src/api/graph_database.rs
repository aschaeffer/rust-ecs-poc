use async_trait::async_trait;

#[async_trait]
pub trait GraphDatabase : Send + Sync {

    fn get_transaction (&mut self) -> indradb::Result<indradb::MemoryTransaction>;

}
