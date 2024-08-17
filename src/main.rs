fn main() {}

pub trait System: 'static {
    type In;
}

impl<'a> System for &'a () {
    type In = &'a ();
}

trait CloneSystem: System {
    fn dyn_clone(&self) -> Box<dyn CloneSystem<In = Self::In>>;
}

impl<I> Clone for Box<dyn CloneSystem<In = I>> {
    fn clone(&self) -> Self {
        self.dyn_clone()
    }
}
