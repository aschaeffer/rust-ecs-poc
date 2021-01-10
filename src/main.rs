use crate::application::Application;
// use crate::api::*;
use crate::di::*;
use waiter_di::*;

mod api;
mod application;
mod di;
mod implementation;
mod model;
mod reactive;

#[async_std::main]
async fn main () {
    // let mut application = Application::new();
    let mut container = di_container::get::<profiles::Default>();
    let container = &mut container;
    let mut application = Provider::<dyn Application>::create(container);
    // let component_manager= Provider::<dyn ComponentManager>::get_ref(container);
    // let entity_type_manager= Provider::<dyn EntityTypeManager>::get_ref(container);
    // let mut application = Provider::<dyn Application>::create(&mut container);
    // Provider::<dyn ComponentManager>::get_ref(&mut container);
    // Provider::<dyn ComponentManager>::get_ref(&mut container);

    // component_manager.load_static_components();
    application.init().await;
    application.run().await;
}
