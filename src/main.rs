#![feature(gen_blocks)]
#![feature(trait_upcasting)]
#![feature(let_chains)]
#![feature(async_iterator)]
#![feature(allocator_api)]

use std::alloc::Global;
use std::any::Any;
use std::async_iter::AsyncIterator;
use std::fmt::{Debug, Display};

use image::PixelWithColorType;
use sea_orm::{ActiveModelTrait, EntityTrait};
use std::ops::{Add, Deref, DerefMut, Index};
use std::rc::Rc;
use std::sync::{LazyLock, OnceLock, RwLock};
use imageproc::definitions::Clamp;
use mimalloc::MiMalloc;
use tokio::time::Instant;
use crate::functions::basic_functions::random_choice;

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

#[inline(never)]
fn do_shit() -> Box<dyn Any>{
    let curr = Instant::now();
    let boxed = Box::new([0u8;350]);
    let some = Instant::now();


    println!("{}",(some - curr).as_nanos());
    boxed
}

#[tokio::main]
async fn main() {
    for i in 0..11000{
        do_shit();
    }

}