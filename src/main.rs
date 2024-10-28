#![feature(gen_blocks)]
#![feature(trait_upcasting)]
#![feature(unboxed_closures,fn_traits)]

use std::mem;

mod bot;
mod help;
mod ping;
mod rps;


#[tokio::main]
async fn main() {

    bot::start().await;
}
