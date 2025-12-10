extern crate nalgebra_glm as glm;

use std::ffi::CString;

use glm::Vec3;

pub struct Camera {
  yaw: f32,
  pos: glm::Vec3,
  up: glm::Vec3,
  front: glm::Vec3,
  direction: glm::Vec3,
  matrix: glm::Mat4,
  projection: glm::Mat4,
}

impl Camera {
  pub fn update(&mut self, program: u32) {
    unsafe {
      self.update_uniforms(program);
    }
    self.matrix = glm::look_at(&self.pos, &(self.pos + self.front), &self.up)
  }
  pub fn move_local_z(&mut self, speed: f32) {
    self.pos += speed * self.front;
  }

  //
  pub fn rotate_local_y(&mut self, speed: f32) {
    self.yaw += speed;
    self.direction.x = self.yaw.to_radians().cos();
    self.direction.z = self.yaw.to_radians().sin();
    self.front = glm::normalize(&self.direction);
  }

  // updates uniforms
  unsafe fn update_uniforms(&mut self, program: u32) {
    unsafe {
      let proj_loc = gl::GetUniformLocation(program, CString::new("proj").unwrap().as_ptr());
      gl::UniformMatrix4fv(
        proj_loc,
        1,
        gl::FALSE,
        &self.projection as *const _ as *const _,
      );
      let view_loc = gl::GetUniformLocation(program, CString::new("view").unwrap().as_ptr());
      gl::UniformMatrix4fv(view_loc, 1, gl::FALSE, &self.matrix as *const _ as *const _);
    }
  }
}

impl Default for Camera {
  fn default() -> Self {
    let pos = Vec3::new(0.0, 5.0, 0.0);
    let direction = Vec3::new(0.0, 0.0, 0.0);
    let up = Vec3::new(0.0, 1.0, 0.0);
    let front = Vec3::new(0.0, 0.0, -1.0);
    Self {
      yaw: -90.0,
      pos,
      direction,
      up,
      front,
      matrix: glm::look_at(&pos, &(pos + front), &up),
      projection: glm::perspective::<f32>(1.0, 45_f32.to_radians(), 0.1, 1000.0),
    }
  }
}
