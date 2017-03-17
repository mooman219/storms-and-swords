//a Component is given to a entity
pub trait Component {
    fn get_name(&self) -> String;
}