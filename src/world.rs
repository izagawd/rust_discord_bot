use std::cell::RefCell;
use std::rc::{Rc, Weak};
use gl::types::GLubyte;
use syn::token::Ge;
use crate::GameNode::GameNode;



trait Spawner{
    fn spawn_game_node<TGameNode: GameNode + Sized + 'static>(&mut self) -> Weak<RefCell<TGameNode>>;
}
impl<TWorld: World> Spawner for TWorld {
    fn spawn_game_node<TGameNode: GameNode + Sized + 'static>(&mut self) -> Weak<RefCell<TGameNode>> {
       let nodes = &mut self.get_world_gen_mut().nodes;
       let spawned_game_node = Rc::new(RefCell::new(TGameNode::new()));
        nodes.push(spawned_game_node.clone());
        Rc::downgrade(&spawned_game_node)


    }
}
pub trait World{


    fn get_world_gen_mut(&mut self) -> &mut GenWorld;
}
pub struct GenWorld{
    nodes: Vec<Rc<RefCell<dyn GameNode>>>
}
impl World for GenWorld{
    fn get_world_gen_mut(&mut self) -> &mut GenWorld {
        self
    }
}

