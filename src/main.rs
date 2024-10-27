#![feature(gen_blocks)]
#![feature(trait_upcasting)]
#![feature(unboxed_closures,fn_traits)]

mod bot;
mod help;


use serenity::client::{ EventHandler};






#[tokio::main]
async fn main() {
    bot::start().await;
}
