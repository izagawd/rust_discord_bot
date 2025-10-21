use image::PixelWithColorType;
use imageproc::definitions::Clamp;
use sea_orm::{ActiveModelTrait, EntityTrait};
use std::any::Any;

use std::fmt::{Debug, Display};
use std::ops::{Add, Deref, DerefMut, Index};

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
    bot::start().await;
}