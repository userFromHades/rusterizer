
extern crate sdl2;
extern crate rand;

#[macro_use]
extern crate bitflags;

mod canvas;
mod wavefront_obj;
mod truevision_tga;
mod vec;
mod mesh;
mod texture;

fn main() {

	let head_t = truevision_tga::load_from_file("african_head_diffuse.tga");
	if head_t.is_err(){
		println!("Can't load tga texture");
		return;
	}
	let head_t = head_t.unwrap();

	let teaport = wavefront_obj::load_from_file("./teapot.obj");
	let head = wavefront_obj::load_from_file("./african_head.obj");

	println!("Hello, world!");

	let mut c = canvas::MyCanvas::new (800, 600);

	c.clear();
	c.point(128, 128, 0xff0000);

	c.line(  10,  10, 200,  20, 0xff0000);
	c.line( 10,  10,  20, 200, 0x00ff00);

	c.line( 210,  210,  20, 200, 0xffff00);
	c.line( 210,  210,  200, 20, 0xff0000);

	//c.line( 10,  10,  210, 210, 0xff0000);
	c.line( 210, 210, 10,  10,  0xff0000);
	//c.line( 200, 20, 20,  200,  0xff00ff);
	c.line( 20,  200, 200, 20,   0xff00ff);

	c.set_texture(head_t);
	/*
	c.draw_textured_triangle(
	    mesh::Vertex{ x:  0.6, y:  0.6, z: 0.0, tx: 1.0, ty: 1.0, nx: 0.0, ny: 0.0, nz: 0.0},
	    mesh::Vertex{ x: -0.4, y:  0.5, z: 7.5, tx: 0.0, ty: 1.0, nx: 0.0, ny: 0.0, nz: 0.0},
	    mesh::Vertex{ x:  0.5, y: -0.4, z: 0.0, tx: 1.0, ty: 0.0, nx: 0.0, ny: 0.0, nz: 0.0});
	c.draw_textured_triangle(
	    mesh::Vertex{ x: -0.5, y: -0.5, z: 0.0, tx: 0.0, ty: 0.0, nx: 0.0, ny: 0.0, nz: 0.0},
	    mesh::Vertex{ x: -0.4, y:  0.5, z: 7.5, tx: 0.0, ty: 1.0, nx: 0.0, ny: 0.0, nz: 0.0},
	    mesh::Vertex{ x:  0.5, y: -0.4, z: 0.0, tx: 1.0, ty: 0.0, nx: 0.0, ny: 0.0, nz: 0.0});
	*/

	let m1 = vec::Mat4x4::scale(0.5, 0.5, 0.5) * vec::Mat4x4::translation(0.0, -0.5, 5.0) * vec::Mat4x4::retro_proj(0.45);
	let m2 = vec::Mat4x4::scale(0.5, 0.5, 0.5) * vec::Mat4x4::translation(0.3, 0.2, 8.5) * vec::Mat4x4::retro_proj(0.45);
	teaport.draw(&mut c, &m1);
	teaport.draw(&mut c, &m2);

	//c.set_texture(head_t);
	let m1 = vec::Mat4x4::y_rotation(3.141592) * vec::Mat4x4::translation(-0.3,- 0.2, 1.5) * vec::Mat4x4::retro_proj(0.45);
	head.draw   (&mut c, &m1);

	c.wait_end();

}
