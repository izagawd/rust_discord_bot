#![feature(gen_blocks)]
#![feature(trait_upcasting)]
#![feature(unboxed_closures,fn_traits)]
#![feature(specialization)]
#[allow(static_mut_refs)]
use std::any::{Any, TypeId};
use std::fmt::Display;
use std::mem;
use std::ops::Deref;
use std::rc::Rc;
use bevy_reflect::{reflect_trait, FromType, Reflect, TypeData, TypeRegistry};
use tokio::task::id;
use tokio::time::interval;
use crate::useful_funcs::{get_reflect_type, register_type};

mod bot;
mod help;
mod ping;
mod rps;
mod useful_funcs;

#[derive(Reflect)]
#[reflect(MyTrait)]
#[reflect(MyOtherTrait)]
struct MyType {
    value: String,
}


impl MyTrait for MyType {
    fn do_thing(&self) -> String {
        format!("{} World!", self.value)
    }
}


#[reflect_trait]
pub trait MyTrait {
    fn do_thing(&self) -> String;
}
#[reflect_trait]
pub trait MyOtherTrait {
    fn do_thing(&self) -> String;
}
impl MyOtherTrait for MyType {
    fn do_thing(&self) -> String {
        String::from("dd")
    }
}
#[tokio::main]
async fn main() {
    let idk : &dyn Reflect = &MyType{ value: "".to_string() };
    register_type::<MyType>();
    let gotten = get_reflect_type::<ReflectMyOtherTrait>(idk);

    if(gotten.is_ok()){
       let unwrapped = gotten.unwrap();
       let casted = unwrapped.get(idk);
        println!("{}", casted.unwrap().do_thing());
    } else{
        println!("aww");
    }

}
// This works because the #[reflect(MyTrait)] we put on MyType informed the Reflect derive to insert a new instance
// of ReflectDoThing into MyType's registration. The instance knows how to cast &dyn Reflect to &dyn DoThing, because it
// knows that &dyn Reflect should first be downcasted to &MyType, which can then be safely casted to &dyn DoThing
