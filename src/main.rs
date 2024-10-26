#![feature(gen_blocks)]
#![feature(trait_upcasting)]
use std::alloc::alloc;
use std::any::Any;
use std::cell::RefCell;
use std::fmt::{Debug, Pointer};
use std::ops::Deref;
use std::rc::Rc;
use std::sync::Arc;
use crate::scene_game_node::SceneGameNode;
use crate::za_muts::RcMut;

mod scene_game_node;
mod game_node;
mod rotation;
mod game;
mod transform;
mod world;
mod za_muts;
mod mesh_game_node;
mod color;
mod intertraitable;




struct Lily{
pub gay : i32
}

trait Namer{

}
impl Namer for Lily{}
trait NamerStuff{
    fn print_name(&self);
}

struct Droppable{

}

fn main() {
    {
        Droppable{};
    }
}