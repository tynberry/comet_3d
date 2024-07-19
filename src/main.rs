pub mod camera;

use camera::CameraState;
use macroquad::prelude::*;

#[macroquad::main("Comet 3D")]
async fn main() {
    //init camera state
    let mut camera_state = CameraState::from_pos(Vec3::ZERO);

    loop {
        //update camera
        let dt = get_frame_time();
        camera_state.update(dt);

        //start rendering
        clear_background(BLACK);

        //set 3D camera
        set_camera(&camera_state.as_camera());

        //draw testing object
        draw_sphere(vec3(2.0, 0.0, 0.0), 0.5, None, Color::from_hex(0x00ffff));

        //end rendering
        next_frame().await;
    }
}
