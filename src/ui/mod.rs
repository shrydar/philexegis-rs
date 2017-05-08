use conrod::backend::glium::glium;
use conrod::backend::glium::glium::{DisplayBuild, Surface};
use conrod::{widget, Colorable, Positionable, Widget};
use std::{time, thread};

extern crate conrod;


const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;


use core::Pixmap;

fn px_to_tx(p: &Pixmap, display: &glium::Display) -> glium::texture::Texture2d {
    let raw_image = glium::texture::RawImage2d::from_raw_rgba_reversed(p.data.clone(),
                                                                       (p.width, p.height));
    let texture = glium::texture::Texture2d::new(display, raw_image).unwrap();
    texture
}


pub fn run() {
    let mut p0 = Pixmap::new(16, 16);
    let mut p1 = Vec::<(u8, u8, u8, u8)>::new();
    for x in 0..16 {
        for y in 0..16 {
            let di = 4 * (y * 16 + x);
            p0.data[di + 0] = x as u8 * 17;
            p0.data[di + 1] = y as u8 * 17;
            p0.data[di + 2] = 117;
            p0.data[di + 3] = 255;

            p1.push((x as u8 * 17, y as u8 * 17, 117u8, 255u8));
        }
    }
    let display = glium::glutin::WindowBuilder::new()
        .with_vsync()
        .with_dimensions(WIDTH, HEIGHT)
        .with_title("Philexegis")
        .build_glium()
        .unwrap();

    let mut ui = conrod::UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();

    const FONT_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"),
                                    "/assets/fonts/NotoSans/NotoSans-Regular.ttf");
    ui.fonts.insert_from_file(FONT_PATH).unwrap();


    let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();
    let mut image_map = conrod::image::Map::<glium::texture::Texture2d>::new();

    let pixbuf = glium::texture::pixel_buffer::PixelBuffer::<(u8, u8, u8, u8)>::new_empty(&display,
                                                                                          16 * 16);

    let tx0 = px_to_tx(&p0, &display);
    let tx_id0 = image_map.insert(tx0);

    widget_ids!(struct Ids { text, texture_test });
    let ids = Ids::new(ui.widget_id_generator());

    let start = time::Instant::now();
    let mut last_update = time::Instant::now();
    'main: loop {
        let sixteen_ms = time::Duration::from_millis(16);
        let duration_since_last_update = time::Instant::now().duration_since(last_update);
        let duration_since_start = time::Instant::now().duration_since(start);
        if duration_since_last_update < sixteen_ms {
            thread::sleep(sixteen_ms - duration_since_last_update);
        }
        let t = (duration_since_start.as_secs() as f64) +
                (duration_since_start.subsec_nanos() as f64) * 1e-9;
        last_update = time::Instant::now();

        {
            let tx = &image_map[&tx_id0];
            let ph: usize = ((t * 172.0) %272.0) as usize;
            for x in 0..16 {
                let mut q = ((x*16)+ph)%272;
                if q>255 {
                    q=(272-q)*15
                };

                for y in 0..16 {
                    let di = y * 16 + x;
                    p1[di] = (q as u8, y as u8 * 17, 117u8, 255u8);
                }
            }

            pixbuf.write(&p1[..]);
            tx.mipmap(0)
                .unwrap()
                .raw_upload_from_pixel_buffer(pixbuf.as_slice(), 0..p0.width, 0..p0.height, 0..1);
        }

        let events: Vec<_> = display.poll_events().collect();

        for event in events {


            // Use the `winit` backend feature to convert the winit event to a conrod one.
            if let Some(event) = conrod::backend::winit::convert(event.clone(), &display) {
                ui.handle_event(event);
            }

            match event {
                glium::glutin::Event::KeyboardInput(_, _, Some(glium::glutin::VirtualKeyCode::Escape)) |
                    glium::glutin::Event::Closed =>
                        break 'main,
                _ => {}
            }
        }


        {

            use conrod::Sizeable;
            let ui = &mut ui.set_widgets();

            // "Hello World!" in the middle of the screen.
            widget::Text::new("Wombats!")
                .x_y(70.0 * t.sin(), 60.0 * t.cos())
                .color(conrod::color::WHITE)
                .font_size(32)
                .set(ids.text, ui);
            widget::Image::new(tx_id0).w_h(32.0, 32.0).middle().set(ids.texture_test, ui);
        }


        // Render the `Ui` and then display it on the screen.
        if let Some(primitives) = ui.draw_if_changed() {
            renderer.fill(&display, primitives, &image_map);
            let mut target = display.draw();
            target.clear_color(0.2, 0.0, 0.1, 1.0);
            renderer.draw(&display, &mut target, &image_map).unwrap();
            target.finish().unwrap();
        }




    }



}
