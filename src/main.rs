#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use beryllium::*;
use ogl33::*;
use std::mem::size_of;
use trxsh::shader_program::ShaderProgram;
use trxsh::vao::VertexArray;

const VERT_SHADER: &str = r#"#version 330 core
  layout (location = 0) in vec3 pos;
  void main() {
    gl_Position = vec4(pos.x, pos.y, pos.z, 1.0);
  }
"#;

const FRAG_SHADER: &str = r#"#version 330 core
  out vec4 final_color;

  void main() {
    final_color = vec4(1.0, 0.0, 0.0, 1.0);
  }
"#;

type Vertex = [f32; 3];

fn main() {
    let sdl = SDL::init(InitFlags::Everything).expect("Couldn't start SDL");

    sdl.gl_set_attribute(SdlGlAttr::MajorVersion, 3).unwrap();
    sdl.gl_set_attribute(SdlGlAttr::MinorVersion, 3).unwrap();
    sdl.gl_set_attribute(SdlGlAttr::Profile, GlProfile::Core)
        .unwrap();

    let win = sdl
        .create_gl_window(
            "trxsh",
            WindowPosition::Centered,
            800,
            800,
            WindowFlags::Shown,
        )
        .expect("Couldn't make a window and context");
    win.set_swap_interval(SwapInterval::Vsync);

    let mut vertices: [Vertex; 3] = [[-1.0, -0.5, 0.0], [0.5, -0.5, 0.0], [0.0, 0.0, 0.0]];

    unsafe {
        load_gl_with(|f_name| win.get_proc_address(f_name));
        glClearColor(0.1, 0.1, 0.1, 1.0);
    }

    let vao = VertexArray::new().expect("Couldn't make a VAO");
    vao.bind();

    let vbo = Buffer::new().expect("Couldn't make a VBO");
    vbo.bind(BufferType::Array);

    let shader_program = ShaderProgram::from_vert_frag(VERT_SHADER, FRAG_SHADER).unwrap();
    shader_program.use_program();

    unsafe {
        glVertexAttribPointer(
            0,
            3,
            GL_FLOAT,
            GL_FALSE,
            size_of::<Vertex>().try_into().unwrap(),
            0 as *const _,
        );
        glEnableVertexAttribArray(0);
    }

    buffer_data(bytemuck::cast_slice(&vertices));
    draw();
    win.swap_window();

    let mut update = false;
    'main_loop: loop {
        while let Some(event) = sdl.poll_events().and_then(Result::ok) {
            match event {
                Event::Quit(_) => break 'main_loop,
                Event::Keyboard(keyboard_event) => {
                    update = true;
                    match keyboard_event.key.keycode.0 {
                        119 => {
                            vertices[0][0] = 1.0;
                        }
                        100 => {
                            vertices[0][0] = -0.5;
                        }
                        _ => update = false,
                    }
                }
                _ => (),
            }
        }
        if update == true {
            overwrite(bytemuck::cast_slice(&vertices));
            draw();
            win.swap_window();
            update = false;
        }
    }
}

fn draw() {
    unsafe {
        glClear(GL_COLOR_BUFFER_BIT);
        glDrawArrays(GL_TRIANGLES, 0, 3);
    }
}

pub enum BufferType {
    Array = GL_ARRAY_BUFFER as isize,
    ElementArray = GL_ELEMENT_ARRAY_BUFFER as isize,
}

pub struct Buffer(pub GLuint);

impl Buffer {
    pub fn new() -> Option<Self> {
        let mut vbo = 0;
        unsafe {
            glGenBuffers(1, &mut vbo);
        }
        if vbo != 0 {
            Some(Self(vbo))
        } else {
            None
        }
    }

    pub fn bind(&self, ty: BufferType) {
        unsafe { glBindBuffer(ty as GLenum, self.0) }
    }

    pub fn clear_binding(ty: BufferType) {
        unsafe { glBindBuffer(ty as GLenum, 0) }
    }
}

pub fn buffer_data(data: &[u8]) {
    unsafe {
        glBufferData(
            GL_ARRAY_BUFFER,
            data.len().try_into().unwrap(),
            data.as_ptr().cast(),
            GL_DYNAMIC_DRAW,
        );
    }
}

pub fn overwrite(data: &[u8]) {
    unsafe {
        glBufferSubData(
            GL_ARRAY_BUFFER,
            0,
            data.len().try_into().unwrap(),
            data.as_ptr().cast(),
        );
    }
}
