use std::cell::RefCell;
use std::os::raw::c_int;
use std::rc::Rc;
use nalgebra::coordinates::M4x4;
use nalgebra::{Quaternion, Vector3};

pub struct  Transform<T>{
    translation: Vector3<T>,
    scale: Vector3<T>,
    rotation: Vector3<T>,
}
impl <T> Transform<T>{
    pub fn new(translation: Vector3<T>, scale: Vector3<T>, rotation: Vector3<T>) -> Transform<T>{
        Transform{translation, scale, rotation}
    }
}
impl<T: Copy> Clone for Transform<T> {
    fn clone(&self) -> Self {
        Transform::new(self.translation, self.scale, self.rotation)
    }
}

impl<T: Copy> Copy for Transform<T>{

}
fn idk(){

    let mut idk = Transform::new(
        Vector3::new(1.0, 1.0, 1.0), Vector3::new(1.0, 1.0, 1.0), Vector3::new(1.0, 1.0, 1.0));
    let mut bruh = idk;
    println!("{}", idk.rotation.x);
}