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
const DEFAULT_MILLIS_LIMIT: u128 = 1000;

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

    vbo.buffer_data(bytemuck::cast_slice(&triangle.vertices));
    draw(1);
    win.swap_window();

    let mut start = false;
    let mut clicked = false;

    let current_time = || {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time problem")
    };

    let mut time = current_time();
    let mut time_limit = DEFAULT_MILLIS_LIMIT;
    let mut score = 0;

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
                        if !start {
                            triangle.generate_new_coordinates();
                            vbo.buffer_data(bytemuck::cast_slice(&triangle.vertices));
                            score = 0;
                            start = true;
                            time = current_time();
                        } else {
                            score += 1;
                        }
                    }
                }
                _ => (),
            }
        }

        if start && clicked {
            clicked = false;
            if (current_time() - time).as_millis() > time_limit {
                start = false;
                time_limit = DEFAULT_MILLIS_LIMIT;
                triangle.reset_to_default();
                vbo.buffer_data(bytemuck::cast_slice(
                    concat_triangle_with_score_grid(
                        &triangle.vertices,
                        &mut generate_score_grid(score, 0.05),
                    )
                    .as_slice(),
                ));
                draw(score + 1);
                win.swap_window();
                continue;
            }

            time_limit -= (DEFAULT_MILLIS_LIMIT as f32 * 0.005) as u128;
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

fn concat_triangle_with_score_grid(
    triangle: &[Vertex; 3],
    score_grid: &mut Vec<Vertex>,
) -> Vec<Vertex> {
    let mut v = Vec::with_capacity(triangle.len() + score_grid.len() * 3);
    v.extend_from_slice(triangle);
    v.append(score_grid);
    v
}

fn generate_score_grid(score: i32, size: f32) -> Vec<Vertex> {
    let mut res: Vec<Vertex> = Vec::new();

    let mut current_x = -1.0 + size * 0.5;
    let mut current_y = 1.0 - size - 0.5 * size;

    let column_count = (WINDOW_HEIGHT as f32 * size * 0.5) as i32;
    let row_count = score / column_count;

    for _ in 0..row_count {
        for _ in 0..column_count {
            res.push([current_x, current_y, 0.0]);
            res.push([current_x + size, current_y, 0.0]);
            res.push([current_x + size / 2.0, current_y + size, 0.0]);
            current_y -= size * 2.0;
        }
        current_x += size * 2.0;
        current_y = 1.0 - size - 0.5 * size;
    }

    for _ in 0..score % column_count {
        res.push([current_x, current_y, 0.0]);
        res.push([current_x + size, current_y, 0.0]);
        res.push([current_x + size / 2.0, current_y + size, 0.0]);
        current_y -= size * 2.0;
    }

    res
}
