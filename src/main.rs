// #![deny(missing_docs)]

use std::f32;

use rastr::math::Float3;
use rastr::render::{RenderTarget, color_buffer_to_byte_array, depth_buffer_to_byte_array};
use rastr::scene::Scene;
use raylib::prelude::*;

fn run(target: &mut RenderTarget, scene: &mut Scene) {
    let (mut rl, thread) = raylib::init()
        .size(target.width as i32, target.height as i32)
        .title("Software Rasterizer")
        .build();
    let mut initial_frames = 2;
    rl.disable_cursor();

    // Framebuffer
    let mut texture = rl
        .load_texture_from_image(
            &thread,
            &Image::gen_image_color(target.width as i32, target.height as i32, Color::WHITE),
        )
        .unwrap();
    let mut texture_bytes: Vec<u8> = Vec::new();
    texture_bytes.resize(target.width * target.height * 4, 0); // RGBA

    // Z-Buffer
    let depth_img = Image::gen_image_color(target.width as i32, target.height as i32, Color::WHITE)
        .from_channel(0);
    let mut depth_texture = rl.load_texture_from_image(&thread, &depth_img).unwrap();
    let mut depth_texture_bytes: Vec<u8> = Vec::new();
    depth_texture_bytes.resize(target.width * target.height, 0); // Grayscale

    let mut show_depth = false;

    // Spotlight depth buffer
    let spotlight = scene.spotlights[0].borrow();
    let spotlight_depth_img = Image::gen_image_color(spotlight.shadow_map.width as i32, spotlight.shadow_map.height as i32, Color::WHITE).from_channel(0);
    let mut spotlight_depth_texture = rl
        .load_texture_from_image(&thread, &spotlight_depth_img)
        .unwrap();
    let mut spotlight_depth_texture_bytes: Vec<u8> = Vec::new();
    spotlight_depth_texture_bytes.resize(spotlight.shadow_map.width * spotlight.shadow_map.height, 0); // Grayscale
    drop(spotlight);

    let mut show_spotlight_depth = false;

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

        if rl.is_key_pressed(KeyboardKey::KEY_F) {
            show_depth = !show_depth;
        }

        if rl.is_key_pressed(KeyboardKey::KEY_G) {
            show_spotlight_depth = !show_spotlight_depth;
        }

        // Update and rasterize scene
        target.clear(Float3::new(0.0, 0.0, 0.0));
        scene.spotlights[0].borrow_mut().shadow_map.image.fill(1.0);
        target.render(scene);

        if !show_depth {
            // Write rasterizer output to texture and display on window
            color_buffer_to_byte_array(
                &target.color_buffer,
                target.width,
                target.height,
                &mut texture_bytes,
            );
            // target.depth_buffer_to_byte_array(&mut texture_bytes);
            texture.update_texture(&texture_bytes).unwrap();
        } else {
            depth_buffer_to_byte_array(
                &target.depth_buffer,
                target.width,
                target.height,
                scene.camera.near,
                scene.camera.far,
                true,
                &mut depth_texture_bytes,
            );
            depth_texture.update_texture(&depth_texture_bytes).unwrap();
        }

        if show_spotlight_depth {
            let spotlight = scene.spotlights[0].borrow();
            depth_buffer_to_byte_array(
                &spotlight.shadow_map.image,
                spotlight.shadow_map.width,
                spotlight.shadow_map.height,
                spotlight.camera.near,
                spotlight.camera.far,
                true,
                &mut spotlight_depth_texture_bytes,
            );
            spotlight_depth_texture.update_texture(&spotlight_depth_texture_bytes).unwrap();
        }

        let mut d = rl.begin_drawing(&thread);
        if !show_depth {
            d.draw_texture(&texture, 0, 0, Color::WHITE);
        } else {
            d.draw_texture(&depth_texture, 0, 0, Color::WHITE);
        }

        if show_spotlight_depth {
            d.draw_texture(
                &spotlight_depth_texture,
                target.width as i32 - spotlight_depth_texture.width - 10,
                10,
                Color::WHITE,
            );
        }

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
