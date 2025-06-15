use rastr::math::Float3;
use rastr::render::RenderTarget;
use rastr::scene::Scene;
use raylib::prelude::*;

fn run(target: &mut RenderTarget, scene: &mut Scene) {
    let (mut rl, thread) = raylib::init()
        .size(target.width as i32, target.height as i32)
        .title("Software Rasterizer")
        .build();
    let mut initial_frames = 2;
    rl.disable_cursor();

    let mut texture = rl
        .load_texture_from_image(
            &thread,
            &Image::gen_image_color(target.width as i32, target.height as i32, Color::WHITE),
        )
        .unwrap();
    let mut texture_bytes: Vec<u8> = Vec::new();
    texture_bytes.resize(target.width * target.height * 4, 0); // RGBA

    // let depth_img = Image::gen_image_color(target.width as i32, target.height as i32, Color::WHITE)
    //     .from_channel(0);
    // let mut depth_texture = rl.load_texture_from_image(&thread, &depth_img).unwrap();
    // let mut depth_texture_bytes: Vec<u8> = Vec::new();
    // depth_texture_bytes.resize(target.width * target.height, 0); // Grayscale

    // Render loop
    while !rl.window_should_close() {
        if initial_frames > 0 {
            initial_frames -= 1;
        } else if initial_frames == 0 {
            rl.set_mouse_position(((target.width / 2) as f32, (target.height / 2) as f32));
            initial_frames = -1;
        } else {
            scene.update(target, &rl);
        }

        // Update and rasterize scene
        target.clear(Float3::new(0.0, 0.0, 0.0));
        target.render(&scene);

        // Write rasterizer output to texture and display on window
        target.color_buffer_to_byte_array(&mut texture_bytes);
        // target.depth_buffer_to_byte_array(&mut texture_bytes);
        texture.update_texture(&texture_bytes).unwrap();

        let mut d = rl.begin_drawing(&thread);
        d.draw_texture(&texture, 0, 0, Color::WHITE);
        d.draw_text(
            &format!("FPS: {}", scene.last_frame_counter),
            0,
            0,
            12,
            Color::WHITE,
        );
        d.draw_text(
            &format!(
                "Average Frame Time: {:.2}",
                scene.average_frame_time * 1000.0
            ),
            0,
            12,
            12,
            Color::WHITE,
        );
    }
}

fn main() {
    const WIDTH: usize = 1024;
    const HEIGHT: usize = 768;

    let mut target = RenderTarget::new(WIDTH, HEIGHT);
    let mut scene = Scene::new(WIDTH as f32 / HEIGHT as f32);

    run(&mut target, &mut scene)
}
