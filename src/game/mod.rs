pub mod input;
pub mod ui;
pub mod ui_components;
pub mod system;
pub mod playfield_controller;
pub mod in_battle_character;
pub mod battle_controller;
pub mod controller;

pub use self::input::Input;
pub type ContentId = u64;