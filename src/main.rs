#![feature(gen_blocks)]
#![feature(trait_upcasting)]
#![feature(let_chains)]
#![feature(async_iterator)]

use std::any::{Any, TypeId};
use std::async_iter::AsyncIterator;
use std::cell::RefCell;
use std::fmt::{Debug, Display};

use image::PixelWithColorType;
use sea_orm::{ActiveModelTrait, EntityTrait};
use std::ops::{Add, Deref, Index};
use crate::character_model::{create_character_from, register_character, Lily};

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
    register_character::<Lily>();



    let possible_lily = create_character_from(1).unwrap();

    println!("{}",possible_lily.deref().type_id() ==  TypeId::of::<Lily>());
    return;
    bot::start().await;

}