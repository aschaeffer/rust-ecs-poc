use crate::model::ReactivePropertyInstance;
use bidule::Stream;
use serde_json::Value;
// use std::sync::RwLock;
// use uuid::Uuid;

pub struct Connector<'b> {

    // Connector uuid
    // pub id: Uuid,

    pub connection: Stream<'b, Value>,

    pub inbound: ReactivePropertyInstance<'b>,
    pub outbound: ReactivePropertyInstance<'b>,

}

impl Connector<'_> {

    fn new(inbound: ReactivePropertyInstance, outbound: ReactivePropertyInstance) -> Connector<'static> {
        let connection = Stream::new();
        let connector = Connector {
            connection,
            inbound,
            outbound
        };
        outbound.observe(move |sig| {
            inbound.send(sig);
        });

        connector.inbound.stream.write().unwrap().send(value)
        // TODO: connector
    }

    fn connect () {

    }
}
