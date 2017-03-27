use game::entity::Entity;
//a Component is given to a entity
pub trait Component {
    fn get_name(&self) -> String;
    fn set_entity(&mut self, entity:Box<Entity>);
//    fn get_uid(&self) -> UID;
}
