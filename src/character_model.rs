use std::any::Any;
use std::cell::Cell;
use std::marker::PhantomData;
use std::rc::Rc;
use std::sync::RwLock;

struct CharacterModel{
    id: i64,
    level: i32,
    discriminator: u32
}


static registered_characters: RwLock<Vec<Box<dyn CharacterRegisterer + Send + Sync>>> = RwLock::new(Vec::new());
pub fn register_character<T: Character + 'static>() -> Result<(),&'static str>{

    match   registered_characters.write(){
        Ok(mut success) => {
            let created_discriminator = T::new().discriminator();
            success.push(Box::new(CharacterRegistererStruct::<T>{discriminator: created_discriminator,phantom_data: PhantomData{}}));
            return Ok(());
        }
        Err(_) => {
            return Err("Poison error has occured");
        }
    }


}

pub fn create_character_from(discriminator: i32) -> Result<Rc<dyn Character>,&'static str>{
    match registered_characters.read() {
        Ok(success) => {
            let gotten_option_char =success
                .iter()
                .find(move |x|x.character_discriminator() as i32 == discriminator );
            match gotten_option_char {
                None => { Err("Character not found") },
                Some(gotten_char) => {
                    return Ok(gotten_char.create_character())
                }
            }
        }
        Err(_) => { Err("Poison error has occured") }
    }
}
trait CharacterRegisterer{
    fn character_discriminator(&self) -> u32;
    fn create_character(&self) -> Rc<dyn Character>;
}
struct CharacterRegistererStruct<T: Character + 'static>{
    discriminator: u32,
    phantom_data: PhantomData<fn() -> T>
}
impl<T: Character + 'static> CharacterRegisterer for CharacterRegistererStruct<T>{
    fn character_discriminator(&self) -> u32 {
        return self.discriminator;
    }

    fn create_character(&self) -> Rc<dyn Character>{
       T::new()
    }
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
pub struct Lily{
    character_data: CharacterData
}

impl Character for Lily{
    fn discriminator(&self) -> u32 {
        1
    }
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
pub trait Character : Any{
    fn discriminator(&self) -> u32;
    fn character_data(&self) -> &CharacterData;
    fn new() -> Rc<Self> where Self : Sized;
}