use std::ops::{Add, AddAssign};
use nalgebra::{Quaternion, RealField, SimdValue, UnitQuaternion};



#[derive(Copy,Clone)]
pub struct Rotation<T>{
    roll : T,
    pitch : T,
    yaw : T,
}

impl<T> Rotation<T>{
    pub fn new(roll : T, pitch : T, yaw : T) -> Self {
        Rotation{roll, pitch, yaw}

    }
    pub fn as_unit_quaternion(&self) -> UnitQuaternion<T> where
    T: Copy + SimdValue + RealField{

        UnitQuaternion::from_euler_angles(self.roll,self.pitch,self.yaw)
    }
    pub fn from_unit_quaternion(quaternion : UnitQuaternion<T>) -> Rotation<T>
    where T: RealField{
        let (roll,pitch,yaw) =quaternion.euler_angles();
        Rotation{
            roll,
            pitch,
            yaw,
        }
    }
}
fn idk(){



}