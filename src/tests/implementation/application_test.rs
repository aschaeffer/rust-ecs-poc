use crate::application::Application;
use crate::di::di_container;
use waiter_di::{profiles, Provider};
use std::sync::RwLock;

#[test]
fn dependency_injection_test () {
    let mut container = di_container::get::<profiles::Default>();
    let container = &mut container;
    let mut application = RwLock::new(Provider::<dyn Application>::create(container));

    {
        application.read().unwrap().init();
    }

}
