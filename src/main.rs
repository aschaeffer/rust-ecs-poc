use crate::application::Application;
use crate::di::*;
use waiter_di::*;

mod api;
mod application;
mod di;
mod implementation;
mod model;
mod reactive;

#[cfg(test)]
mod tests;

#[async_std::main]
async fn main () {
    // let mut application = Application::new();
    let mut container = di_container::get::<profiles::Default>();
    let container = &mut container;
    let mut application = Provider::<dyn Application>::create(container);

    application.init();
    application.run().await;
}
