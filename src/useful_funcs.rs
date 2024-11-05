use bevy_reflect::{Reflect, TypeRegistry};
use std::any::Any;
use std::sync::{LazyLock, RwLock};


pub static REGISTRAR
: LazyLock<RwLock<TypeRegistry>> = LazyLock::new(||{


    RwLock::new(TypeRegistry::default())
});
pub fn get_reflect_type<TReflectType: std::marker::Send + std::marker::Sync + std::clone::Clone
+ 'static>(to_reflect: &dyn Reflect) -> Result<TReflectType, String>{


    if let Ok(ref bru) = REGISTRAR.read(){
        let data = bru.get_type_data::<TReflectType>((to_reflect as &dyn Any).type_id());
        if let Some(ref to_reflect) = data{
            return Ok((*to_reflect).clone());
        }else{
            return Err(String::from("Inputted trait not registered for inputted object"))
        }
    } else{
        return Err(String::from("Another thread has caused an error"));
    }

}
pub fn register_type<T: Sized + bevy_reflect::GetTypeRegistration>(){
    if let Ok(ref mut the_register) = REGISTRAR.write(){
        the_register.register::<T>()
    } else{
        panic!("This thread has caused an error. this is not supposed to happen")
    }

}
