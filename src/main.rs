
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
	    vec::Vec3::new(0.6, 0.6, 0.0), vec::Vec2{x: 1.0, y : 1.0},
	    vec::Vec3::new(0.0, 0.5, 0.0), vec::Vec2{x: 0.0, y : 1.0},
	    vec::Vec3::new(0.5, 0.0, 0.0), vec::Vec2{x: 1.0, y : 0.0});
	c.draw_textured_triangle(
	    vec::Vec3::new(0.1, 0.1, 0.0), vec::Vec2{x: 0.0, y : 0.0},
	    vec::Vec3::new(0.0, 0.5, 0.0), vec::Vec2{x: 0.0, y : 1.0},
	    vec::Vec3::new(0.5, 0.0, 0.0), vec::Vec2{x: 1.0, y : 0.0});
	*/

	teaport.draw(&mut c, 0.2, vec::Vec3::new(0.0, -0.5, 1.0));
	teaport.draw(&mut c, 0.2, vec::Vec3::new(0.3, -0.2, 1.5));
	head.draw   (&mut c, 0.5, vec::Vec3::new(-0.5, 0.0, 0.5));

	c.wait_end();

}
