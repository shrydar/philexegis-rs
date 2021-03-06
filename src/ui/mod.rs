use conrod::backend::glium::glium;
use conrod::backend::glium::glium::{DisplayBuild, Surface};
use conrod::{widget, Colorable, Positionable, Widget, Sizeable};
use std::{time, thread, mem, slice};
use core::{Pixmap, Editor};

extern crate conrod;


type PixelBuffer = glium::texture::pixel_buffer::PixelBuffer<(u8, u8, u8, u8)>;
type ImageMap = conrod::image::Map<glium::texture::Texture2d>;


struct RenderFrame {
    pixbuf: PixelBuffer,
    width: u32,
    height: u32,
    tx_id: conrod::image::Id,
}
impl RenderFrame {
    fn new_for_pixmap(p: &Pixmap, display: &glium::Display, image_map: &mut ImageMap) -> RenderFrame {
        let pixbuf = PixelBuffer::new_empty(display, (p.width * p.height) as usize);
        let tx0 = px_to_tx(p, display);
        let tx_id = image_map.insert(tx0);
        RenderFrame {
            pixbuf: pixbuf,
            width: p.width,
            height: p.height,
            tx_id: tx_id,
        }
    }
    fn update_from(&self, p: &Pixmap, image_map: &ImageMap) -> () {
        debug_assert_eq!([1u8, 2, 3, 4],
                         unsafe { mem::transmute::<(u8, u8, u8, u8), [u8; 4]>((1u8, 2u8, 3u8, 4u8)) });
        debug_assert_eq!(self.width, p.width);
        debug_assert_eq!(self.height, p.height);
        if self.width == p.width && self.height == p.height {

            unsafe {
                self.pixbuf.write(slice::from_raw_parts(p.data.as_ptr() as *const (u8, u8, u8, u8), p.data.len() / 4));
            }
            image_map[&self.tx_id]
                .mipmap(0)
                .unwrap()
                .raw_upload_from_pixel_buffer(self.pixbuf.as_slice(), 0..p.width, 0..p.height, 0..1);
        }
    }
}

struct Pacer {
    start: time::Instant,
    last_update: time::Instant,
    increment: time::Duration,
}
impl Pacer {
    fn from_millis(millis: u64) -> Pacer {
        let now = time::Instant::now();
        Pacer {
            start: now,
            last_update: now,
            increment: time::Duration::from_millis(millis),
        }
    }
    fn tick(&mut self) -> f64 {
        let now = time::Instant::now();
        let duration_since_last_update = now.duration_since(self.last_update);
        if duration_since_last_update < self.increment {
            thread::sleep(self.increment - duration_since_last_update);
        }

        let now = time::Instant::now();
        self.last_update = now;
        let duration_since_start = now.duration_since(self.start);
        (duration_since_start.as_secs() as f64) + (duration_since_start.subsec_nanos() as f64) * 1e-9
    }
}


const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;



fn px_to_tx(p: &Pixmap, display: &glium::Display) -> glium::texture::Texture2d {
    let raw_image = glium::texture::RawImage2d::from_raw_rgba_reversed(p.data.clone(), (p.width, p.height));
    glium::texture::Texture2d::new(display, raw_image).unwrap()
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
    let display = glium::glutin::WindowBuilder::new()
        .with_vsync()
        .with_dimensions(WIDTH, HEIGHT)
        .with_title("Philexegis")
        .build_glium()
        .unwrap();

    let mut ui = conrod::UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();
    let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();
    let mut image_map = conrod::image::Map::<glium::texture::Texture2d>::new();
    let mut editor = Editor::new();



    const FONT_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"),
                                    "/assets/fonts/NotoSans/NotoSans-Regular.ttf");
    ui.fonts.insert_from_file(FONT_PATH).unwrap();



    let mut p0 = Pixmap::new(320, 200);
    anim_test_tx(&mut p0, 0.0);
    let renderframe = RenderFrame::new_for_pixmap(editor.view(), &display, &mut image_map);




    widget_ids!(struct Ids { text, texture_test });
    let ids = Ids::new(ui.widget_id_generator());

    let mut pacer = Pacer::from_millis(16);
    'main: loop {
        let t = pacer.tick();


        let events: Vec<_> = display.poll_events().collect();
        for event in events {
            if let Some(event) = conrod::backend::winit::convert(event.clone(), &display) {
                ui.handle_event(event);
            }

            match event {
                glium::glutin::Event::KeyboardInput(_, _, Some(glium::glutin::VirtualKeyCode::Escape)) |
                glium::glutin::Event::Closed => break 'main,
                _ => {}
            }
        }

        anim_test_tx(&mut p0, t);
        // renderframe.update_from(&p0, &image_map);
        renderframe.update_from(editor.view(), &image_map);


        {
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
            target.clear_color(0.1, 0.0, 0.07, 1.0);
            renderer.draw(&display, &mut target, &image_map).unwrap();
            target.finish().unwrap();
        }




    }



}
