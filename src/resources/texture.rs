use std::{ffi::CString, path::PathBuf};

use asset_importer::TextureInfo;
pub struct Texture {
  texture: Option<TextureInfo>,
  pub id: u32,
}

impl Texture {
  pub fn new(texture: Option<TextureInfo>) -> Self {
    Self {
      texture: texture,
      id: 0,
    }
  }

  // if there is a texture, this will load the path of the texture
  // otherwise it will load the fallback texture
  #[allow(clippy::ptr_arg)]
  pub fn load(&mut self, model_path: &PathBuf) {
    let mut tex_path: PathBuf;
    if let Some(texture) = &mut self.texture {
      let path = texture.path.clone();
      tex_path = model_path.clone();
      tex_path.pop();
      tex_path.push(path);
    } else {
      tex_path = PathBuf::from("assets/textures/empty.png");
    }
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
    } else {
      println!("Could not load any texture!");
    }
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
