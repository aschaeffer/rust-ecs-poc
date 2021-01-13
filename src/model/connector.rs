use crate::model::ReactivePropertyInstance;
use bidule::Stream;
use serde_json::Value;

pub struct Connector<'a, 'b, 'c> {
    // Connector uuid
    // pub id: Uuid,
    pub inbound: ReactivePropertyInstance<'a>,

    pub connection: Stream<'b, Value>,

    pub outbound: ReactivePropertyInstance<'c>,
}

impl Connector<'_, '_, '_> {
    fn new(
        inbound: ReactivePropertyInstance<'static>,
        outbound: ReactivePropertyInstance<'static>,
    ) -> Connector<'static, 'static, 'static> {
        let connection = Stream::new();
        let connector = Connector {
            connection,
            inbound,
            outbound,
        };
        //
        // outbound.stream.read().unwrap().observe(move |value| {
        //     inbound.stream.read().unwrap().send(value);
        // });
        connector

        // connector.inbound.stream.write().unwrap().send(value);
        // TODO: connector
    }
}
