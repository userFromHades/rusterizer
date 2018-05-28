//use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io;
use std::io::prelude::*;
use std::mem;
use std::time;

use texture::*;

fn f <T : Sized> () -> usize{
	mem::size_of::<T>()
}


fn take<T : Sized>(file: &mut BufReader<File>) -> Result<T, io::Error> {

	// Todo Replace to array of correct size when it will be possible (https://github.com/rust-lang/rust/issues/43408)
	//const len: usize = mem::size_of::<T>();
	//let mut array = [0 as u8; len];

	let len = f::<T> ();
	let mut array = [0 as u8; 256];
	file.read_exact(&mut array[0 .. len])?;
	unsafe {
		let out: T = mem::transmute_copy(&array);
		return Ok(out)
	}
}


pub fn load_from_file (file_name : &str) -> Result< Texture, io::Error >{
	let start = time::SystemTime::now();

	let f = File::open(file_name)?;
	let f = &mut BufReader::with_capacity(1024, f);

	let id_length = take::<u8>(f)?;
	let colourmap_type = take::<u8>(f)?;
	let datatype_code= take::<u8>(f)?;

	let _colour_map_origin = take::<i16>(f)?;
	let _colour_map_length = take::<i16>(f)?;
	let colour_map_depth = take::<u8>(f)?;

	let _x_origin= take::<i16>(f)?;
	let _y_origin= take::<i16>(f)?;
	let width= take::<u16>(f)? as usize;
	let height= take::<u16>(f)? as usize;
	let bits_per_pixel = take::<u8>(f)? as usize;
	let imagedescriptor = take::<u8>(f)?;

	let size  = (width * height * bits_per_pixel / 8) as usize;

	println!("{} {} {}", id_length, colourmap_type, datatype_code);
	println!("w {} h {}", width, height);
	println!("bits {} depth {}", bits_per_pixel, colour_map_depth);
	println!("imagedescriptor {}", imagedescriptor);
	println!("total size {} bytes", size);

	// I don't worl with Image ID and Color Map Data in my paticular case it's 0
	// f.seek(io::SeekFrom::Current(id_length + colourmap_type))

	//Read as RLE RGB 24 bit
	let mut data  : Vec<u8> = Vec::new();
	data.reserve(size);
	while data.len() < size {

		let h = take::<u8>(f)?;
		let n = ((h & 0x7F)) as usize;

		if h & 0x80 != 0 {

			let r = take::<u8>(f)?;
			let g = take::<u8>(f)?;
			let b = take::<u8>(f)?;

			for  _ in 0 .. n + 1 {
				data.push(r);
				data.push(g);
				data.push(b);
			}
		}
		else{
			for _ in 0 .. n+1 {
				let r = take::<u8>(f)?;
				let g = take::<u8>(f)?;
				let b = take::<u8>(f)?;

				data.push(r);
				data.push(g);
				data.push(b);
			}
		}
	}
	println!("readed {} ", data.len());
	let end = time::SystemTime::now();
	let elapsed = end.duration_since(start).expect("");
	let s = elapsed.as_secs() as f32 + 1e-9 *(elapsed.subsec_nanos() as f32);
	println!("time elapsed {:4.3} ", s);

	return Ok(Texture::new(width, height,ColorType::RGB24,data))
}