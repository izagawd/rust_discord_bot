#![feature(gen_blocks)]
#![feature(trait_upcasting)]
#![feature(let_chains)]
#![feature(async_iterator)]

use std::any::Any;
use std::async_iter::AsyncIterator;
use std::cell::{Cell, RefCell};
use std::fmt::Display;

use std::ops::{Add, Deref};
use std::rc::{Rc, Weak};
use std::sync::{Arc, LazyLock, Mutex, RwLock};
use std::{sync, thread};
use crate::basic_functions::random_choice;

mod bot;
mod help;
mod ping;
mod rps;
mod tic_tac_toe;
mod basic_functions;


#[tokio::main]
async fn main() {

}