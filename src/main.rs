#![feature(gen_blocks)]
#![feature(trait_upcasting)]
#![feature(let_chains)]

use std::any::Any;
use std::fmt::Display;
use std::ops::Deref;
use std::rc::Weak;

mod bot;
mod help;
mod ping;
mod rps;
mod tic_tac_toe;
mod basic_functions;


struct Person{
    movement_speed: i32,

}
impl Person{
    fn movement(&self) {
        println!("I moved with a speed of {}",self.movement_speed);
    }

    fn move_then_rest(&self){
        self.movement();
        println!("that was a lot of movement. im resting now")
    }
}
struct Superman{
    person: Person,
}
impl Superman {
    fn new() -> Superman{
        Superman{person: Person{movement_speed: 1}}
    }
    fn movement(&self){
        println!("I am flying with a speed of {}",self.person.movement_speed);
    }
    fn move_then_rest(&self){
        self.person.move_then_rest();
    }
}
#[tokio::main]
async fn main() {
    bot::start().await;

}
