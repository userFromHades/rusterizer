
extern crate sdl2;
extern crate rand;

mod canvas;
mod wavefront_obj;
mod vec3;


fn main() {

    let (vertex, index) = wavefront_obj::load_from_file("./teapot.obj");

    println!("Hello, world!");
    //canvas::test();
    let mut c = canvas::MyCanvas::new (800, 600);
    //c.test();

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

    //c.daraw_triangle_list(vertex, index);

    c.draw_solid_triangle(
        0.5, 0.5, 0.0,
        0.0, 0.5, 0.0,
        0.5, 0.0, 5.5, 0xffffff);

    c.draw_solid_triangle_list(&vertex, &index, 0.2, vec3::vec3::new(0.0, -0.5, 1.0));
    c.draw_solid_triangle_list(&vertex, &index, 0.2, vec3::vec3::new(0.3, -0.2, 1.5));

    c.wait_end();

}
