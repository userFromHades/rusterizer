
pub struct vec3{
	pub x : f32,
	pub y : f32,
	pub z : f32
}

impl vec3 {

	pub fn new (x: f32, y: f32, z: f32) -> vec3 {
		vec3 {x: x, y: y, z: z}
	}

	pub fn length(&self) -> f32 {
		(self.x*self.x + self.y*self.y + self.z*self.z).sqrt()
	}

	pub fn normalize(&mut self){
		let l = self.length();
		self.x /= l;
		self.y /= l; 
		self.z /= l;
	}

	pub fn normalized(&self) -> vec3{
		let l = self.length();
		vec3 { x: self.x / l, y: self.y / l, z: self.z / l}
	}
}

fn dot_product(v1 : vec3, v2 : vec3) -> f32{
    v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
}

fn cross_product(v1 : vec3, v2 : vec3) -> (vec3){
    vec3 {x: v1.y * v2.z - v1.z * v2.y,
          y: v1.z * v2.x - v1.x * v2.z,
          z: v1.x * v2.y - v1.y * v2.x}
}
