#![feature(gen_blocks)]
#![feature(trait_upcasting)]
use bevy_reflect::{FromType, GetField, Reflect, TypeData};

use std::any::Any;
use std::fmt::Display;
use std::ops::Deref;

mod bot;
mod help;
mod ping;
mod rps;
mod useful_funcs;


#[tokio::main]
async fn main() {

    bot::start().await;
}
