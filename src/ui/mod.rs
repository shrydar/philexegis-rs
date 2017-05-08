use conrod::backend::glium::glium;
use conrod::backend::glium::glium::{DisplayBuild, Surface};
use conrod::{widget, Colorable, Positionable, Widget};
use std::{time,thread};

extern crate conrod;


const WIDTH:u32 = 800;
const HEIGHT:u32 = 600;


use core::Pixmap;



pub fn run() {
    let _p= Pixmap::new(16,16);
    let display = glium::glutin::WindowBuilder::new()
        .with_vsync()
        .with_dimensions(WIDTH, HEIGHT)
        .with_title("Philexegis")
        .build_glium()
        .unwrap();

    let mut ui = conrod::UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();
    
    const FONT_PATH: &str =
        concat!(env!("CARGO_MANIFEST_DIR"), "/assets/fonts/NotoSans/NotoSans-Regular.ttf");
    ui.fonts.insert_from_file(FONT_PATH).unwrap();


    let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();
    let image_map = conrod::image::Map::<glium::texture::Texture2d>::new();
    


    widget_ids!(struct Ids { text });
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

            //use conrod::{Labelable, Sizeable};
            let ui = &mut ui.set_widgets();

            // "Hello World!" in the middle of the screen.
            widget::Text::new("Wombats!")
                .x_y(70.0 * t.sin(), 60.0 * t.cos())
                .color(conrod::color::WHITE)
                .font_size(32)
                .set(ids.text, ui);
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

