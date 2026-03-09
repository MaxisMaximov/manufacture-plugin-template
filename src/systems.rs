// # Systems module
// This is where you can define Systems that will be responsible for running your app
use super::*;
use comp::SampleComponent;

pub struct SampleSystem;
impl System for SampleSystem{
    type Data<'a> = Query<&'a SampleComponent, ()>;
    const ID: &'static str = "PositionSystem";

    fn new() -> Self { Self }

    fn execute(&mut self, data: Request<'_, Self::Data<'_>>) {
        for sample_comp in data.iter(){
            println!("Sample component val: {}", sample_comp.a)
        }
    }
}