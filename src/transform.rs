use nalgebra::{ Vector3};
use crate::rotation::Rotation;


pub struct  Transform<T>{
    pub translation: Vector3<T>,
    pub scale: Vector3<T>,
    pub rotation: Rotation<T>,
}

impl <T> Transform<T>{

    pub fn new(translation: Vector3<T>, scale: Vector3<T>, rotation: Rotation<T>) -> Transform<T>{
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

