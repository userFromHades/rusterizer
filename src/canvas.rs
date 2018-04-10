use std::thread;
use std::time;
use std::mem;

extern crate rand;

extern crate sdl2;
use sdl2::pixels::PixelFormatEnum;

use sdl2::keyboard::Keycode;
use sdl2::video::Window;

fn normalise( x : f32, y : f32, z : f32) ->(f32, f32, f32) {
    let l = (x * x + y * y + z * z ).sqrt();
    (x / l, y / l, z / l)
}

fn dot_product(x1 : f32, y1 : f32, z1 : f32,
               x2 : f32, y2 : f32, z2 : f32) -> f32{
    x1 * x2 + y1 * y2 + z1 * z2
}

fn cross_product(x1 : f32, y1 : f32, z1 : f32,
                 x2 : f32, y2 : f32, z2 : f32) -> (f32, f32, f32){
    (y1 * z2 - z1 * y2,
     z1 * x2 - x1 * z2,
     x1 * y2 - y1 * x2)
}

fn clump (x : f32, a :f32, b: f32) -> f32 {
    if x < a{
        a
    }else if x > b {
        b
    }
    else {x}
}

fn sort_vertexes (mut x0 : f32, mut y0 : f32, mut z0 : f32,
                    mut x1 : f32, mut y1 : f32, mut z1 : f32,
                    mut x2 : f32, mut y2 : f32, mut z2 : f32) ->
                    (f32, f32, f32,
                    f32, f32, f32,
                    f32, f32, f32)
{
    if x0 > x1 {
        mem::swap(&mut x0, &mut x1);
        mem::swap(&mut y0, &mut y1);
        mem::swap(&mut z0, &mut z1);
    }
    if x0 > x2 {
        mem::swap(&mut x0, &mut x2);
        mem::swap(&mut y0, &mut y2);
        mem::swap(&mut z0, &mut z2);
    }
    if x1 > x2 {
        mem::swap(&mut x1, &mut x2);
        mem::swap(&mut y1, &mut y2);
        mem::swap(&mut z1, &mut z2);
    }
    (x0,y0,z0, x1,y1,z1, x2,y2,z2)
}


pub struct MyCanvas {
    sdl_context : sdl2::Sdl,
    sdl_canvas: sdl2::render::Canvas<Window>,

    width: u32,
    height: u32,

    rgb_buffer: Vec<u8>,
    z_buffer: Vec<f32>,

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
            z_buffer: vec![0.0; (width * height) as usize]
        }
    }

    pub fn clear(&mut self) {
         self.sdl_canvas.clear();
    }

    pub fn setDepth (&mut self, x: i32, y: i32, d : f32){
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
            ((x + 1.0) * (self.width  as f32)/ 2.0 )as i32,
            ((1.0 - y) * (self.height as f32)/ 2.0) as i32
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


    pub fn draw_solid_triangle (&mut self, mut x0 : f32, mut y0 : f32, mut z0 : f32,
                                           mut x1 : f32, mut y1 : f32, mut z1 : f32,
                                           mut x2 : f32, mut y2 : f32, mut z2 : f32,
                                           color : u32 )
    {
        let (x0,y0,z0, x1,y1,z1, x2,y2,z2) = sort_vertexes(x0,y0,z0, x1,y1,z1, x2,y2,z2);

        let (x0, y0) = self.to_pix_coord(x0, y0);
        let (x1, y1) = self.to_pix_coord(x1, y1);
        let (x2, y2) = self.to_pix_coord(x2, y2);


        let ky10 = (y1 - y0) as f32 / (x1 - x0) as f32;
        let ky21 = (y2 - y1) as f32 / (x2 - x1) as f32;
        let ky20 = (y2 - y0) as f32 / (x2 - x0) as f32;

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

            let kz = _z1 - _z0 / (_y1 - _y0) as f32;
            for y in _y0.._y1{
                let _z = _z0 + (y - _y0) as f32 * kz;
                if _z <  self.depth(x, y){
                    self.setDepth(x, y, _z);
                    self.point(x, y, color);
                }
            }
        }

        for x in x1..x2{
            let mut _y0 = y0 + (ky20 * (x - x0) as f32) as i32;
            let mut _y1 = y1 + (ky21 * (x - x1) as f32) as i32;

            let mut _z0 = z0 + (kz20 * (x - x0) as f32) ;
            let mut _z1 = z1 + (kz21 * (x - x0) as f32) ;

            if _y0 > _y1{
                mem::swap(&mut _y0, &mut _y1);
                mem::swap(&mut _z0, &mut _z1);
            }

            let kz = _z1 - _z0 / (_y1 - _y0) as f32;
            for y in _y0.._y1{
                let _z = _z0 + (y - _y0) as f32 * kz;
                if _z <  self.depth(x, y){
                    self.setDepth(x, y, _z);
                    self.point(x, y, color);
                }
            }
        }
    }

    pub fn draw_solid_triangle_list (&mut self,
        vertex : Vec<f32>,
        index  : Vec<u32>)
    {

        let mut rng = rand::thread_rng();
        for i in 0..index.len()/3 {

            let idx0 = index[i * 3 + 0] - 1;
            let idx1 = index[i * 3 + 1] - 1;
            let idx2 = index[i * 3 + 2] - 1;

            let x0 = 0.2 * vertex[(idx0 * 3 + 0) as usize];
            let y0 = 0.2 * vertex[(idx0 * 3 + 1) as usize];
            let z0 = 0.2 * vertex[(idx0 * 3 + 2) as usize];

            let x1 = 0.2 * vertex[(idx1 * 3 + 0) as usize];
            let y1 = 0.2 * vertex[(idx1 * 3 + 1) as usize];
            let z1 = 0.2 * vertex[(idx1 * 3 + 2) as usize];

            let x2 = 0.2 * vertex[(idx2 * 3 + 0) as usize];
            let y2 = 0.2 * vertex[(idx2 * 3 + 1) as usize];
            let z2 = 0.2 * vertex[(idx2 * 3 + 2) as usize];

            let (vx1, vy1, vz1 ) = (x1 - x0, y1 - y0, z1 - z0);
            let (vx2, vy2, vz2 ) = (x2 - x0, y2 - y0, z2 - z0);

            let (nx, ny, nz) = cross_product(vx1, vy1, vz1, vx2, vy2, vz2);

            let (nx, ny, nz) = normalise (nx, ny, nz);

           /* if nz >= 0.0 {
                continue;
            }*/

            let (lx, ly, lz) = normalise(1.0, 0.0, 0.0);

            let f = clump (0.5 + 0.5 * dot_product(nx, ny, nz, lx, ly, lz), 0.0, 1.0);


            let cl = (f * 255.0) as u32;
            let color : u32 = (cl << 16) | (cl << 8) | cl;

            self.draw_solid_triangle (x0, y0, z0, x1, y1, z1, x2, y2, z2, color);
        }
         println!("end");
    }

    pub fn wait_end (&mut self) {


        let texture_creator = self.sdl_canvas.texture_creator();

        let mut texture = texture_creator.create_texture_streaming(
        PixelFormatEnum::RGB24, self.width, self.height).unwrap();

        texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
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
