use std::cell::RefCell;
use std::rc::{Rc, Weak};
use crate::world::World;
use crate::za_muts::*;




pub struct GameNodeImpl {
    parent: Option<WeakMut<dyn GameNode>>,
    children: Vec<RcMut<dyn GameNode>>,
    world: Option<WeakMut<dyn World>>,
}
impl GameNode for GameNodeImpl {
    fn get_children(&self) -> &Vec<Rc<RefCell<dyn GameNode>>>{
        &self.children
    }

    fn set_world(&mut self, world: Option<WeakMut<dyn World>>) {

        self.world = world;
    }

    fn get_parent(&self) -> Option<Weak<RefCell<dyn GameNode>>> {
        self.parent.clone()
    }

    fn new() -> Self
    where
        Self: Sized
    {

            Self{
                parent: None,
                children: Vec::new(),
                world: None,
            }

    }
}


impl GameNodeImpl {

}

pub trait GameNode{

    fn get_children(&self) -> &Vec<RcMut<dyn GameNode>>;
    fn set_world(&mut self, world: Option<WeakMut<dyn World>>);
    fn get_parent(&self) -> Option<WeakMut<dyn GameNode>>;
    fn new() -> Self where Self: Sized;
}