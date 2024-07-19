use std::f32::consts::PI;

use macroquad::{
    camera::{Camera3D, Projection},
    input::{
        is_key_down, is_key_pressed, mouse_delta_position, set_cursor_grab, show_mouse, KeyCode,
    },
    math::Vec3,
    window::set_fullscreen,
};

const CAMERA_SPEED: f32 = 1.0;

pub struct CameraState {
    position: Vec3,
    pitch: f32,
    yaw: f32,

    mouse_locked: bool,
}

impl CameraState {
    pub fn from_pos(pos: Vec3) -> Self {
        Self {
            position: pos,
            pitch: 0.0,
            yaw: 0.0,

            mouse_locked: false,
        }
    }

    pub fn update(&mut self, dt: f32) {
        //change position according to keyboard inputs
        let direction = self.direction();

        //movement along look direction
        if is_key_down(KeyCode::W) {
            self.position += direction * CAMERA_SPEED * dt;
        }
        if is_key_down(KeyCode::S) {
            self.position -= direction * CAMERA_SPEED * dt;
        }
        if is_key_down(KeyCode::D) {
            self.position += direction.cross(Vec3::Z) * CAMERA_SPEED * dt;
        }
        if is_key_down(KeyCode::A) {
            self.position -= direction.cross(Vec3::Z) * CAMERA_SPEED * dt;
        }
        //movement along UP/DOWN
        if is_key_down(KeyCode::Space) {
            self.position += Vec3::Z * CAMERA_SPEED * dt;
        }
        if is_key_down(KeyCode::LeftShift) {
            self.position -= Vec3::Z * CAMERA_SPEED * dt;
        }

        //change pitches and yaws
        //pracuj pouze se zamčenou myší
        if self.mouse_locked {
            let mouse_delta = mouse_delta_position();
            self.yaw += mouse_delta.x * 0.80;
            self.pitch += mouse_delta.y * 0.80;
            //clamp pitch
            if self.pitch >= PI * 0.5 {
                self.pitch = PI * 0.5;
            } else if self.pitch <= -PI * 0.5 {
                self.pitch = -PI * 0.5;
            }
        }

        //lock mouse if user enters fullscreen
        if is_key_pressed(KeyCode::F10) {
            if self.mouse_locked {
                set_cursor_grab(false);
                show_mouse(true);
                set_fullscreen(false);
                self.mouse_locked = false;
            } else {
                set_cursor_grab(true);
                set_fullscreen(true);
                show_mouse(false);
                self.mouse_locked = true;
            }
        }
    }

    #[inline]
    pub fn direction(&self) -> Vec3 {
        //METH source https://learnopengl.com/Getting-started/Camera
        //first rotate by yaw
        let yaw_vec = Vec3::new(self.yaw.cos(), self.yaw.sin(), 1.0);
        //then rotate by pitch
        Vec3::new(self.pitch.cos(), self.pitch.cos(), self.pitch.sin()) * yaw_vec
    }

    pub fn as_camera(&self) -> Camera3D {
        Camera3D {
            position: self.position,
            target: self.position + self.direction(),
            up: Vec3::Z,
            fovy: 45.0,
            aspect: None,
            projection: Projection::Perspective,
            render_target: None,
            viewport: None,
        }
    }
}
