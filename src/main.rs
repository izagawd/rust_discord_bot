#![feature(gen_blocks)]
#![feature(trait_upcasting)]

use std::any::Any;
use std::fmt::Display;
use std::ops::Deref;

mod bot;
mod help;
mod ping;
mod rps;



#[tokio::main]
async fn main() {

    bot::start().await;
}
