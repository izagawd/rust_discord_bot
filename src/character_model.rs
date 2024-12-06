use std::any::TypeId;
use std::cell::{Cell, RefCell};
use std::marker::PhantomData;
use std::rc::Rc;
use std::sync::LazyLock;

struct CharacterModel{
    id: i64,
    level: i32,
    discriminator: u32
}




struct CharacterData{

    health: Cell<i32>,
}
impl CharacterData{
    fn new() -> CharacterData{
        CharacterData{
            health: Cell::new(0),
        }
    }
}
trait CharacterExt {
    fn get_health(&self) -> i32;
    fn set_health(&self, health: i32);

}
struct Lily{
    character_data: CharacterData
}

impl Character for Lily{
    fn character_data(&self) -> &CharacterData {
        &self.character_data
    }

    fn new() -> Rc<Self>
    where
        Self: Sized
    {
        Rc::new(Self{
            character_data: CharacterData::new()
        })
    }
}
impl<T: Character> CharacterExt for T{
    fn get_health(&self) -> i32{
        return self.character_data().health.get();
    }
    fn set_health(&self, health: i32){
        self.character_data().health.set(health);
    }

}
trait Character{
    fn character_data(&self) -> &CharacterData;
    fn new() -> Rc<Self> where Self : Sized;
}