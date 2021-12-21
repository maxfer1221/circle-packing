use image::open;
use sdl2::{event::Event, gfx::primitives::DrawRenderer, keyboard::Keycode, pixels::Color};
use std::time::Duration;

mod circles;
mod scene;
use scene::Scene;
mod features;
mod hashgrid;
mod image_processing;

fn main() {
    let args: &[String] = &std::env::args().collect::<Vec<String>>();
    let img_name: &String = &args[1];
    let (img, size) = match open(img_name) {
        Err(e) => {
            println!("Error opening image: {:?}", e);
            std::process::exit(1);
        }
        Ok(i) => (i.into_rgb8(), image::image_dimensions(img_name).unwrap()),
    };

    // feature extraction
    let cpf: u32 = args[2].parse::<u32>().unwrap();
    let rate: f64 = args[3].parse::<f64>().unwrap();
    let t: i16 = args[4].parse::<i16>().unwrap();
    let th: u8 = args[5].parse::<u8>().unwrap();
    let step: usize = args[6].parse::<usize>().unwrap();
    let (mut feature_pixels, mut index): (Vec<Vec<[usize; 2]>>, isize) =
        image_processing::detect_features_clean(&img, size, t, th, step).unwrap();

    let mut scene = Scene::new(size, cpf, rate, (10.0, 10.0));

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Circle Packing", size.0, size.1)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    // index -= 1;
    // scene.populate_fp(scene.cpf, &img, &mut feature_pixels[index as usize]);

    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        scene.update();
        if index >= 0 && !scene.populate_fp(scene.cpf, &img, &mut feature_pixels[index as usize]) {
            index -= 1;
        }

        // canvas.set_draw_color(Color::RGB(255, 0, 0));
        // for x in 0..scene.hashgrid.cell_count.0 {
        //     canvas.draw_rect(Rect::new(
        //         (x as f64 * scene.hashgrid.cell_dimensions.0) as i32,
        //         0,
        //         1,
        //         scene.size.1 as u32,
        //     ));
        //     for y in 0..scene.hashgrid.cell_count.1 {
        //         canvas.draw_rect(Rect::new(
        //             x as i32,
        //             (y as f64 * scene.hashgrid.cell_dimensions.1) as i32,
        //             scene.size.0 as u32,
        //             1,
        //         ));
        //     }
        // }

        for c in &scene.hashgrid.all_elements {
            canvas
                .filled_circle(c.x as i16, c.y as i16, c.radius as i16, c.color)
                .unwrap();
        }
        // let mut _i = 0;
        // for x in &feature_pixels {
        //     canvas.set_draw_color(Color::RGB(120 * _i, 0, 0));
        //     for y in x {
        //         canvas.fill_rect(sdl2::rect::Rect::new(y[0] as i32, y[1] as i32, 1, 1));
        //     }
        //     _i += 1;
        // }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
