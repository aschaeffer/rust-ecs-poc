use crate::entity_system::model::component::Component;

pub struct ComponentFactory {}
impl ComponentFactory {

    // TODO: load json file from statically compiled resource
    pub fn from_resource(name: String) -> Option<Component> {
        None
    }

    // TODO: load json file from file system
    pub fn from_file(name: String) -> Option<Component> {
        None
    }

}
