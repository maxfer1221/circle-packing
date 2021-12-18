use sdl2::{gfx::primitives::DrawRenderer, pixels::Color, rect::Rect, event::Event, keyboard::Keycode};
use std::time::Duration;
use rand::{thread_rng, Rng};

mod circles;
use circles::Circle;
mod scene;
use scene::Scene;

fn main() {
    let size = (800, 800);
    let c_count = 10;

    let mut rng = rand::thread_rng();
    
    let mut circles = Vec::<Circle>::with_capacity(c_count);
    for x in 0..c_count {
        circles.push(Circle::new(rng.gen_range(20.0..780.0), rng.gen_range(20.0..780.0), rng.gen_range(5.0..20.0)));
    }
    let mut scene = Scene::new(size, 0.5, circles);

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

        canvas.set_draw_color(Color::RGB(255, 210, 0));
        for c in &scene.circles {
            canvas.filled_circle(c.x as i16, c.y as i16, c.r as i16, Color::RGB(255, 0, 0));
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