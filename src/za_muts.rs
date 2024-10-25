use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub type WeakMut<T> = Weak<RefCell<T>>;
pub type RcMut<T> = Rc<RefCell<T>>;