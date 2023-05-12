#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use beryllium::*;
use ogl33::*;
use std::mem::size_of;
use std::time::{SystemTime, UNIX_EPOCH};
use trxsh::shader_program::ShaderProgram;
use trxsh::triangle::Triangle;
use trxsh::vao::VertexArray;
use trxsh::vbo::Buffer;

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

const WINDOW_HEIGHT: i32 = 800;
const WINDOW_WIDTH: i32 = 800;

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
            WINDOW_WIDTH as u32,
            WINDOW_HEIGHT as u32,
            WindowFlags::Shown,
        )
        .expect("Couldn't make a window and context");
    win.set_swap_interval(SwapInterval::Vsync);

    let mut triangle = Triangle::new(0.1, 0.1);

    unsafe {
        load_gl_with(|f_name| win.get_proc_address(f_name));
        glClearColor(0.1, 0.1, 0.1, 1.0);
    }

    let vao = VertexArray::new().expect("Couldn't make a VAO");
    vao.bind();

    let vbo = Buffer::new(GL_ARRAY_BUFFER).expect("Couldn't make a VBO");
    vbo.bind();

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

    vbo.buffer_data(bytemuck::cast_slice(
        concat_vertex_arrays(
            &triangle.vertices,
            &[[-0.025, 0.9, 0.0], [0.025, 0.9, 0.0], [0.0, 0.95, 0.0]],
        )
        .as_slice(),
    ));
    draw(2);
    win.swap_window();

    let mut start = false;
    let mut clicked = false;

    let current_time = || {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time problem")
    };

    let mut time = current_time();

    'main_loop: loop {
        while let Some(event) = sdl.poll_events().and_then(Result::ok) {
            match event {
                Event::Quit(_) => break 'main_loop,
                Event::MouseButton(mouse_event) => {
                    let click_x =
                        ((mouse_event.x_pos - WINDOW_WIDTH / 2) * 2) as f32 / WINDOW_WIDTH as f32;
                    let click_y = ((-mouse_event.y_pos + WINDOW_HEIGHT / 2) * 2) as f32
                        / WINDOW_HEIGHT as f32;

                    clicked = triangle.in_triangle(&click_x, &click_y);
                    if clicked {
                        start = true;
                    }
                }
                _ => (),
            }
        }

        if start && clicked {
            if (current_time() - time).as_millis() > 1000 {
                break 'main_loop;
            }
            clicked = false;
            triangle.generate_new_coordinates();
            vbo.overwrite(bytemuck::cast_slice(&triangle.vertices));
            draw(1);
            win.swap_window();
            time = current_time();
        }
    }
}

fn draw(count: i32) {
    unsafe {
        glClear(GL_COLOR_BUFFER_BIT);
        glDrawArrays(GL_TRIANGLES, 0, 3 * count);
    }
}

pub fn concat_vertex_arrays(first: &[Vertex; 3], second: &[Vertex; 3]) -> Vec<Vertex> {
    let mut v = Vec::with_capacity(first.len() + second.len());
    v.extend_from_slice(first);
    v.extend_from_slice(second);
    v
}
