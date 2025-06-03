use raylib::prelude::*;
use rastr::render::RenderTarget;
use rastr::scene::Scene;
use rastr::math::Float3;

fn run(target: &mut RenderTarget, scene: &mut Scene) {
    let (mut rl, thread) = raylib::init()
        .size(target.width as i32, target.height as i32)
        .title("Software Rasterizer")
        .build();
    let mut texture = rl
        .load_texture_from_image(
            &thread,
            &Image::gen_image_color(target.width as i32, target.height as i32, Color::WHITE),
        )
        .unwrap();
    let mut texture_bytes: Vec<u8> = Vec::new();
    texture_bytes.resize(target.width * target.height * 4, 0); // RGBA

    // Render loop
    while !rl.window_should_close() {
        // Update and rasterize scene
        scene.update(target, &rl);
        target.clear(Float3::new(0.0, 0.0, 0.0));
        target.render(&scene);

        // Write rasterizer output to texture and display on window
        target.color_buffer_to_byte_array(&mut texture_bytes);
        texture.update_texture(&texture_bytes).unwrap();

        let mut d = rl.begin_drawing(&thread);
        d.draw_texture(&texture, 0, 0, Color::WHITE);
    }
}

fn main() {
    const WIDTH: usize = 640;
    const HEIGHT: usize = 480;

    let mut target = RenderTarget::new(WIDTH, HEIGHT);
    let mut scene = Scene::new();

    run(&mut target, &mut scene)
}
