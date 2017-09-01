use game::entity::{Entity, UID};
use cgmath::Vector3;



pub struct Player {
    position: Vector3<f32>,
    scale: Vector3<f32>,
    rotation: Vector3<f32>,
    uid: UID,
}

impl Player {
    pub fn new(uid: UID) -> Player {
        Player {
            position: Vector3::new(0.0, 0.0, 0.0),
            scale: Vector3::new(1.0, 1.0, 1.0),
            rotation: Vector3::new(0.0, 0.0, 0.0),
            uid: uid,
        }
    }
}
