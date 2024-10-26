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
    fn get_game_node_impl(&self) -> &GameNodeImpl {
        self
    }

    fn get_game_node_impl_mut(&mut self) -> &mut GameNodeImpl {
        self
    }

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

    fn get_game_node_impl(&self) -> &GameNodeImpl;
    fn get_game_node_impl_mut(&mut self) -> &mut GameNodeImpl;
    fn get_children(&self) -> &Vec<RcMut<dyn GameNode>>{
        self.get_game_node_impl().get_children()
    }
    fn set_world(&mut self, world: Option<WeakMut<dyn World>>){
        self.get_game_node_impl_mut().set_world(world);
    }
    fn get_parent(&self) -> Option<WeakMut<dyn GameNode>>{
        self.get_game_node_impl().get_parent()
    }
    fn new() -> Self where Self: Sized;
}