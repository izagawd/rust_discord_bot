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
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::async_iter::AsyncIterator;
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

#[tokio::main]
async fn main() {
    bot::start().await;
}