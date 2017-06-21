use conrod::backend::glium::glium;
use conrod::backend::glium::glium::{DisplayBuild, Surface};
use conrod::{widget, Colorable, Labelable, Positionable, Sizeable, Widget};
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

macro_rules! trigger {
    ($ui:expr,$id:expr, $y:expr, $name:expr, $action:block) => {
                for _click in widget::Button::new()
                    .x_y(-(WIDTH as f64) * 0.5 + 40.0, (HEIGHT as f64) * 0.5 - 20.0-24.0*($y as f64))
                    .w_h(80.0, 20.0)
                    .color(conrod::color::WHITE)
                    .label($name)
                    .label_font_size(14)
                    .label_color(conrod::color::BLACK)
                    //.horizontal_align(HorizontalAlign::Left)
                    .set($id, $ui)
                    $action
    }
}

enum UIState {
    Main,
    Load,
}

use std;
use std::path::{Path, PathBuf};
use std::fs;

struct DirEnt {
    name: String,
    is_dir: bool,
    is_plx: bool,
}

fn getdir(dir: &Path) -> std::io::Result<Vec<DirEnt>> {
    let mut files: Vec<DirEnt> = vec![DirEnt {
                                          name: "..".to_string(),
                                          is_dir: true,
                                          is_plx: false,
                                      }];
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let path = entry?.path();

            let name = if let Some(x) = path.file_name() {
                if let Some(x) = x.to_str() {
                    x
                } else {
                    "non unicode"
                }
            } else {
                "foo"
            };
            let is_dir = path.is_dir();
            let is_plx = if let Some(x) = path.extension() {
                x == "plx"
            } else {
                false
            };
            if is_dir || is_plx {
                files.push(DirEnt {
                    name: name.to_string(),
                    is_dir: is_dir,
                    is_plx: is_plx,
                });
            }
        }

    }
    Ok(files)
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



    let font_paths = ["NotoSans-Regular.ttf".to_string(),
                      concat!(env!("CARGO_MANIFEST_DIR"),
                              "/assets/fonts/NotoSans/NotoSans-Regular.ttf")
                          .to_string()];
    let mut font_loaded = false;
    for p in font_paths.iter() {
        if let Ok(x) = ui.fonts.insert_from_file(p) {
            println!("font loaded from {}:{:?}", p, x);
            font_loaded = true;
            break;
        }
    }
    if !font_loaded {
        println!("Failed to load font");
        return;
    }



    let mut p0 = Pixmap::new(320, 200);
    anim_test_tx(&mut p0, 0.0);
    let renderframe = RenderFrame::new_for_pixmap(editor.view(), &display, &mut image_map);


    let mut curdir = PathBuf::from(".");
    let mut dirlist = getdir(&curdir).unwrap();






    widget_ids!(struct Ids { text, texture_test, id_load, id_save, id_export, id_quit, id_ok, id_cancel, id_list });
    let ids = Ids::new(ui.widget_id_generator());

    let mut pacer = Pacer::from_millis(16);
    let mut ui_state = UIState::Main;
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


        match ui_state {
            UIState::Load => {
                let ui = &mut ui.set_widgets();
                trigger!(ui,ids.id_ok, 0, "load", {ui_state = UIState::Main; });
                trigger!(ui,ids.id_cancel, 1, "cancel", {ui_state = UIState::Main; });



                // Instantiate the `ListSelect` widget.
                let num_items = dirlist.len();
                let item_h = 30.0;
                let font_size = item_h as conrod::FontSize / 2;
                let (mut events, scrollbar) = widget::ListSelect::single(num_items, 40.0)
                    .scrollbar_next_to()
                    .middle()
                    .w_h(400.0, 230.0)
                    .set(ids.id_list, ui);

                let mut updatelist = false;
                // Handle the `ListSelect`s events.
                while let Some(event) = events.next(ui, |i| i < 2) {
                    use conrod::widget::list_select::Event;
                    match event {
                        // For the `Item` events we instantiate the `List`'s items.
                        Event::Item(item) => {
                            let label = &dirlist[item.i].name;
                            let (color, label_color) = match dirlist[item.i].is_dir {
                                true => (conrod::color::LIGHT_BLUE, conrod::color::YELLOW),
                                false => (conrod::color::LIGHT_GREY, conrod::color::BLACK),
                            };
                            let button = widget::Button::new()
                                .color(color)
                                .label(label)
                                .label_font_size(font_size)
                                .label_color(label_color);
                            item.set(button, ui);
                        }

                        // The selection has changed.
                        Event::Selection(j) => {
                            if dirlist[j].is_plx {
                                let loadpath = curdir.with_file_name(&dirlist[j].name);
                                println!("time to load {:?}",loadpath);
                            } else if dirlist[j].is_dir {
                                curdir.push(&dirlist[j].name);
                                curdir = curdir.canonicalize().unwrap();
                                updatelist = true;
                            }
                        }

                        _ => (),
                    }
                }

                // Instantiate the scrollbar for the list.
                if let Some(s) = scrollbar {
                    s.set(ui);
                }
                if updatelist {
                    dirlist = getdir(&curdir).unwrap()
                }

            }
            UIState::Main => {
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

                trigger!(ui,ids.id_load,   0, "load", {ui_state = UIState::Load; });
                trigger!(ui,ids.id_save,   1, "save", {});
                trigger!(ui,ids.id_export, 2, "export", {println!("boo")});
                trigger!(ui,ids.id_quit, 3, "quit", {break 'main});
            }
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
