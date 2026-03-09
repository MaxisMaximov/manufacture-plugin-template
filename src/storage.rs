// # Storage module
// This is where you can define custom Storage types for Components, in case current implementations don't suffice
use super::*;

pub struct SampleStorage<C: Component>{
    inner: [Option<C>; 20]
}
impl<C: Component + Copy> Storage<C> for SampleStorage<C>{
    fn new() -> Self {
        Self{
            inner: [None; 20],
        }
    }

    fn insert(&mut self, id: usize, comp: C) {
        if id >= 20{
            return
        }
        self.inner[id] = Some(comp)
    }
    fn remove(&mut self, id: &usize) {
        self.inner[*id] = None
    }

    fn get(&self, id: &usize) -> Option<&C> {
        if *id >= 20{
            return None
        }
        self.inner[*id].as_ref()
    }
    fn get_mut(&mut self, id: &usize) -> Option<&mut C> {
        if *id >= 20{
            return None
        }
        self.inner[*id].as_mut()
    }

    fn len(&self) -> usize {
        20
    }
}