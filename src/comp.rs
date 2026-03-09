// # Components module
// This is where you can define Components *(data)* for Entities to have
use super::*;
use storage::SampleStorage;

#[derive(Clone, Copy)]
pub struct SampleComponent{
    pub a: u8,
    pub b: Vector2
}
impl Component for SampleComponent{
    type STORAGE = SampleStorage<Self>;

    const ID: &'static str = "SampleComponent";
}