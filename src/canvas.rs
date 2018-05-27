use std::thread;
use std::time;
use std::mem;

extern crate rand;
extern crate sdl2;

use sdl2::pixels::PixelFormatEnum;
use sdl2::keyboard::Keycode;
use sdl2::video::Window;

use vec;
use texture;
use mesh;

fn sort_vertexes ( mut p0 : vec::Vec3,
                   mut p1 : vec::Vec3,
                   mut p2 : vec::Vec3) ->
                (vec::Vec3, vec::Vec3, vec::Vec3)
{
	if p0.x > p1.x {
		mem::swap(&mut p0, &mut p1);
	}
	if p0.x > p2.x {
		mem::swap(&mut p0, &mut p2);
	}
	if p1.x > p2.x {
		mem::swap(&mut p1, &mut p2);
	}

	(p0, p1, p2)
}

fn sort_vertexes_ (
    mut p0 : mesh::Vertex,  mut p1 : mesh::Vertex,  mut p2 : mesh::Vertex) ->
    (mesh::Vertex, mesh::Vertex, mesh::Vertex)
{
	if p0.x > p1.x {
		mem::swap(&mut p0, &mut p1);
	}
	if p0.x > p2.x {
		mem::swap(&mut p0, &mut p2);
	}
	if p1.x > p2.x {
		mem::swap(&mut p1, &mut p2);
	}

	(p0, p1, p2)
}

pub struct MyCanvas {
	sdl_context : sdl2::Sdl,
	sdl_canvas: sdl2::render::Canvas<Window>,

	width: u32,
	height: u32,

	rgb_buffer: Vec<u8>,
	z_buffer: Vec<f32>,

	pub texture : Option<texture::Texture>,

}

impl MyCanvas {

	pub fn new(width: u32, height: u32) -> MyCanvas {
		let sdl_context = sdl2::init().unwrap();
		let sdl_video = sdl_context.video().unwrap();

		let sdl_window = sdl_video.window("rust-sdl2 demo: Video", width, height)
		    .position_centered()
		    .opengl()
		    .build()
		    .unwrap();

		let sdl_canvas = sdl_window.into_canvas().build().unwrap();

		MyCanvas {
			sdl_context: sdl_context,
			sdl_canvas: sdl_canvas,
			width: width,
			height: height,
			rgb_buffer: vec![0; (width * height * 3) as usize],
			z_buffer: vec![100000.0; (width * height) as usize],
			texture : None,
		}
	}

	pub fn set_texture(&mut self, t: texture::Texture){
		self.texture = Some(t);
	}

	pub fn get_tex_colour (&mut self, x : f32, y : f32) -> u32 {
		return if self.texture.is_some(){
			let t = self.texture.as_ref().unwrap();
			t.get(x, y)
		} else { 0x777777 };
	}

	pub fn clear(&mut self) {
		self.sdl_canvas.clear();
	}

	pub fn set_depth (&mut self, x: i32, y: i32, d : f32){
		if x < 0 ||
		   y < 0 ||
		   x >= (self.width as i32) ||
		   y >= (self.height as i32)
		{
			return;
		}

		let i = (x as u32 + y as u32 * self.width) as usize;
		self.z_buffer[i] = d
	}

	pub fn depth (&mut self, x: i32, y: i32) -> f32{
		if x < 0 ||
		   y < 0 ||
		   x >= (self.width as i32) ||
		   y >= (self.height as i32)
		{
			return -100000.0;
		}

		let i = (x as u32 + y as u32 * self.width) as usize;
		self.z_buffer[i]
	}

	pub fn point (&mut self, x: i32, y: i32, color: u32){
		if x < 0 ||
		   y < 0 ||
		   x >= (self.width as i32) ||
		   y >= (self.height as i32)
		{
			return;
		}

		let i = 3 * (x as u32 + y as u32 * self.width) as usize;
		self.rgb_buffer[i + 0] = (color >> (8*2)) as u8;
		self.rgb_buffer[i + 1] = (color >> (8*1)) as u8;
		self.rgb_buffer[i + 2] = (color >> (8*0)) as u8;
	}

	pub fn line (&mut self,
	              mut x0 : i32, mut y0 : i32,
	              mut x1 : i32, mut y1 : i32,
	              color: u32)
	{
		let steep = (x1 - x0).abs() < (y1 - y0).abs();
		if steep {
			mem::swap(&mut y0, &mut x0);
			mem::swap(&mut y1, &mut x1);
		}

		if x0 > x1{
			mem::swap(&mut y0, &mut y1);
			mem::swap(&mut x0, &mut x1);
		}

		let dx = x1 - x0;
		let k  = 2 * (y1 - y0).abs();
		let inv = y0 > y1;

		let mut err  = 0;
		let mut y = y0;

		for x in x0 .. x1 + 1 {
			err += k;
			let d =  if err > dx { err -= 2*dx; 1 } else { 0 };
			y += if !inv{ d } else { -d };

			if !steep {
				self.point(x, y, color);
			}
			else{
				self.point(y, x, color);
			}
		}
	}

	pub fn to_pix_coord (&self, x : f32, y : f32) ->(i32,  i32){
		(
			((x + 1.0) * (self.width  as f32)/ 2.0) as i32,
			((1.0 - y) * (self.height as f32)/ 2.0) as i32
		)
	}

	#[allow(dead_code)]
	pub fn daraw_triangle_list (&mut self, vertex : Vec<f32>, index:Vec<u32>){
		for i in 0..index.len()/3 {
			let idx0 = index[i * 3 + 0] - 1;
			let idx1 = index[i * 3 + 1] - 1;
			let idx2 = index[i * 3 + 2] - 1;

			let x0 = vertex[(idx0 * 3 + 0) as usize];
			let y0 = vertex[(idx0 * 3 + 1) as usize];

			let x1 = vertex[(idx1 * 3 + 0) as usize];
			let y1 = vertex[(idx1 * 3 + 1) as usize];

			let x2 = vertex[(idx2 * 3 + 0) as usize];
			let y2 = vertex[(idx2 * 3 + 1) as usize];

			let (x0, y0) = self.to_pix_coord(x0, y0);
			let (x1, y1) = self.to_pix_coord(x1, y1);
			let (x2, y2) = self.to_pix_coord(x2, y2);

			self.line(x0, y0, x1, y1, 0xffffff);
			self.line(x1, y1, x2, y2, 0xffffff);
			self.line(x2, y2, x0, y0, 0xffffff);

		}
	}

	pub fn draw_solid_triangle (&mut self, p0 : vec::Vec3,
	                                       p1 : vec::Vec3,
	                                       p2 : vec::Vec3,
	                                       color : u32 )
	{
		let (p0, p1, p2) = sort_vertexes(p0, p1, p2);

		let (x0, y0) = self.to_pix_coord(p0.x, p0.y);
		let (x1, y1) = self.to_pix_coord(p1.x, p1.y);
		let (x2, y2) = self.to_pix_coord(p2.x, p2.y);

		let ky10 = (y1 - y0) as f32 / (x1 - x0) as f32;
		let ky21 = (y2 - y1) as f32 / (x2 - x1) as f32;
		let ky20 = (y2 - y0) as f32 / (x2 - x0) as f32;

		let z0 = p0.z;
		let z1 = p1.z;
		let z2 = p2.z;

		let kz10 = (z1 - z0) as f32 / (x1 - x0) as f32;
		let kz21 = (z2 - z1) as f32 / (x2 - x1) as f32;
		let kz20 = (z2 - z0) as f32 / (x2 - x0) as f32;

		for x in x0..x1{
			let mut _y0 = y0 + (ky20 * (x - x0) as f32) as i32;
			let mut _y1 = y0 + (ky10 * (x - x0) as f32) as i32;

			let mut _z0 = z0 + (kz20 * (x - x0) as f32) ;
			let mut _z1 = z0 + (kz10 * (x - x0) as f32) ;

			if _y0 > _y1{
				mem::swap(&mut _y0, &mut _y1);
				mem::swap(&mut _z0, &mut _z1);
			}

			let kz = (_z1 - _z0) / (_y1 - _y0) as f32;
			for y in _y0.._y1{
				let _z = _z0 + (y - _y0) as f32 * kz;
				if _z <  self.depth(x, y){
					self.set_depth(x, y, _z);
					self.point(x, y, color);
				}
			}
		}

		for x in x1..(x2 ){
			let mut _y0 = y0 + (ky20 * (x - x0) as f32) as i32;
			let mut _y1 = y1 + (ky21 * (x - x1) as f32) as i32;

			let mut _z0 = z0 + (kz20 * (x - x0) as f32) ;
			let mut _z1 = z1 + (kz21 * (x - x1) as f32) ;

			if _y0 > _y1{
				mem::swap(&mut _y0, &mut _y1);
				mem::swap(&mut _z0, &mut _z1);
			}

			let kz = (_z1 - _z0) / (_y1 - _y0) as f32;
			for y in _y0.._y1{
				let _z = _z0 + (y - _y0) as f32 * kz;
				if _z <  self.depth(x, y){
					self.set_depth(x, y, _z);
					self.point(x, y, color);
				}
			}
		}
	}

	fn draw_vline (&mut self, x : i32, _p0 : &mesh::Vertex, _p1 : &mesh::Vertex){

		let y0 = ((1.0 - _p0.y) * (self.height as f32)/ 2.0) as i32;
		let y1 = ((1.0 - _p1.y) * (self.height as f32)/ 2.0) as i32;

		for y in y0..y1{
			let dy = (y - y0) as f32;
			let k = -(dy as f32) * 2.0 / (self.height as f32) / (_p1.y - _p0.y);

			let p = mesh::Vertex::interpolate(&_p0, &_p1, k);

			if p.z <  self.depth(x, y){
				self.set_depth(x, y, p.z);
				let color = self.get_tex_colour(p.tx, p.ty);
				self.point(x, y, color);
			}
		}
	}

	pub fn draw_textured_triangle (&mut self,
	    p0 : mesh::Vertex,
	    p1 : mesh::Vertex,
	    p2 : mesh::Vertex
	    )
	{
		let (p0, p1, p2) = sort_vertexes_(p0, p1, p2);

		let (x0, _) = self.to_pix_coord(p0.x, p0.y);
		let (x1, _) = self.to_pix_coord(p1.x, p1.y);
		let (x2, _) = self.to_pix_coord(p2.x, p2.y);

		let kx10 = 2.0 / (self.width as f32) / (p1.x - p0.x);
		let kx20 = 2.0 / (self.width as f32) / (p2.x - p0.x);
		let kx21 = 2.0 / (self.width as f32) / (p2.x - p1.x);

		for x in x0..x1{
			let dx = (x - x0) as f32;

			let k = (dx as f32) * kx10;
			let mut _p0 = mesh::Vertex::interpolate(&p0, &p1, k);

			let k = (dx as f32) * kx20;
			let mut _p1 = mesh::Vertex::interpolate(&p0, &p2, k);

			if _p0.y < _p1.y {
				mem::swap(&mut _p0,   &mut _p1);
			}

			self.draw_vline(x, &_p0, &_p1);
		}

		for x in x1..x2{

			let k = (x - x1) as f32 * kx21;
			let mut _p0 = mesh::Vertex::interpolate(&p1, &p2, k);

			let k = (x - x0) as f32 * kx20;
			let mut _p1 = mesh::Vertex::interpolate(&p0, &p2, k);

			if _p0.y < _p1.y {
				mem::swap(&mut _p0,   &mut _p1);
			}

			self.draw_vline(x, &_p0, &_p1);
		}
	}

	pub fn wait_end (&mut self) {

		let texture_creator = self.sdl_canvas.texture_creator();

		let mut texture = texture_creator.create_texture_streaming(
		PixelFormatEnum::RGB24, self.width, self.height).unwrap();

		texture.with_lock(None, |buffer: &mut [u8], _: usize| {
			let n = (self.width * self.height * 3) as usize;
			for i in 0..n {
				buffer[i] = self.rgb_buffer[i];
			}
		}).unwrap();

		self.sdl_canvas.copy(&texture, None, None).unwrap();

		self.sdl_canvas.present();
		let mut running = true;
		let mut event_pump = self.sdl_context.event_pump().unwrap();
		while running {
			for event in event_pump.poll_iter() {
				use sdl2::event::Event;

				match event {
					Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
						running = false
					},
					_ => {}
				}
			}
			thread::sleep(time::Duration::from_millis(1))
		}
	}
}
