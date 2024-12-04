#![feature(gen_blocks)]
#![feature(trait_upcasting)]
#![feature(let_chains)]
#![feature(async_iterator)]

use std::any::Any;
use std::async_iter::AsyncIterator;
use std::cell::{Cell, RefCell};
use std::fmt::Display;

use std::ops::{Add, Deref, Index};
use std::rc::{Rc, Weak};
use std::sync::{Arc, LazyLock, Mutex, RwLock};
use std::{sync, thread};
use std::path::Path;
use image::DynamicImage::{ImageRgb16, ImageRgb32F};
use image::{ImageBuffer, ImageReader, PixelWithColorType, Rgb, Rgba, RgbaImage};
use crate::basic_functions::random_choice;

mod bot;
mod help;
mod ping;
mod rps;
mod tic_tac_toe;
mod basic_functions;
mod image_functions;
mod tic_tac_toe_simulator;

#[derive(Clone,Copy)]
struct Goku {
    power_level: i32
}
impl Add for Goku {
    type Output = Goku;

    fn add(self, rhs: Self) -> Self::Output {
        Goku{power_level: self.power_level + rhs.power_level}
    }
}

fn bruh<T>(input: T)  -> T where T : Add<T, Output = T> + Copy{
    return input + input;
}
#[tokio::main]
async fn main() {

    bot::start().await;

}