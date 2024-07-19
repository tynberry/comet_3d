use std::f32::consts::PI;

use macroquad::math::Vec3;

fn gen_f32(min: f32, max: f32) -> f32 {
    fastrand::f32() * (max - min) + min
}

pub struct ArchLine {
    const_ang: f32,
    lin_ang: f32,
    sin_ang: f32,
    cos_ang: f32,

    lin_cone: f32,
}

impl ArchLine {
    pub fn generate() -> Self {
        Self {
            const_ang: gen_f32(0.0, 2.0 * PI),
            lin_ang: gen_f32(-0.40, 0.40),
            sin_ang: gen_f32(-0.5, 0.5),
            cos_ang: gen_f32(-0.25, 0.25),
            lin_cone: gen_f32(0.10, 0.12),
        }
    }

    pub fn sample(&self, t: f32) -> Vec3 {
        //compute cone size at distance
        let cone_size = self.lin_cone * t;
        //compute angle at distance
        let angle =
            self.const_ang + self.lin_ang * t + self.sin_ang * t.sin() + self.cos_ang * t.cos();
        //compute the point itself
        Vec3::new(t, angle.sin() * cone_size, angle.cos() * cone_size)
    }
}
