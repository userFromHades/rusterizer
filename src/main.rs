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
   // c.line( 10,  10,  20, 200, 0x00ff00);
   /* 
    c.line( 200, 200, 0,  0, 0xffffff);
    c.line( 0, 200, 200,  0, 0x0000ff);
    c.line(20, 200, 200,  0, 0x00ffff);
    c.line( 200, 200, 20, 20,  0xff00ff);

    c.line( 200,  20, 220, 220, 0x0000ff);
    c.line( 220, 220, 20, 200, 0x0000ff);
*/
    c.wait_end();
}
