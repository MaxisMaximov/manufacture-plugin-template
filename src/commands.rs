// # Commands module
// This is where you can define Commands that perform operations on World at root level
use super::*;

pub struct SampleCommand(usize);
impl Command for SampleCommand{
    fn execute(&mut self, world: &mut World) {
        world.despawn(self.0);
    }
}