#![feature(unsized_tuple_coercion)]

use waiter_di::*;

use crate::application::Application;
use crate::di::*;

mod api;
mod application;
mod di;
mod implementation;
mod model;
mod reactive;
pub mod bidule;
mod behaviour;

#[cfg(test)]
#[cfg_attr(tarpaulin, ignore)]
mod tests;

#[async_std::main]
async fn main() {
    let mut container = di_container::get::<profiles::Default>();
    let container = &mut container;
    let mut application = Provider::<dyn Application>::create(container);

    application.init();
    application.run().await;
}
