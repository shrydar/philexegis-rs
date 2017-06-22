
extern crate serde;
extern crate serde_json;
use std;
use std::path::PathBuf;
pub mod tests;

use self::serde_json::error::Error as SerdeError;

#[derive(Serialize, Deserialize, Debug)]
struct P2d {
    x: i32,
    y: i32,
}

#[derive(Deserialize)]
struct PlxFile(String, VersionedLayerList);
#[derive(Serialize)]
struct PlxFileRef<'a>(String, VersionedLayerListRef<'a>);

#[derive(Deserialize)]
struct VersionedLayerList {
    formatversion: u32,
    layers: Vec<TaggedLayer>,
}
#[derive(Serialize)]
struct VersionedLayerListRef<'a> {
    formatversion: u32,
    layers: Vec<TaggedLayerRef<'a>>,
}

#[derive(Deserialize)]
#[serde(tag = "layertype")]
enum TaggedLayer {
    ImageLayer(ImageLayer),
    ModeFilterHi5OnKoala(ModeFilterHi5OnKoala),
}
#[derive(Serialize)]
#[serde(tag = "layertype")]
enum TaggedLayerRef<'a> {
    ImageLayer(&'a ImageLayer),
    ModeFilterHi5OnKoala(&'a ModeFilterHi5OnKoala),
}



trait Layer {
    fn tag(&self) -> TaggedLayerRef;
    fn get_name(&self) -> String;
    fn composite_over(&self, p: &mut Pixmap);
}
impl Layer for ImageLayer {
    fn tag(&self) -> TaggedLayerRef {
        TaggedLayerRef::ImageLayer(self)
    }
    fn get_name(&self) -> String {
        self.name.clone()
    }
    fn composite_over(&self, p: &mut Pixmap) {

        let q = &self.imagedata;
        if !self.visible {
            return;
        }
        for x in 0..p.width {
            for y in 0..p.height {
                let pi = ((p.width * y + x) * 4) as usize;
                let qi = ((q.width * y + x) * 4) as usize;
                if (x < q.width) && (y < q.height) && q.data[qi + 3] == 255 {
                    p.data[pi + 0] = q.data[qi + 0];
                    p.data[pi + 1] = q.data[qi + 1];
                    p.data[pi + 2] = q.data[qi + 2];
                    p.data[pi + 3] = 255;
                }
            }
        }
    }
}
impl Layer for ModeFilterHi5OnKoala {
    fn tag(&self) -> TaggedLayerRef {
        TaggedLayerRef::ModeFilterHi5OnKoala(self)
    }
    fn get_name(&self) -> String {
        self.name.clone()
    }
    fn composite_over(&self, _p: &mut Pixmap) {}
}
fn un_enum(x: TaggedLayer) -> Box<Layer> {
    match x {
        TaggedLayer::ImageLayer(v) => Box::new(v),
        TaggedLayer::ModeFilterHi5OnKoala(v) => Box::new(v),
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct ImageLayer {
    name: String,
    uuid: String,
    visible: bool,
    delta: P2d,
    pixel_scale: P2d,
    delta_snap: P2d,
    #[serde(deserialize_with = "deserialize_png_data", serialize_with = "serialize_png_data")]
    imagedata: Pixmap,
    #[serde(skip_serializing, skip_deserializing)]
    _preview: Option<Pixmap>,
}
#[derive(Serialize, Deserialize, Debug)]
struct ModeFilterHi5OnKoala {
    name: String,
    uuid: String,
    visible: bool,
    d021: u8,
    #[serde(rename="fivePal")]
    five_pal: [u8; 5],
    #[serde(rename="detailColour")]
    detail_colour: u8, /* #[serde(skip_serializing, skip_deserializing)]
                        * _preview: Option<Pixmap>, */
}


#[derive(Serialize)]
pub struct Pixmap {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
}
impl Pixmap {
    pub fn new(w: u32, h: u32) -> Pixmap {
        Pixmap {
            width: w,
            height: h,
            data: vec![0;(w*h*4) as usize],
        }
    }
}

impl std::fmt::Debug for Pixmap {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,
               "Pixmap {{ width: {}, height: {}, data: {{...}}  }}",
               self.width,
               self.height)
    }
}

fn serialize_png_data<S>(p: &Pixmap, se: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer
{
    use base64;
    let mut buf: Vec<u8> = Vec::new();
    {
        use png;
        use png::HasParameters;
        let mut png_encoder = png::Encoder::new(&mut buf, p.width, p.height);
        png_encoder.set(png::ColorType::RGBA).set(png::BitDepth::Eight);

        let mut writer = png_encoder.write_header().unwrap();

        writer.write_image_data(&p.data).unwrap(); // Save
    }


    let buf64 = format!("data:image/png;base64,{}", base64::encode(&buf));
    se.serialize_str(&buf64)
}

fn deserialize_png_data<'de, D>(de: D) -> Result<Pixmap, D::Error>
    where D: serde::Deserializer<'de>
{
    use base64;
    use png;
    use std::io::Cursor;

    let deser_result = serde::Deserialize::deserialize(de)?;

    let s: &String = match deser_result {
        serde_json::Value::String(ref s) => Ok(s),
        _ => Err(serde::de::Error::custom("string missing for png data")),
    }?;

    if "data:image/png;base64," != &s[0..22] {
        return Err(serde::de::Error::custom("header mismatch!"));
    }

    let data = base64::decode(&s[22..]).map_err(|_| serde::de::Error::custom("base64 failure"))?;

    let png_decoder = png::Decoder::new(Cursor::new(data));

    let (info, mut reader) = png_decoder.read_info()
        .map_err(|_| serde::de::Error::custom("PNG decoding failure"))?;
    let mut buf = vec![0; info.buffer_size()];

    reader.next_frame(&mut buf).map_err(|_| serde::de::Error::custom("PNG decoding failure"))?;

    Ok(Pixmap {
        width: info.width,
        height: info.height,
        data: buf,
    })
}

fn load_from_reader<R>(r: R) -> Result<Vec<Box<Layer>>, SerdeError>
    where R: std::io::Read
{
    let p: PlxFile = serde_json::from_reader(r)?;
    if p.1.formatversion != 1 {
        return Err(serde::de::Error::custom("incorrect format version"));
    }
    let mut a: Vec<TaggedLayer> = p.1.layers;
    let list = a.drain(..).map(un_enum).collect();
    Ok(list)
}

fn save_to_writer<W>(fo: W, v: &[Box<Layer>]) -> ()
    where W: std::io::Write
{

    let q: Vec<TaggedLayerRef> = v.iter().map(|x| x.tag()).collect();
    let p: PlxFileRef = PlxFileRef("philexegis".to_owned(),
                                   VersionedLayerListRef {
                                       formatversion: 1,
                                       layers: q,
                                   });
    serde_json::to_writer_pretty(fo, &p).unwrap()
}



pub struct Editor {
    px_base: Pixmap,
    px_view: Pixmap,
    layers: Vec<Box<Layer>>,
}
const DEFAULT_WIDTH: u32 = 320;
const DEFAULT_HEIGHT: u32 = 200;

impl Editor {
    pub fn new() -> Editor {
        let mut layerinit: Vec<Box<Layer>> = Vec::new();
        if let Ok(file) = std::fs::File::open("Deadlock repixel.plx") {
            if let Ok(layers) = load_from_reader(file) {
                layerinit = layers;
            }
        }
        Editor {
            px_base: Pixmap::new(DEFAULT_WIDTH, DEFAULT_HEIGHT),
            px_view: Pixmap::new(DEFAULT_WIDTH, DEFAULT_HEIGHT),
            layers: layerinit,
        }
    }
    pub fn load(&mut self, path: &PathBuf) {
        let file = std::fs::File::open(path).unwrap();
        match load_from_reader(file) {
            Ok(layers) => self.layers = layers,
            Err(e) => println!("failed to load {:?}: {:?}", path, e),
        }
    }

    pub fn view(&mut self) -> &Pixmap {
        {
            for y in 0..200 {
                for x in 0..320 {
                    let pi = 1280 * y + x * 4;
                    self.px_base.data[pi + 0] = ((x + y) / 4 % 2 * 255) as u8;
                    self.px_base.data[pi + 3] = 255;
                }
            }
            for l in &self.layers {
                l.composite_over(&mut self.px_base);
            }
            let mut px_view = &mut self.px_view;
            let px_base = &self.px_base;
            assert_eq!(px_base.width, px_view.width);
            assert_eq!(px_base.height, px_view.height);
            for dy in 0..px_view.height {
                let sy = px_base.height - 1 - dy;
                let mut si = (sy * px_base.width * 4) as usize;
                let mut di = (dy * px_view.width * 4) as usize;
                for _ in 0..px_view.width {
                    px_view.data[di + 0] = px_base.data[si + 0];
                    px_view.data[di + 1] = px_base.data[si + 1];
                    px_view.data[di + 2] = px_base.data[si + 2];
                    px_view.data[di + 3] = px_base.data[si + 3];

                    si += 4;
                    di += 4;
                }
            }
        }
        &self.px_view
    }
}
