#![feature(gen_blocks)]
#![feature(trait_upcasting)]
#![feature(unboxed_closures,fn_traits)]
use bevy_reflect::{FromType, GetField, Reflect, TypeData};
#[allow(static_mut_refs)]
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
