use std::ops::{Add, Sub, Mul};

pub struct Vec2{
	pub x : f32,
	pub y : f32
}

impl Vec2 {

	#[allow(dead_code)]
	pub fn new (x: f32, y: f32) -> Vec2 {
		Vec2 {x: x, y: y}
	}
}
//-----------------------------------------------------------------------------
pub struct Vec3{
	pub x : f32,
	pub y : f32,
	pub z : f32
}

impl Vec3 {

	#[allow(dead_code)]
	pub fn new (x: f32, y: f32, z: f32) -> Vec3 {
		Vec3 {x: x, y: y, z: z}
	}

	#[allow(dead_code)]
	pub fn length(&self) -> f32 {
		(self.x*self.x + self.y*self.y + self.z*self.z).sqrt()
	}

	#[allow(dead_code)]
	pub fn normalize(&mut self){
		let l = self.length();
		self.x /= l;
		self.y /= l; 
		self.z /= l;
	}

	#[allow(dead_code)]
	pub fn normalized(&self) -> Vec3{
		let l = self.length();
		Vec3 { x: self.x / l, y: self.y / l, z: self.z / l}
	}

	#[allow(dead_code)]
	pub fn scale(&mut self, s : f32){
		self.x *= s;
		self.y *= s;
		self.z *= s;
	}

	#[allow(dead_code)]
	pub fn scaled(&self, s : f32) -> Vec3{
		Vec3 { x: self.x * s, y: self.y * s, z: self.z * s}
	}
}

impl Add for Vec3 {
	type Output = Vec3;

	fn add(self, other: Vec3) -> Vec3 {
		Vec3 { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
	}
}

impl<'a> Add for &'a Vec3 {
	type Output = Vec3;

	fn add(self, other: &'a Vec3) -> Vec3 {
		Vec3{x: self.x + other.x, y: self.y + other.y, z: self.z + other.z}
	}
}

impl Sub for Vec3 {
	type Output = Vec3;

	fn sub(self, other: Vec3) -> Vec3 {
		Vec3 { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z }
	}
}

impl<'a> Sub for &'a Vec3 {
	type Output = Vec3;

	fn sub(self, other: &'a Vec3) -> Vec3 {
		Vec3{x: self.x - other.x, y: self.y - other.y, z: self.z - other.z}
	}
}

#[allow(dead_code)]
pub fn dot_product(v1 : Vec3, v2 : Vec3) -> f32{
	v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
}

#[allow(dead_code)]
pub fn cross_product(v1 : Vec3, v2 : Vec3) -> (Vec3){
	Vec3 { x: v1.y * v2.z - v1.z * v2.y,
	       y: v1.z * v2.x - v1.x * v2.z,
	       z: v1.x * v2.y - v1.y * v2.x }
}

//-----------------------------------------------------------------------------

pub struct Vec4{
	pub x : f32,
	pub y : f32,
	pub z : f32,
	pub w : f32
}

impl Vec4 {

	#[allow(dead_code)]
	pub fn new (x: f32, y: f32, z: f32, w: f32) -> Vec4 {
		Vec4 {x: x, y: y, z: z, w : w}
	}
}

//-----------------------------------------------------------------------------

pub struct Mat4x4 {
	pub m : [f32; 16]
}

impl Mat4x4{
	#[allow(dead_code)]
	pub fn zero () -> Mat4x4 {
		Mat4x4 {m : [0.0; 16]}
	}

	#[allow(dead_code)]
	pub fn ident () ->Mat4x4 {
		Mat4x4 {m : [1.0, 0.0, 0.0, 0.0,
		             0.0, 1.0, 0.0, 0.0,
		             0.0, 0.0, 1.0, 0.0,
		             0.0, 0.0, 0.0, 1.0,]}
	}

	#[allow(dead_code)]
	pub fn x_rotation (a : f32) -> Mat4x4{
		let c = a.cos();
		let s = a.sin();
		Mat4x4 {m :
		  [1.0, 0.0, 0.0, 0.0,
		   0.0,   c,  -s, 0.0,
		   0.0,   s,   c, 0.0,
		   0.0, 0.0, 0.0, 1.0,]}
	}

	#[allow(dead_code)]
	pub fn y_rotation (a : f32) -> Mat4x4{
		let c = a.cos();
		let s = a.sin();
		Mat4x4 {m :
		  [  c, 0.0,  -s, 0.0,
		   0.0, 1.0, 0.0, 0.0,
		     s, 0.0,   c, 0.0,
		   0.0, 0.0, 0.0, 1.0,]}
	}

	#[allow(dead_code)]
	pub fn z_rotation (a : f32) -> Mat4x4{
		let c = a.cos();
		let s = a.sin();
		Mat4x4 {m :
		  [  c,  -s, 0.0, 0.0,
		     s,   c, 0.0, 0.0,
		   0.0, 0.0, 1.0, 0.0,
		   0.0, 0.0, 0.0, 1.0,]}
	}

	#[allow(dead_code)]
	pub fn scale (x : f32, y : f32, z : f32) -> Mat4x4{
		Mat4x4 {m :
		[    x, 0.0, 0.0, 0.0,
		   0.0,   y, 0.0, 0.0,
		   0.0, 0.0,   z, 0.0,
		   0.0, 0.0, 0.0, 1.0,]}
	}

	#[allow(dead_code)]
	pub fn translation (x : f32, y : f32, z : f32) -> Mat4x4{
		Mat4x4 {m :
		[  1.0, 0.0, 0.0,   x,
		   0.0, 1.0, 0.0,   y,
		   0.0, 0.0, 1.0,   z,
		   0.0, 0.0, 0.0, 1.0,]}
	}

	#[allow(dead_code)]
	pub fn retro_proj (d : f32) -> Mat4x4{
		Mat4x4 {m :
		[  1.0, 0.0, 0.0, 0.0,
		   0.0, 1.0, 0.0, 0.0,
		   0.0, 0.0, 1.0, 0.0,
		   0.0, 0.0,   d, 1.0,]}
	}

	pub fn apply ( &self, inp: &Vec3) -> Vec3{

		let w = inp.x * self.m[12] + inp.y * self.m[13] + inp.z * self.m[14] + self.m[15];
		let inv_v = 1.0 / w;

		Vec3 {
			x : inv_v * (inp.x * self.m[0] + inp.y * self.m[1] + inp.z * self.m[2]  + self.m[3]),
			y : inv_v * (inp.x * self.m[4] + inp.y * self.m[5] + inp.z * self.m[6]  + self.m[7]),
			z : inv_v * (inp.x * self.m[8] + inp.y * self.m[9] + inp.z * self.m[10] + self.m[11]),
		}
	}
}

impl Add for Mat4x4 {
	type Output = Mat4x4;

	fn add(self, other: Mat4x4) -> Mat4x4 {
		let mut out = Mat4x4::zero();

		for n in 0..16{
			out.m[n] = self.m[n] + other.m[n];
		}

		return out;
	}
}

impl<'a> Add for &'a Mat4x4 {
	type Output = Mat4x4;

	fn add(self, other: &'a Mat4x4) -> Mat4x4 {
		let mut out = Mat4x4::zero();

		for n in 0..16{
			out.m[n] = self.m[n] + other.m[n];
		}

		return out;
	}
}

impl Sub for Mat4x4 {
	type Output = Mat4x4;

	fn sub(self, other: Mat4x4) -> Mat4x4 {
		let mut out = Mat4x4::zero();

		for n in 0..16{
			out.m[n] = self.m[n] - other.m[n];
		}

		return out;
	}
}

impl<'a> Sub for &'a Mat4x4 {
	type Output = Mat4x4;

	fn sub(self, other: &'a Mat4x4) -> Mat4x4 {
		let mut out = Mat4x4::zero();

		for n in 0..16{
			out.m[n] = self.m[n] + other.m[n];
		}

		return out;
	}
}

impl Mul for Mat4x4 {
	type Output = Mat4x4;

	fn mul(self, other: Mat4x4) -> Mat4x4 {
		let mut out = Mat4x4::zero();

		for n in 0..4{
			for m in 0..4{
				let mut s : f32 = 0.0;
				for k in 0..4{
					s += self.m[n + k*4] * other.m[k + m*4];
				}
				out.m[n + m*4] = s
			}
		}

		return out;
	}
}

impl<'a> Mul for &'a Mat4x4 {
	type Output = Mat4x4;

	fn mul(self, other: &'a Mat4x4) -> Mat4x4 {
		let mut out = Mat4x4::zero();

		for n in 0..4{
			for m in 0..4{
				let mut s : f32 = 0.0;
				for k in 0..4{
					s += self.m[n + k*4] * other.m[k + m*4];
				}
				out.m[n + m*4] = s
			}
		}

		return out;
	}
}


//--------------------------------------------------------------

#[cfg(test)]
mod tests {
	use super::*;
	use std::f32::consts::PI;

	#[test]
	fn test_matrix_mul() {

		let i1 = Mat4x4::ident();
		let i2 = Mat4x4::ident();

		let i3 = &i1 * &i2;
		for n in 0..16{
			assert_eq!(i3.m[n], i1.m[n]);
		}

		let r1 = Mat4x4::x_rotation(0.1);
		let r2 = Mat4x4::x_rotation(-0.1);
		let r3 = &r1 * &r2;

		for n in 0..16{
			let e = r3.m[n] - i1.m[n];
			assert!(e.abs()< 0.001);
		}

		let v1 = Vec3::new(0.0, 1.0, 0.0);
		let v2 = Mat4x4::z_rotation(PI / 2.0).apply(&v1);

		assert!((v2.x - -1.0).abs() < 0.001);
		assert!((v2.y -  0.0).abs() < 0.001);
		assert!((v2.y -  0.0).abs() < 0.001);

		let i1 = Mat4x4::ident();
		let t1 = Mat4x4::translation(0.1, 0.2, 0.3);
		let r = i1 * t1;

		assert!((r.m[0] - 1.0).abs() < 0.001);
		assert!((r.m[1] - 0.0).abs() < 0.001);
		assert!((r.m[2] - 0.0).abs() < 0.001);

		assert!((r.m[3]  - 0.1).abs() < 0.001);
		assert!((r.m[7]  - 0.2).abs() < 0.001);
		assert!((r.m[11] - 0.3).abs() < 0.001);

	}
}