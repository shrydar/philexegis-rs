use conrod::backend::glium::glium;
use conrod::backend::glium::glium::{DisplayBuild, Surface};
use conrod::{widget, Colorable, Positionable, Widget};
use std::{time, thread, mem, slice};
use core::Pixmap;

extern crate conrod;


type PixelBuffer = glium::texture::pixel_buffer::PixelBuffer<(u8, u8, u8, u8)>;
type ImageMap = conrod::image::Map<glium::texture::Texture2d>;


struct RenderFrame {
    pixbuf: PixelBuffer,
    tx_id: conrod::image::Id,
}
impl RenderFrame {
    fn new_for_pixmap(p: &Pixmap,
                      display: &glium::Display,
                      image_map: &mut ImageMap)
                      -> RenderFrame {
        let pixbuf = PixelBuffer::new_empty(display, (p.width * p.height) as usize);
        let tx0 = px_to_tx(p, &display);
        let tx_id = image_map.insert(tx0);
        RenderFrame {
            pixbuf: pixbuf,
            tx_id: tx_id,
        }
    }
    fn update_from(&self, p: &Pixmap, image_map: &ImageMap) -> () {
        debug_assert_eq!([1u8, 2, 3, 4], unsafe {
            mem::transmute::<(u8, u8, u8, u8), [u8; 4]>((1u8, 2u8, 3u8, 4u8))
        });

        unsafe {
            self.pixbuf.write(slice::from_raw_parts(p.data.as_ptr() as *const (u8, u8, u8, u8),
                                                    p.data.len() / 4));
        }
        image_map[&self.tx_id]
            .mipmap(0)
            .unwrap()
            .raw_upload_from_pixel_buffer(self.pixbuf.as_slice(), 0..p.width, 0..p.height, 0..1);
    }
}


const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;



fn px_to_tx(p: &Pixmap, display: &glium::Display) -> glium::texture::Texture2d {
    let raw_image = glium::texture::RawImage2d::from_raw_rgba_reversed(p.data.clone(),
                                                                       (p.width, p.height));
    let texture = glium::texture::Texture2d::new(display, raw_image).unwrap();
    texture
}

fn anim_test_tx(p0: &mut Pixmap, t: f64) {

    let ph = ((272.0 + (t * 3.0).sin() * 172.0) % 272.0) as u32;
    let oy = ((272.0 + (t * 3.0).cos() * 172.0) % 272.0) as u32;
    let sf = (700.0 + 500.0 * (t * 2.0).sin()) as u32;
    for x in 0..p0.width {
        let mut q = ((x * sf / 100) + ph) % 272;
        if q > 255 {
            q = (272 - q) * 15
        };

        for y in 0..p0.height {
            let mut v = ((y * sf / 100) + oy) % 272;
            if v > 255 {
                v = (272 - v) * 15
            };
            let di = (4 * (y * p0.width + x)) as usize;
            p0.data[di + 0] = q as u8;
            p0.data[di + 1] = v as u8;
            p0.data[di + 2] = 117;
            p0.data[di + 3] = 255;
        }
    }
}

pub fn run() {


    let mut p0 = Pixmap::new(320, 200);
    anim_test_tx(&mut p0, 0.0);

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


    let renderframe = RenderFrame::new_for_pixmap(&p0, &display, &mut image_map);


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



        anim_test_tx(&mut p0, t);

        renderframe.update_from(&p0, &image_map);


        {

            use conrod::Sizeable;
            let ui = &mut ui.set_widgets();

            widget::Image::new(renderframe.tx_id)
                .w_h((p0.width * 2) as f64, (p0.height * 2) as f64)
                .middle()
                .set(ids.texture_test, ui);

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
