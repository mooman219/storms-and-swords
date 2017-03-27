use game::entity::Entity;
//a Component is given to a entity
pub trait Component {
    fn get_name(&self) -> String;
    fn to_box(self) -> Box<Component>;
//    fn get_uid(&self) -> UID;
}
