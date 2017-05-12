
extern crate serde;
extern crate serde_json;
use std;
pub mod tests;

use self::serde_json::error::Error as SerdeError;

#[derive(Serialize, Deserialize, Debug)]
struct P2d {
    x: i32,
    y: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct PlxFile(String, VersionedLayerList);

#[derive(Serialize, Deserialize, Debug)]
struct VersionedLayerList {
    formatversion: u32,
    layers: Vec<Layer>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "layertype")]
enum Layer {
    ImageLayer {
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
    },
    ModeFilterHi5OnKoala {
        name: String,
        uuid: String,
        visible: bool,
        d021: u8,
        #[serde(rename="fivePal")]
        five_pal: [u8; 5],
        #[serde(rename="detailColour")]
        detail_colour: u8,
        #[serde(skip_serializing, skip_deserializing)]
        _preview: Option<Pixmap>,
    },
}


#[derive(Serialize)]
pub struct Pixmap {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
}
impl Pixmap {
    pub fn new(w:u32, h:u32) -> Pixmap {
        Pixmap {
        width:w,
        height:h,
        data:vec![0;(w*h*4) as usize],
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

fn load_from_reader<R>(r: R) -> Result<PlxFile, SerdeError>
    where R: std::io::Read
{
    serde_json::from_reader(r)
}



pub struct Editor {
    base:Pixmap,
}
const DEFAULT_WIDTH:u32= 320;
const DEFAULT_HEIGHT:u32= 200;

impl Editor {
    pub fn new()->Editor {
        Editor {
            base:Pixmap::new(DEFAULT_WIDTH, DEFAULT_HEIGHT),
        }
    }
    pub fn view<'a>(&mut self)-> &Pixmap {
        for x in 0..320 { for y in 0..200 {
            let pi=1280*y+x*4;
            self.base.data[pi+0]=((x+y)/4%2*255) as u8;
            self.base.data[pi+3]=255;
        }
        }
        return &self.base;
    }
}
