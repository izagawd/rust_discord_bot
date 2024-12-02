#![feature(gen_blocks)]
#![feature(trait_upcasting)]
#![feature(let_chains)]

use std::any::Any;
use std::collections::HashMap;
use std::ffi::CString;
use std::fmt::Display;
use std::ops::Deref;
use crate::basic_functions::random_choice;
use crate::tic_tac_toe::GameSimulator;

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
    GameSimulator::default().start();
}
