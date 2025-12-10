use std::{ffi::CString, path::PathBuf};

use asset_importer::TextureInfo;
pub struct Texture {
  texture: TextureInfo,
  pub id: u32,
}

impl Texture {
  pub fn new(texture: TextureInfo) -> Self {
    Self { texture, id: 0 }
  }

  #[allow(clippy::ptr_arg)]
  pub fn load(&mut self, model_path: &PathBuf) {
    let path = &self.texture.path;
    let mut tex_path = model_path.clone();
    tex_path.pop();
    tex_path.push(path);
    if let Ok(img) = image::open(tex_path) {
      let rgba = img.flipv().to_rgba8();
      let (w, h) = rgba.dimensions();
      let mut data: Vec<u8> = Vec::new();
      data.extend_from_slice(&rgba.into_raw());
      unsafe {
        gl::GenTextures(1, &mut self.id);
        gl::ActiveTexture(gl::TEXTURE0);
        gl::BindTexture(gl::TEXTURE_2D, self.id);
        gl::TexParameteri(
          gl::TEXTURE_2D,
          gl::TEXTURE_MIN_FILTER,
          gl::LINEAR_MIPMAP_LINEAR as i32,
        );
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        gl::TexImage2D(
          gl::TEXTURE_2D,
          0,
          gl::RGBA as i32,
          w as i32,
          h as i32,
          0,
          gl::RGBA,
          gl::UNSIGNED_BYTE,
          data.as_ptr() as *const _,
        );
        gl::GenerateMipmap(gl::TEXTURE_2D);
      }
    };
  }
  pub unsafe fn draw(&mut self, program: u32) {
    unsafe {
      gl::BindTexture(gl::TEXTURE_2D, self.id);
      gl::ActiveTexture(gl::TEXTURE0);
      let texture_loc = gl::GetUniformLocation(program, CString::new("texture0").unwrap().as_ptr());
      gl::Uniform1i(texture_loc, 0);
    }
  }
}
