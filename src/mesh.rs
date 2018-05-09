use canvas;
use vec3;
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

pub struct Mesh {
    vertex  : Vec<f32>,
    index   : Vec<u32>,
    
    vertex_type : VertexType
}

impl Mesh {

	pub fn new (vertex : Vec<f32>, index : Vec<u32>, vertex_type : VertexType) -> Mesh {
		Mesh { vertex : vertex, index : index, vertex_type : vertex_type }
	}

    pub fn draw (& self,
                 c : &mut canvas::MyCanvas, 
                 scale : f32,
                 offset : vec3::Vec3) 
    {
        let index  = & self.index;
        let vertex = & self.vertex;

        let count = self.vertex_type.count();

        for i in 0..index.len()/3 {

            let idx0 = (index[i * 3 + 0]) as usize;
            let idx1 = (index[i * 3 + 1]) as usize;
            let idx2 = (index[i * 3 + 2]) as usize;

            let x0 = scale * vertex[(idx0 * count + 0) as usize] + offset.x;
            let y0 = scale * vertex[(idx0 * count + 1) as usize] + offset.y;
            let z0 = scale * vertex[(idx0 * count + 2) as usize] + offset.z;

            let x1 = scale * vertex[(idx1 * count + 0) as usize] + offset.x;
            let y1 = scale * vertex[(idx1 * count + 1) as usize] + offset.y;
            let z1 = scale * vertex[(idx1 * count + 2) as usize] + offset.z;

            let x2 = scale * vertex[(idx2 * count + 0) as usize] + offset.x;
            let y2 = scale * vertex[(idx2 * count + 1) as usize] + offset.y;
            let z2 = scale * vertex[(idx2 * count + 2) as usize] + offset.z;

            let (vx1, vy1, vz1 ) = (x1 - x0, y1 - y0, z1 - z0);
            let (vx2, vy2, vz2 ) = (x2 - x0, y2 - y0, z2 - z0);

            let n = vec3::cross_product (vec3::Vec3::new(vx1, vy1, vz1), 
                                         vec3::Vec3::new(vx2, vy2, vz2) ).normalized();

            if n.z >= 0.0 {
                continue;
            }

            let l = vec3::Vec3::new(1.0, 0.0, 0.0).normalized();

            let f = clump (0.5 + 0.5 * vec3::dot_product(n, l), 0.0, 1.0);

            let cl = (f * 255.0) as u32;
            let color : u32 = (cl << 16) | (cl << 8) | cl;

            c.draw_solid_triangle (x0, y0, z0, x1, y1, z1, x2, y2, z2, color);
        }
    }

}
