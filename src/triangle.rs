use rand;
use rand::Rng;

type Vertex = [f32; 3];

pub struct Triangle {
    pub vertices: [Vertex; 3],
    width: f32,
    height: f32,
}

impl Triangle {
    pub fn new(width: f32, height: f32) -> Triangle {
        Triangle {
            vertices: [[-0.05, 0.0, 0.0], [0.05, 0.0, 0.0], [0.0, 0.1, 0.0]],
            width: width,
            height: height,
        }
    }

    pub fn generate_new_coordinates(&mut self) {
        let x = rand::thread_rng().gen_range(-0.9..0.8);
        let y = rand::thread_rng().gen_range(-0.9..0.8);

        self.vertices[0][0] = x;
        self.vertices[0][1] = y;

        self.vertices[1][0] = x + self.width;
        self.vertices[1][1] = y;

        self.vertices[2][0] = x + self.width / 2.0;
        self.vertices[2][1] = y + self.height;
    }

    // down, left, right
    fn calculate_lengths(&self) -> [f32; 3] {
        let mut res = [0.0; 3];

        for i in 1..3 {
            res[i - 1] = ((self.vertices[0][0] - self.vertices[i][0]).powf(2.0)
                + (self.vertices[0][1] - self.vertices[i][1]).powf(2.0))
            .sqrt();
        }

        res[2] = ((self.vertices[1][0] - self.vertices[2][0]).powf(2.0)
            + (self.vertices[1][1] - self.vertices[2][1]).powf(2.0))
        .sqrt();

        res
    }

    // down, left, right
    fn calculate_midpoints(&self) -> [Vertex; 3] {
        let mut res = [[0.0; 3]; 3];

        for i in 1..3 {
            res[i - 1][0] = (self.vertices[0][0] + self.vertices[i][0]) / 2.0;
            res[i - 1][1] = (self.vertices[0][1] + self.vertices[i][1]) / 2.0;
        }

        res[2][0] = (self.vertices[1][0] + self.vertices[2][0]) / 2.0;
        res[2][1] = (self.vertices[1][1] + self.vertices[2][1]) / 2.0;

        res
    }

    pub fn in_triangle(&self, click_x: &f32, click_y: &f32) -> bool {
        let midpoints = self.calculate_midpoints();
        let lengths = self.calculate_lengths();

        let mut area_sum = 0.0;
        for i in 0..2 {
            let distance_from_midpoint = ((midpoints[i][0] - click_x).powf(2.0)
                + (midpoints[i][1] - click_y).powf(2.0))
            .sqrt();
            area_sum += distance_from_midpoint * lengths[i] / 2.0;
        }
        let triangle_area = self.width * self.height / 2.0;

        area_sum < triangle_area + self.height * 0.01
    }
}
