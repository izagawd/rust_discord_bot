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
mod help;
mod ping;
mod rps;
mod tic_tac_toe;
mod basic_functions;
mod image_functions;
mod tic_tac_toe_simulator;


#[tokio::main]
async fn main() {

}