use game::World;

pub trait Event {
    fn execute(&self, world: &mut World);
}

pub mod move_event;