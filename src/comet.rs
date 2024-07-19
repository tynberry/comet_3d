use macroquad::prelude::*;

struct Line {
    a: Vec3,
    b: Vec3,
}

impl Line {
    fn render(&self) {
        draw_line_3d(self.a, self.b, Color::from_hex(0x00ffff));
    }
}

pub struct CometState {
    lines: Vec<Line>,
}

impl CometState {
    pub fn new() -> Self {
        let mut lines = vec![];

        //generate some lines
        lines.push(Line {
            a: Vec3::ZERO,
            b: vec3(0.0, 10.0, 0.0),
        });

        Self { lines }
    }

    pub fn render(&self) {
        //draw center of comet
        draw_sphere(Vec3::ZERO, 1.0, None, Color::from_hex(0x00ffff));
        draw_sphere_wires(Vec3::ZERO, 1.0, None, BLACK);
        //draw lines of the comet
        for line in &self.lines {
            line.render();
        }
    }
}
