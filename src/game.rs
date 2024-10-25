use std::cell::RefCell;
use std::rc::Rc;
use crate::world::World;
use crate::za_muts::RcMut;

trait Game{
    fn render(&self);
    fn get_world(&self) -> Option<RcMut<dyn World>>;
}

struct GameImpl{
    world: Option<Rc<RefCell<dyn World>>>
}
impl Game for GameImpl{
    fn render(&self) {
        todo!()
    }

    fn get_world(&self) -> Option<RcMut<dyn World>>{
        self.world.clone()
    }
}
impl GameImpl{

    fn new() -> GameImpl{
        GameImpl{world:None}
    }
}