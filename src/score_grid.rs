use crate::triangle::Vertex;

pub fn concat_triangle_with_score_grid(
    triangle: &[Vertex; 3],
    score: i32,
    element_size: f32,
    window_height: f32,
) -> Vec<Vertex> {
    let mut v = Vec::with_capacity(triangle.len() + score as usize * 3);
    v.extend_from_slice(triangle);
    v.append(&mut generate_score_grid(score, element_size, window_height));
    v
}

fn generate_score_grid(score: i32, element_size: f32, window_height: f32) -> Vec<Vertex> {
    let mut res: Vec<Vertex> = Vec::new();

    let mut current_x = -1.0 + element_size * 0.5;
    let mut current_y = 1.0 - 1.5 * element_size;

    let column_count = (window_height * element_size * 0.5) as i32;
    let row_count = score / column_count;

    for _ in 0..row_count {
        for _ in 0..column_count {
            res.push([current_x, current_y, 0.0]);
            res.push([current_x + element_size, current_y, 0.0]);
            res.push([
                current_x + element_size / 2.0,
                current_y + element_size,
                0.0,
            ]);
            current_y -= element_size * 2.0;
        }
        current_x += element_size * 2.0;
        current_y = 1.0 - 1.5 * element_size;
    }

    for _ in 0..score % column_count {
        res.push([current_x, current_y, 0.0]);
        res.push([current_x + element_size, current_y, 0.0]);
        res.push([
            current_x + element_size / 2.0,
            current_y + element_size,
            0.0,
        ]);
        current_y -= element_size * 2.0;
    }

    res
}
