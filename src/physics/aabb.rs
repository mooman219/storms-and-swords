use cgmath::{Vector2, Vector3};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AABB2 {
    pub min: Vector2<f32>,
    pub max: Vector2<f32>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AABB3 {
    pub min: Vector3<f32>,
    pub max: Vector3<f32>,
}

impl AABB2 {
    pub fn new(minx: f32, miny: f32, maxx: f32, maxy: f32) -> AABB2 {
        AABB2 {
            min: Vector2 { x: minx, y: miny },
            max: Vector2 { x: maxx, y: maxy },
        }
    }

    pub fn intersects(&self, other: AABB2) -> bool {
        if self.max.x < other.min.x || self.min.x > other.max.x {
            return false;
        }
        if self.max.y < other.min.y || self.min.y > other.max.y {
            return false;
        }
        true
    }

    pub fn slide<'a, T>(&mut self, mov: Vector2<f32>, others: T)
        where T: Iterator<Item = &'a AABB2> + Clone
    {
        if mov.x == 0f32 && mov.y == 0f32 {
            return;
        }

        let mut res = mov; // Copy
        let mut aabb = self; // Copy

        // Y movement

        if mov.y < 0f32 {
            for other in others.clone() {
                if aabb.max.x > other.min.x && aabb.min.x < other.max.x &&
                   other.max.y <= aabb.min.y {
                    let min = other.max.y - aabb.min.y;
                    if min > res.y {
                        res.y = min;
                    }
                }
            }
        }

        if mov.y > 0f32 {
            for other in others.clone() {
                if aabb.max.x > other.min.x && aabb.min.x < other.max.x &&
                   other.min.y >= aabb.max.y {
                    let max = other.min.y - aabb.max.y;
                    if max < res.y {
                        res.y = max;
                    }
                }
            }
        }

        aabb.min.y += res.y;
        aabb.max.y += res.y;

        // X movement

        if mov.x < 0f32 {
            for other in others.clone() {
                if aabb.max.y > other.min.y && aabb.min.y < other.max.y &&
                   other.max.x <= aabb.min.x {
                    let min = other.max.x - aabb.min.x;
                    if min > res.x {
                        res.x = min;
                    }
                }
            }
        }

        if mov.x > 0f32 {
            for other in others.clone() {
                if aabb.max.y > other.min.y && aabb.min.y < other.max.y &&
                   other.min.x >= aabb.max.x {
                    let max = other.min.x - aabb.max.x;
                    if max < res.x {
                        res.x = max;
                    }
                }
            }
        }

        aabb.min.x += res.x;
        aabb.max.x += res.x;
    }
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

mod tests {
    use super::*;
    use test::*;

    #[test]
    fn aabb2_slide_test() {
        let v = vec![AABB2::new(2f32, 0f32, 3f32, 1f32), AABB2::new(0f32, 1f32, 1f32, 2f32)];
        let mut aabb = AABB2::new(0f32, 0f32, 1f32, 1f32);

        {
            let mot = Vector2::new(2f32, 0f32);
            aabb.slide(mot, v.iter());
            assert_eq!(aabb, AABB2::new(1f32, 0f32, 2f32, 1f32));
        }

        {
            let mot = Vector2::new(-4f32, 1f32);
            aabb.slide(mot, v.iter());
            assert_eq!(aabb, AABB2::new(1f32, 1f32, 2f32, 2f32));
        }
    }

    fn data() -> Vec<AABB2> {
        vec![AABB2::new(2f32, 0f32, 3f32, 1f32),
             AABB2::new(0f32, 1f32, 1f32, 2f32),
             AABB2::new(3f32, 1f32, 4f32, 2f32),
             AABB2::new(1f32, 2f32, 2f32, 3f32),

             AABB2::new(2f32, 0f32, 3f32, 1f32),
             AABB2::new(0f32, 1f32, 1f32, 2f32),
             AABB2::new(3f32, 1f32, 4f32, 2f32),
             AABB2::new(1f32, 2f32, 2f32, 3f32)]
    }

    #[bench]
    fn bench_test(b: &mut Bencher) {
        let v = data();
        b.iter(|| {
            let mut aabb = AABB2::new(0f32, 0f32, 1f32, 1f32);

            {
                let mot = black_box(Vector2::new(2f32, 0f32));
                aabb.slide(mot, v.iter());
            }

            {
                let mot = black_box(Vector2::new(-4f32, 1f32));
                aabb.slide(mot, v.iter());
            }
        });
    }
}