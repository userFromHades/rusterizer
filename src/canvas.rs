use std::thread;
use std::time;
use std::mem;

extern crate sdl2;

use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;

pub struct Canvas {
    sdl_context : sdl2::Sdl,
    renderer: sdl2::render::Renderer<'static>,

    width: u32,
    height: u32,
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

        Canvas { sdl_context: sdl_context, renderer: renderer, width: width, height: height}
    }

    pub fn clear(&mut self) {
         self.renderer.clear();
    }

    pub fn point (&mut self, x: i32, y: i32, color: u32){
        let color =  Color::RGB((color >> (8*2)) as u8,
                                (color >> (8*1)) as u8,
                                 color           as u8);
        self.renderer.set_draw_color(color);
        self.renderer.draw_point(Point::new(x, y));
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
            ((x + 1.0) * (self.width  as f32)/ 2.0 )as i32,
            ((y + 1.0) * (self.height as f32)/ 2.0) as i32
        )
    }

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

        self.renderer.present();

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
