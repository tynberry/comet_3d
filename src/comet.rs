mod arch_line;

use arch_line::ArchLine;
use macroquad::prelude::*;

const COLORS: [Color; 7] = [
    RED,
    YELLOW,
    GREEN,
    BLUE,
    Color {
        r: 0.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    },
    WHITE,
    MAGENTA,
];

struct Line {
    a: Vec3,
    b: Vec3,
    color: Color,
}

impl Line {
    fn render(&self) {
        draw_line_3d(self.a, self.b, self.color);
    }
}

pub struct CometState {
    lines: Vec<Line>,
}

impl CometState {
    pub fn new() -> Self {
        //generate arch lines
        //(arch lines are pure meth representation on strings)
        let mut arch_lines = vec![];

        for _ in 0..6 {
            arch_lines.push(ArchLine::generate());
        }

        //generate render lines
        //(render lines are sampled arch lines)
        const SAMPLE_COUNT: usize = 4096;
        const SAMPLE_DISTANCE: f32 = 0.05;
        const SCALE: f32 = 1.20;

        let mut lines = vec![];

        for (arch_id, arch) in arch_lines.iter().enumerate() {
            let mut nodes = [Vec3::ZERO; SAMPLE_COUNT];

            //generate nodes
            for sample in 0..SAMPLE_COUNT {
                let t = sample as f32 * SAMPLE_DISTANCE;
                nodes[sample] = arch.sample(t) * (Vec3::X * SCALE + Vec3::Y + Vec3::Z);
            }

            //connect nodes
            for window in nodes.windows(2) {
                let [node_a, node_b] = window else {
                    break;
                };

                lines.push(Line {
                    a: *node_a,
                    b: *node_b,
                    color: COLORS[arch_id % COLORS.len()],
                });
            }
        }

        Self { lines }
    }

    pub fn render(&self) {
        //draw center of comet
        draw_sphere(Vec3::ZERO, 0.2, None, Color::from_hex(0x00ffff));
        draw_sphere_wires(Vec3::ZERO, 0.2, None, BLACK);
        //draw lines of the comet
        for line in &self.lines {
            line.render();
        }
    }
}
