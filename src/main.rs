#![feature(gen_blocks)]
#![feature(trait_upcasting)]
#![feature(let_chains)]
#![feature(async_iterator)]

use std::any::Any;
use std::async_iter::AsyncIterator;
use std::fmt::Display;

use image::PixelWithColorType;
use std::ops::{Add, Deref, Index};
use sea_orm::{ActiveModelTrait, DeriveEntityModel, EntityTrait};
use sea_orm::ActiveValue::Set;

mod bot;
mod functions{
    pub mod basic_functions;
    pub mod image_functions;
}
mod tic_tac_toe_simulator;
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