pub mod vector {
	
	use auto_ops::*;
	use std::ops::{Add, Mul};
	
	#[derive(Debug)]
	pub struct Vec3<T>
	{
		pub x: T,
		pub y: T,
		pub z: T,
	}
	
	impl<T> Vec3<T>
	where
		T: Mul<Output=T> + Add<Output=T> + Copy
	{
		pub fn new(x: T, y: T, z: T) -> Vec3<T> {
			Vec3::<T> { x: x, y: y, z: z }
		}
		
		pub fn dot(&self, other: &Vec3<T>) -> T {
			self.x * other.x +
			self.y * other.y +
			self.z * other.z
		}
	}
	
	impl_op_ex!(- |a: &Vec3<f64>, b: &Vec3<f64>| -> Vec3<f64> { Vec3::<f64>::new(a.x - b.x, a.y - b.y, a.z - b.z) });
	
	/*
	impl<T> Sub for Vec3<T>
	where T: Sub<Output=T> + Copy
	{
		type Output = Self;
		
		fn sub(&self, rhs: &Vec3<T>) -> Self {
			Self {
				x: self.x - rhs.x,
				y: self.y - rhs.y,
				z: self.z - rhs.z,				
			}
		}
	}
	*/
}

