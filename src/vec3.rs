
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
}
#[allow(dead_code)]
fn dot_product(v1 : Vec3, v2 : Vec3) -> f32{
    v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
}

#[allow(dead_code)]
fn cross_product(v1 : Vec3, v2 : Vec3) -> (Vec3){
    Vec3 {x: v1.y * v2.z - v1.z * v2.y,
          y: v1.z * v2.x - v1.x * v2.z,
          z: v1.x * v2.y - v1.y * v2.x}
}
