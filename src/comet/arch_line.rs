use macroquad::math::Vec3;

pub struct ArchLine {
    sin_y: [f32; 4],
    cos_y: [f32; 4],
    sin_z: [f32; 4],
    cos_z: [f32; 4],
}

impl ArchLine {
    pub fn generate() -> Self {
        let mut sin_y = [0.0; 4];
        let mut cos_y = [0.0; 4];
        let mut sin_z = [0.0; 4];
        let mut cos_z = [0.0; 4];

        //generate completely random parameters
        for i in 0..sin_y.len() {
            sin_y[i] = (fastrand::f32() * 2.0 - 1.0) * 0.8_f32.powi(i as i32);
            sin_z[i] = (fastrand::f32() * 2.0 - 1.0) * 0.8_f32.powi(i as i32);
        }
        for i in 0..cos_y.len() {
            cos_y[i] = (fastrand::f32() * 2.0 - 1.0) * 0.8_f32.powi(i as i32);
            cos_z[i] = (fastrand::f32() * 2.0 - 1.0) * 0.8_f32.powi(i as i32);
        }

        //normalize cosines to start
        let cos_y_sum = cos_y.iter().fold(0.0, |acc, a| acc + *a);
        let cos_z_sum = cos_z.iter().fold(0.0, |acc, a| acc + *a);
        cos_y[0] -= cos_y_sum;
        cos_z[0] -= cos_z_sum;

        Self {
            sin_y,
            cos_y,
            sin_z,
            cos_z,
        }
    }

    pub fn sample(&self, t: f32) -> Vec3 {
        //compute values
        let mut sum_y = 0.0;
        let mut sum_z = 0.0;

        self.sin_y
            .iter()
            .zip(self.sin_z.iter())
            .enumerate()
            .for_each(|(ind, (sin_y, sin_z))| {
                sum_y += (t * (ind as f32 + 1.0)).sin() * sin_y;
                sum_z += (t * (ind as f32 + 1.0)).sin() * sin_z;
            });

        self.cos_y
            .iter()
            .zip(self.cos_z.iter())
            .enumerate()
            .for_each(|(ind, (sin_y, sin_z))| {
                sum_y += (t * (ind as f32 + 1.0)).cos() * sin_y;
                sum_z += (t * (ind as f32 + 1.0)).cos() * sin_z;
            });

        Vec3::new(t, sum_y, sum_z)
    }
}
