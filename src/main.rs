use raylib::prelude::*;

fn main() {
    const WIDTH: i32 = 800;
    const HEIGHT: i32 = 600;

    let (mut rl, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("flow field rust")
        .build();

    rl.set_target_fps(60);

    while !rl.window_should_close() {
        let mut drawing_context = rl.begin_drawing(&thread);

        drawing_context.clear_background(Color::BLACK);
    }
}
