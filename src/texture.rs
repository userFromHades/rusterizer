
pub enum ColorType {
	RGB24,
}

impl ColorType {
	pub fn size(&self) ->usize {
		match *self{
			ColorType::RGB24 => return 3
		}
	}
}

pub struct Texture {
	width : usize,
	height : usize,
	color_type : ColorType,
	data  : Vec<u8>
}

impl Texture {

	pub fn new (width : usize,
				height : usize,
				color_type : ColorType,
				data  : Vec<u8>) -> Texture
	{
		Texture {width : width, height : height, color_type : color_type, data : data}
	}

	pub fn get (&self, x : f32, y : f32) ->u32{

		let x = (self.width - 1) as f32 * x;
		let x = x as usize % self.width;

		let y = (self.height - 1) as f32 * y;
		let y = y as usize % self.height;

		let i = x  + y * self.width;
		let n = i * self.color_type.size();

		( (self.data[n + 2] as u32) << 16) | ((self.data[n + 1] as u32) << 8) | (self.data[n + 0] as u32 )

		//(((255.0 * x) as u32) << 8) | ((255.0 * y) as u32)
	}
}
