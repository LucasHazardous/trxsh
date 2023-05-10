#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use beryllium::*;
use ogl33::*;
use rand;
use rand::Rng;
use std::mem::size_of;
use std::thread;
use std::time::Duration;
use trxsh::shader_program::ShaderProgram;
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

    let mut vertices: [Vertex; 3] = [[-0.5, -0.1, 0.0], [0.1, -0.1, 0.0], [0.0, 0.0, 0.0]];

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

    vbo.buffer_data(bytemuck::cast_slice(&vertices));
    draw();
    win.swap_window();

    let mut start = false;
    let mut handle = thread::spawn(|| ());
    'main_loop: loop {
        while let Some(event) = sdl.poll_events().and_then(Result::ok) {
            match event {
                Event::Quit(_) => break 'main_loop,
                Event::Keyboard(keyboard_event) => match keyboard_event.key.keycode.0 {
                    115 => {
                        start = true;
                    }
                    _ => (),
                },
                Event::MouseButton(mouse_event) => {
                    println!(
                        "{}",
                        ((mouse_event.x_pos - WINDOW_WIDTH / 2) * 2) as f64 / WINDOW_WIDTH as f64
                    );

                    println!(
                        "{}",
                        ((-mouse_event.y_pos + WINDOW_HEIGHT / 2) * 2) as f64
                            / WINDOW_HEIGHT as f64
                    );
                }
                _ => (),
            }
        }

        if start && handle.is_finished() {
            handle = thread::spawn(|| {
                thread::sleep(Duration::from_millis(1000));
            });
            generate_object(&mut vertices, 0.1, 0.1);
            vbo.overwrite(bytemuck::cast_slice(&vertices));
            draw();
            win.swap_window();
        }
    }
}

fn draw() {
    unsafe {
        glClear(GL_COLOR_BUFFER_BIT);
        glDrawArrays(GL_TRIANGLES, 0, 3);
    }
}

fn generate_object(vertices: &mut [Vertex; 3], obj_width: f32, obj_height: f32) {
    let x = rand::thread_rng().gen_range(-0.9..0.8);
    let y = rand::thread_rng().gen_range(-0.9..0.8);

    vertices[0][0] = x;
    vertices[0][1] = y;

    vertices[1][0] = x + obj_width;
    vertices[1][1] = y;

    vertices[2][0] = x + obj_width / 2.0;
    vertices[2][1] = y + obj_height;
}
