extern crate gl;
extern crate glfw;
extern crate nalgebra_glm as glm;

use std::{ffi::CString, os::raw::c_void, ptr, str};

use gl::{DEPTH_TEST, types::*};
use glfw::{Context, ffi::GLFW_FALSE};
use glm::{Vec3, vec3};

static VS_SRC: &str = include_str!("v.glsl");
static FS_SRC: &str = include_str!("f.glsl");

fn compile_shader(src: &str, ty: GLenum) -> GLuint {
    let shader;
    unsafe {
        shader = gl::CreateShader(ty);
        let c_str = CString::new(src.as_bytes()).unwrap();
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
        gl::CompileShader(shader);
        let mut status = gl::FALSE as GLint;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);
        if status != (gl::TRUE as GLint) {
            println!("shader not valid");
        }
    };
    shader
}

fn link_program(fs: GLuint, vs: GLuint) -> GLuint {
    unsafe {
        let program = gl::CreateProgram();
        gl::AttachShader(program, vs);
        gl::AttachShader(program, fs);
        gl::LinkProgram(program);
        let mut status = gl::FALSE as GLint;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);
        if status != (gl::TRUE as GLint) {
            println!("Shader didn't link")
        }
        program
    }
}

fn main() {
    let obj_file = std::env::args()
        .skip(1)
        .next()
        .expect("An obj file is required!");
    let (models, _) =
        tobj::load_obj(&obj_file, &tobj::LoadOptions::default()).expect("Failed to load the .obj");
    let model = models.iter().next().expect("Failed to extract mesh");
    let mesh = &model.mesh.positions;
    let indices = &model.mesh.indices;
    println!("{:?}", mesh.len());
    // initialize GLFW
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));
    glfw.window_hint(glfw::WindowHint::OpenGlDebugContext(true));

    let camera = glm::look_at_rh(&vec3(10.0, 10.0, 10.0), &vec3(0.0, 0.0, 0.0), &Vec3::y());
    let model = glm::identity::<f32, 4>();
    let proj = glm::perspective::<f32>(1.0, glm::half_pi::<f32>() * 0.8, 0.1, 100.0);

    // initialize openGL context & load function pointer
    let (mut window, events) = glfw
        .create_window(600, 600, "Hi!!!", glfw::WindowMode::Windowed)
        .unwrap();
    window.make_current();
    window.set_key_polling(true);
    gl::load_with(|symbol| window.get_proc_address(symbol).unwrap() as *const _);
    let vs = compile_shader(VS_SRC, gl::VERTEX_SHADER);
    let fs = compile_shader(FS_SRC, gl::FRAGMENT_SHADER);
    let program = link_program(fs, vs);

    let mut vao = 0;
    let mut vbo = 0;
    let mut ebo = 0;

    unsafe {
        gl::Enable(DEPTH_TEST);
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            size_of_val(&mesh) as GLsizeiptr,
            mesh.as_ptr().cast(),
            gl::STATIC_DRAW,
        );
        gl::GenBuffers(1, &mut ebo);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            size_of_val(&indices) as GLsizeiptr,
            indices.as_ptr().cast(),
            gl::STATIC_DRAW,
        );
        gl::UseProgram(program);
        gl::BindFragDataLocation(program, 0, CString::new("out_color").unwrap().as_ptr());
        let pos_attr = gl::GetAttribLocation(program, CString::new("position").unwrap().as_ptr());
        gl::EnableVertexAttribArray(pos_attr as GLuint);
        gl::VertexAttribPointer(
            pos_attr as GLuint,
            3,
            gl::FLOAT,
            gl::FALSE as GLboolean,
            3 * size_of::<GLfloat>() as i32,
            0 as *const c_void,
        );
        gl::Viewport(0, 0, 600, 600);
    }

    while !window.should_close() {
        unsafe {
            let view_loc = gl::GetUniformLocation(program, CString::new("view").unwrap().as_ptr());
            gl::UniformMatrix4fv(
                view_loc,
                1,
                GLFW_FALSE as u8,
                &camera as *const _ as *const _,
            );

            let proj_loc = gl::GetUniformLocation(program, CString::new("proj").unwrap().as_ptr());
            gl::UniformMatrix4fv(proj_loc, 1, GLFW_FALSE as u8, &proj as *const _ as *const _);

            let model_loc =
                gl::GetUniformLocation(program, CString::new("model").unwrap().as_ptr());
            gl::UniformMatrix4fv(
                model_loc,
                1,
                GLFW_FALSE as u8,
                &model as *const _ as *const _,
            );
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::UseProgram(program);
            gl::BindVertexArray(vao);

            gl::DrawElements(gl::TRIANGLES, 20, gl::UNSIGNED_INT, 0 as *const c_void);
        }
        glfw.poll_events();
        window.swap_buffers();
        for (_, _event) in glfw::flush_messages(&events) {}
    }
}
