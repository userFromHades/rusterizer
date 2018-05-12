
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

use std::ops::{Add, Sub};

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
