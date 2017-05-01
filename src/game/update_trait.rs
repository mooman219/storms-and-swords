use game::World;

pub trait Update {
    fn update(world: &World);
}