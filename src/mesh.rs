use canvas;
use vec;
//#[macro_use]
//extern crate bitflags;

#[allow(dead_code)]
fn clump (x : f32, a :f32, b: f32) -> f32 {
	if x < a{
		a
	}else if x > b {
		b
	}
	else {x}
}

bitflags! {
	pub struct VertexType: u32 {
		const POSITION = 0b00000001;
		const TEXTURE  = 0b00000010;
		const NORMALE  = 0b00000100;
	}
}

impl VertexType {

	#[allow(dead_code)]
	pub fn size (self) -> usize {
		let mut size : usize = 0;
		if ! (self &  VertexType::POSITION).is_empty() {
			size += 3 * 4;
		}
		if ! (self & VertexType::TEXTURE).is_empty() {
			size += 2 * 4;
		}
		if ! (self & VertexType::NORMALE).is_empty() {
			size += 3 * 4;
		}
		size
	}

	#[warn(dead_code)]
	pub fn count (self) -> usize {
		let mut size : usize = 0;
		if ! (self &  VertexType::POSITION).is_empty() {
			size += 3;
		}
		if ! (self & VertexType::TEXTURE).is_empty() {
			size += 2;
		}
		if ! (self & VertexType::NORMALE).is_empty() {
			size += 3;
		}
		size
	}
}

pub struct Vertex {
	pub x  : f32,
	pub y  : f32,
	pub z  : f32,

	pub tx : f32,
	pub ty : f32,

	pub nx : f32,
	pub ny : f32,
	pub nz : f32,
}

impl Vertex {
	pub fn get_position(&self) -> vec::Vec3 {
		vec::Vec3::new(self.x, self.y, self.z)
	}

	pub fn set_position(&mut self, p : & vec::Vec3){
		self.x = p.x;
		self.y = p.y;
		self.z = p.z;
	}

	pub fn interpolate (p0 : & Vertex, p1 : & Vertex, k : f32) ->Vertex{
		Vertex {
			x :  (1.0 - k) * p0.x   +  k * p1.x,
			y :  (1.0 - k) * p0.y   +  k * p1.y,
			z :  (1.0 - k) * p0.z   +  k * p1.z,

			tx : (1.0 - k) * p0.tx  +  k * p1.tx,
			ty : (1.0 - k) * p0.ty  +  k * p1.ty,

			nx : (1.0 - k) * p0.nx  +  k * p1.nx,
			ny : (1.0 - k) * p0.ny  +  k * p1.ny,
			nz : (1.0 - k) * p0.nz  +  k * p1.nz,
		}
	}
}



pub struct Mesh {
	vertex  : Vec<f32>,
	index   : Vec<u32>,
	
	vertex_type : VertexType
}

impl Mesh {

	pub fn new (vertex : Vec<f32>, index : Vec<u32>, vertex_type : VertexType) -> Mesh {
		Mesh { vertex : vertex, index : index, vertex_type : vertex_type }
	}

	fn get_position (&self, index : usize) -> vec::Vec3 {
		let count = self.vertex_type.count();
		vec::Vec3::new(  self.vertex[(index * count + 0) as usize],
		                 self.vertex[(index * count + 1) as usize],
		                 self.vertex[(index * count + 2) as usize])
	}

	fn get_tex_coord (&self, index : usize) -> vec::Vec2 {
		let count = self.vertex_type.count();
		vec::Vec2::new(  self.vertex[(index * count + 3) as usize],
		                 self.vertex[(index * count + 4) as usize])
	}

	pub fn get_vertex (&self, index : usize) -> Vertex {
		let count = self.vertex_type.count();

		let x = self.vertex[(index * count + 0) as usize];
		let y = self.vertex[(index * count + 1) as usize];
		let z = self.vertex[(index * count + 2) as usize];

		let (tx, ty) = if ! (self.vertex_type & VertexType::TEXTURE).is_empty() {(
			self.vertex[(index * count + 3) as usize],
			self.vertex[(index * count + 4) as usize]
		)} else {(0.0, 0.0)};

		let (nx, ny, nz) = if ! (self.vertex_type & VertexType::NORMALE).is_empty() {(
			self.vertex[(index * count + 5) as usize],
			self.vertex[(index * count + 6) as usize],
			self.vertex[(index * count + 7) as usize]
		)} else {(0.0, 0.0, 0.0)};

		Vertex { x : x , y : y, z : z, nx : nx, ny : ny, nz : nz, tx: tx, ty : ty}
	}

	pub fn draw (& self,
	             c : &mut canvas::MyCanvas,
	             transform : &vec::Mat4x4)
	{
		let index  = & self.index;

		for i in 0..index.len()/3 {

			let idx0 = (index[i * 3 + 0]) as usize;
			let idx1 = (index[i * 3 + 1]) as usize;
			let idx2 = (index[i * 3 + 2]) as usize;

			let mut v0 = self.get_vertex(idx0);
			let mut v1 = self.get_vertex(idx1);
			let mut v2 = self.get_vertex(idx2);

			let mut p0 = transform.apply(&self.get_position(idx0));
			let mut p1 = transform.apply(&self.get_position(idx1));
			let mut p2 = transform.apply(&self.get_position(idx2));

			v0.set_position(&p0);
			v1.set_position(&p1);
			v2.set_position(&p2);

			//let v1 = &p1 - &p0;
			//let v2 = &p2 - &p0;
			let n = vec::cross_product (&p1 - &p0, &p2 - &p0).normalized();

			if n.z >= 0.0 {
				continue;
			}

			let l = vec::Vec3::new(1.0, 0.0, 0.0).normalized();

			let f = clump (0.5 + 0.5 * vec::dot_product(n, l), 0.0, 1.0);

			let cl = (f * 255.0) as u32;
			let color : u32 = (cl << 16) | (cl << 8) | cl;

			if (self.vertex_type & VertexType::TEXTURE).is_empty() {
				c.draw_solid_triangle (p0, p1, p2, color);
			}
			else{
				c.draw_textured_triangle(v0, v1, v2);
			}
		}
	}

}



#[cfg(test)]
mod tests {
	use super::*;
	use std::f32::consts::PI;

	#[test]
	fn test_vertex_interpolate() {

		let v0 = Vertex{ x : 0.0, y : 1.0, z : 2.0, tx: 3.0, ty : 4.0, nx : 5.0,  ny : 6.0, nz: 7.0 };
		let v1 = Vertex{ x : 1.0, y : 2.0, z : 3.0, tx: 4.0, ty : 5.0, nx : 6.0,  ny : 7.0, nz: 8.0 };

		let v = Vertex::interpolate(&v0, &v1, 0.5);

		assert_eq!(v.x, 0.5);
		assert_eq!(v.y, 1.5);
		assert_eq!(v.z, 2.5);
		assert_eq!(v.tx, 3.5);
		assert_eq!(v.ty, 4.5);
		


	}
}