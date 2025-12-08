use std::{
  fmt::Debug,
  io::{BufRead, BufReader, Read},
  os::raw::c_void,
  path::PathBuf,
};

use asset_importer::sys::aiTexture;
use cursor::Cursor;
use image::ImageReader;
pub struct Texture {
  texture: asset_importer::Texture,
  id: u32,
}

impl Texture {
  pub fn new(texture: asset_importer::Texture) -> Self {
    Self {
      texture: texture,
      id: 0,
    }
  }
  pub fn load(self: &mut Self) {
    let path = self.texture.filename().expect("There should be a filename");
    println!("{}", path);
    unsafe {
      // I can't decompress the self.data so it can load into OpenGL correctly
      gl::GenTextures(1, &mut self.id);
      gl::BindTexture(gl::TEXTURE_2D, self.id);
    }
  }
}
