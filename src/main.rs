#![feature(gen_blocks)]
#![feature(trait_upcasting)]
#![feature(let_chains)]

use std::any::Any;
use std::fmt::Display;
use std::ops::Deref;

mod bot;
mod help;
mod ping;
mod rps;
mod tic_tac_toe;
mod basic_functions;

pub async fn idk(){

}
gen fn bruh() -> i32{
    yield 4;
    yield 5;
    yield 6
}

#[tokio::main]
async fn main() {
    bot::start().await;
}
