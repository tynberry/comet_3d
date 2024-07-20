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

    trig_ampl_ang: f32,
    trig_freq_ang: f32,
    trig_shift_ang: f32,

    lin_cone: f32,
    sin_freq_cone: f32,
}

impl ArchLine {
    pub fn generate() -> Self {
        Self {
            //constant angle
            const_ang: gen_f32(0.0, 2.0 * PI),

            //"normal" angle functions
            lin_ang: gen_f32(-0.40, 0.40),
            sin_ang: gen_f32(-0.2, 0.2),
            cos_ang: gen_f32(-0.25, 0.25),

            //custom trigonometry function
            trig_ampl_ang: gen_f32(-0.1, 0.1),
            trig_freq_ang: gen_f32(0.08, 0.24),
            trig_shift_ang: gen_f32(0.0, 2.0 * PI),

            //cone shape functions
            lin_cone: gen_f32(0.10, 0.12),
            sin_freq_cone: gen_f32(0.05, 0.16),
        }
    }

    pub fn sample(&self, t: f32) -> Vec3 {
        //compute cone size at distance
        let cone_size = self.lin_cone * t * (t * self.sin_freq_cone).sin();
        //compute angle at distance
        let angle =
            self.const_ang + self.lin_ang * t + self.sin_ang * t.sin() + self.cos_ang * t.cos();
        //add custom trig function
        let angle =
            angle + (t * self.trig_freq_ang + self.trig_shift_ang).sin() * self.trig_ampl_ang;
        //compute the point itself
        Vec3::new(t, angle.sin() * cone_size, angle.cos() * cone_size)
    }
}
