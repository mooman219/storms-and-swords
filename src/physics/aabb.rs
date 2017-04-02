use cgmath::{Vector2, Vector3};

#[derive(Debug, Clone, Copy)]
pub struct AABB2 {
    pub min: Vector2<f32>,
    pub max: Vector2<f32>,
}

#[derive(Debug, Clone, Copy)]
pub struct AABB3 {
    pub min: Vector3<f32>,
    pub max: Vector3<f32>,
}

impl AABB2 {
    pub fn intersects(&self, other: AABB2) -> bool {
        if self.max.x < other.min.x || self.min.x > other.max.x {
            return false;
        }
        if self.max.y < other.min.y || self.min.y > other.max.y {
            return false;
        }
        true
    }

    pub fn slide(&mut self, _movement: Vector2<f32>, _asdf: Vec<AABB2>) {}
}

impl AABB3 {
    pub fn intersects(&self, other: AABB3) -> bool {
        if self.max.x < other.min.x || self.min.x > other.max.x {
            return false;
        }
        if self.max.y < other.min.y || self.min.y > other.max.y {
            return false;
        }
        if self.max.z < other.min.z || self.min.z > other.max.z {
            return false;
        }
        true
    }
}