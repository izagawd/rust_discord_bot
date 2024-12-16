#![feature(gen_blocks)]
#![feature(trait_upcasting)]
#![feature(let_chains)]
#![feature(async_iterator)]
#![feature(allocator_api)]
#![feature(test)]
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

#[tokio::main]
async fn main() {


}