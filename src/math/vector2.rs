use std::ops::Add;


#[derive(Clone, Copy)]
pub struct Vector2 {
    pub vals: [f32: 2]
}

impl Add for Vector2 {
    type Output = Vector2;

    fn add(self, other: Vector2) -> Vector2 {
        Vector{vals: [self.vals[0] + other.vals[0], 
                      self.vals[1] + other.vals[1]]}
    }
}