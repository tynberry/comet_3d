pub mod camera;
pub mod comet;

use camera::CameraState;
use comet::CometState;
use macroquad::prelude::*;

#[macroquad::main("Comet 3D")]
async fn main() {
    //init camera state
    let mut camera_state = CameraState::from_pos(vec3(10.0, 0.0, 0.0));

    //create comet
    let comet_state = CometState::new();

    loop {
        //update camera
        let dt = get_frame_time();
        camera_state.update(dt);

        //start rendering
        clear_background(BLACK);

        //set 3D camera
        set_camera(&camera_state.as_camera());

        //draw comet
        comet_state.render();

        //end rendering
        next_frame().await;
    }
}
