#![feature(gen_blocks)]
#![feature(trait_upcasting)]
#![feature(let_chains)]
#![feature(async_iterator)]
#![feature(allocator_api)]
#![feature(test)]
extern crate test;
use image::PixelWithColorType;
use imageproc::definitions::Clamp;
use sea_orm::{ActiveModelTrait, EntityTrait};
use std::any::Any;
use std::async_iter::AsyncIterator;
use std::cell::{Cell, RefCell};
use std::fmt::{Debug, Display};
use std::ops::{Add, Deref, DerefMut, Index};

mod bot;
mod functions{
    pub mod basic_functions;
    pub mod image_functions;
}
mod tic_tac_toe_simulator;
mod character_model;
mod commands{
    pub mod help;
    pub mod ping;
    pub mod rps;
    pub mod tic_tac_toe;
}
trait Character{
    fn name(&self) -> &'static str;
    fn discriminator(&self) -> i32;
}
struct Lily;

impl Character for Lily {
    fn name(&self) -> &'static str {
        "Lily"
    }
    fn discriminator(&self) -> i32{
        4
    }
}



#[tokio::main]
async fn main() {
    let bruh = Cell::new(5);
    println!("{}",size_of::<RefCell<bool>>());
}