// extern crate serde;
// extern crate waiter_di;
// use std::rc::Rc;
// use waiter_di::*;
// use indradb::MemoryDatastore;
//
// trait GraphDatabase: Send {
//     fn demo(&self);
// }
//
// #[component]
// struct GraphDatabaseImpl {
//
//     pub datastore: MemoryDatastore,
//
// }
//
// #[provides]
// impl GraphDatabase for GraphDatabaseImpl {
//     fn demo(&self) {
//         println!("Dependency");
//     }
// }
