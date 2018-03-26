use std::thread;
use std::time;
use std::mem;
use std::num;

extern crate sdl2;

use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;

pub struct Canvas {
    sdl_context : sdl2::Sdl,
    renderer: sdl2::render::Renderer<'static>
}

impl Canvas {

    pub fn new(width: u32, height: u32) -> Canvas {
        let sdl_context = sdl2::init().video().unwrap();

        let window = sdl_context.window("rust-sdl2 demo: Video", width, height)
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let renderer = window.renderer().build().unwrap();

        Canvas { sdl_context: sdl_context, renderer: renderer}
    }

    pub fn clear(&mut self) {
         self.renderer.clear();
    }

    pub fn point (&mut self, x: i32, y: i32, color: u32){
        self.renderer.set_draw_color(Color::RGB((color >> (8*2)) as u8, (color >> (8*1)) as u8, color as u8));
        self.renderer.draw_point(Point::new(x, y));
        self.renderer.present();
    }

    pub fn line (&mut self, x0 : i32, y0 : i32, x1 : i32, y1 : i32, color: u32  ){

        if (x1 - x0).abs() > (y1 - y0).abs() {
            if  x1 > x0 {
                for x in x0 .. x1 {
                    self.point(x, y0 + (x - x0) * (y1 - y0)/(x1 - x0), color);
                }
                println!("1") 
            }
            else {
                for x in x1 .. x0 {
                    self.point(x, y1 + (x - x1) * (y0 - y1)/(x0 - x1), color);
                }
                println!("2")
            }
        }
        else{
            if y1 > y0{
                for y in y0 .. y1 {
                    self.point(x0 + (y - y0) * (x1 - x0) / (y1 - y0), y, color);
                }
                println!("3")
            }
            else
            {
                for y in y1 .. y0 {
                    self.point(x1 + (y - y1) * (x0 - x1) / (y0 - y1), y, color);
                }
                println!("4")
            }
        }


    }

    pub fn test (&mut self) {
        // FIXME: rework it
        let mut texture = self.renderer.create_texture_streaming(PixelFormatEnum::RGB24, (256, 256)).unwrap();
        // Create a red-green gradient
        texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
            for y in 0..256 {
                for x in 0..256 {
                    let offset = y*pitch + x*3;
                    buffer[offset + 0] = x as u8;
                    buffer[offset + 1] = y as u8;
                    buffer[offset + 2] = 0 as u8;
                }
            }
        }).unwrap();

        self.renderer.clear();
        self.renderer.copy(&texture, None, Some(Rect::new_unwrap(100, 100, 256, 256)));
        self.renderer.copy_ex(&texture, None, Some(Rect::new_unwrap(450, 100, 256, 256)), 30.0, None, (false, false));
        self.renderer.present();
    }

    pub fn wait_end (&mut self) {

        let mut running = true;

        while running {
            for event in self.sdl_context.event_pump().poll_iter() {
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
