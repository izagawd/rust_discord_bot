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
use std::rc::Rc;

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

struct Hey{
    kk: Cell<i32>,
}

impl Hey{

    fn duplicate_kk(&self) {
        self.kk.set(self.kk.get() *2);
    }
    fn get_kk(&self) -> i32 {
        self.kk.get()
    }
}
#[tokio::main]
async fn main() {
    let created = Rc::new(Hey{kk: Cell::new(1)});
    created.duplicate_kk();
    created.duplicate_kk();
    print!("{}",created.get_kk());
}