use sdl2::{gfx::primitives::DrawRenderer, pixels::Color, event::Event, keyboard::Keycode};
use std::time::Duration;
use rand;

mod circles;
mod scene;
use scene::Scene;

fn main() {
    let size = (800, 800);
    let circles_per_frame = 10;
    let rate = 1.0f64;
    let mut scene = Scene::new(size, rate);
    let mut rng = rand::thread_rng();

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Circle Packing", size.0, size.1)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        canvas.set_draw_color(Color::RGB(0,0,0));
        canvas.clear();
        scene.populate(circles_per_frame, &mut rng);
        scene.update();

        canvas.set_draw_color(Color::RGB(255, 210, 0));
        for c in &scene.circles {
            canvas.circle(c.x as i16, c.y as i16, c.r as i16, c.c);
        } for c in &scene.dynamic {
            canvas.circle(c.x as i16, c.y as i16, c.r as i16, c.c);
        }
        
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit{..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}