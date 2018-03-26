extern crate sdl2;

mod canvas;
//use canvas;

fn main() {

    println!("Hello, world!");
    //canvas::test();
    let mut c = canvas::Canvas::new (800, 600);
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


    c.wait_end();
}
