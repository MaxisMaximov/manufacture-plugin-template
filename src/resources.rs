// # Resource module
// This is where you can define global Resources to be used by Systems
use super::*;

pub struct SampleResource{
    pub a: Vector2,
    pub b: Vector2
}
impl Resource for SampleResource{
    const ID: &'static str = "SampleResource";

    fn new() -> Self {
        Self{
            a: Vector2 { x: 5.0, y: 5.0 },
            b: Vector2 { x: 10.0, y: 10.0 },
        }
    }
}