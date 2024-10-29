use bevy_reflect::{Reflect, TypeRegistry};
use std::any::Any;
use std::sync::{LazyLock, RwLock};


pub static REGISTRAR
: LazyLock<RwLock<TypeRegistry>> = LazyLock::new(||{


    RwLock::new(TypeRegistry::default())
});
pub fn get_reflect_type<TReflectType: std::marker::Send + std::marker::Sync + std::clone::Clone + 'static>(to_reflect: &dyn Reflect) -> Result<TReflectType, String>{

    let bru =REGISTRAR.read();
    if bru.is_ok(){
        let data = bru.as_ref().unwrap().get_type_data::<TReflectType>((to_reflect as &dyn Any).type_id());
        if data.is_some(){
            return Ok(data.unwrap().clone());
        }else{
            return Err(String::from("Inputted type not registered"))
        }
    } else{
        return Err(String::from("Another thread has caused an error"));
    }

}
pub fn register_type<T: Sized + bevy_reflect::GetTypeRegistration>(){
    let the_register = REGISTRAR.write();
    if the_register.is_ok(){

        the_register.unwrap().register::<T>();

    } else{
        panic!("This thread has caused an error. this is not supposed to happen")
    }
}
