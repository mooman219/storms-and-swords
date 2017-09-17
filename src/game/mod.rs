pub mod entity;
pub mod world;
pub mod player;
pub mod paddle;
pub mod input;
pub mod ball;

pub use self::player::Player;
pub use self::world::World;
pub use self::input::Input;
pub type ContentId = u64;
