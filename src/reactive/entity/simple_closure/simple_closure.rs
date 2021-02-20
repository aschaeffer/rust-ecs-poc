use std::sync::Arc;

use log::debug;
use serde_json::Value;

use crate::model::ReactiveEntityInstance;
use crate::reactive::Disconnectable;
use crate::reactive::entity::simple_closure::SimpleClosureProperties;

pub type SimpleClosureFunction = fn(&Value) -> ();
// pub type SimpleClosureFunction = Box<dyn FnMut(&Value) -> ()>;
// pub type SimpleClosureFunction = Box<dyn Fn(&Value) -> ()>;

/// The simple closure is meant to be used as a sink.
///
/// The entity has only one property named "value". The
pub struct SimpleClosure {
    // pub f: SimpleClosureFunction,

    pub entity: Arc<ReactiveEntityInstance>,

    pub handle_id: u128,
}

impl SimpleClosure {
    pub fn new<'a, F: 'static + Sized>(e: Arc<ReactiveEntityInstance>, f: Box<F>) -> SimpleClosure
    where
        F: 'a + FnMut(&Value) -> ()
        // F: 'a + SimpleClosureFunction
    {
        let handle_id = e.properties.get(SimpleClosureProperties::INPUT.as_ref()).unwrap().id.as_u128();
        e.properties.get(SimpleClosureProperties::INPUT.as_ref()).unwrap()
            .stream.read().unwrap()
            .observe_with_handle(f, handle_id);
        SimpleClosure {
            // f,
            entity: e.clone(),
            handle_id
        }
    }

    pub fn type_name(&self) -> String {
        self.entity.type_name.clone()
    }
}

impl Disconnectable for SimpleClosure {
    fn disconnect(&self) {
        debug!("Disconnect simple closure {}", self.handle_id);
        self.entity.properties.get(SimpleClosureProperties::INPUT.as_ref()).unwrap()
            .stream.read().unwrap().remove(self.handle_id);
    }
}

/// Automatically disconnect streams on destruction
impl Drop for SimpleClosure {
    fn drop(&mut self) {
        debug!("Drop simple closure");
        self.disconnect();
    }
}
