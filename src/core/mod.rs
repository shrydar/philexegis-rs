
extern crate serde;
extern crate serde_json;
use std;

#[derive(Serialize, Deserialize, Debug)]
struct P2d {
    x: i32,
    y: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct PlxFile( String, VersionedLayerList );

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
        #[serde(deserialize_with = "deserialize_png_data")]
        imagedata: Pixmap,
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
    },
}
#[derive(Debug,Serialize)]
struct Pixmap {
    width: u32,
    height: u32,
    data: Vec<u8>,
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

    if "data:image/png;base54," != &s[0..22] {
        return Err(serde::de::Error::custom("header mismatch!"));
    }

    let data = base64::decode(&s[22..]).map_err(|_| serde::de::Error::custom("base64 failure"))?;

    let png_decoder = png::Decoder::new(Cursor::new(data));

    let (info, mut reader) = png_decoder.read_info().unwrap();
    let mut buf = vec![0; info.buffer_size()];
    reader.next_frame(&mut buf).unwrap();

    println!("decoded {} x {} png ({} bytes) ",
             info.width,
             info.height,
             buf.len());

    Ok(Pixmap {
        width: info.width,
        height: info.height,
        data: buf,
    })
}

const EG: &str = r#"[
  "philexegis",
  {
    "formatversion": 1,
    "layers": [
      {
        "layertype": "ImageLayer",
        "name": "background",
        "uuid": "9b744f82-4c0d-4951-8a2d-00c01d0a4701",
        "visible": true,
        "delta": {
          "x": 0,
          "y": 0
        },
        "pixel_scale": {
          "x": 2,
          "y": 1
        },
        "delta_snap": {
          "x": 2,
          "y": 1
        },
        "imagedata": "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAKAAAADICAYAAABvaOoaAAADeUlEQVR4Xu3SMQ0AAAzDsJU/6cHI4xKoFHlnCoQFFn67VuAAhCAtAGCa3zmADKQFAEzzOweQgbQAgGl+5wAykBYAMM3vHEAG0gIApvmdA8hAWgDANL9zABlICwCY5ncOIANpAQDT/M4BZCAtAGCa3zmADKQFAEzzOweQgbQAgGl+5wAykBYAMM3vHEAG0gIApvmdA8hAWgDANL9zABlICwCY5ncOIANpAQDT/M4BZCAtAGCa3zmADKQFAEzzOweQgbQAgGl+5wAykBYAMM3vHEAG0gIApvmdA8hAWgDANL9zABlICwCY5ncOIANpAQDT/M4BZCAtAGCa3zmADKQFAEzzOweQgbQAgGl+5wAykBYAMM3vHEAG0gIApvmdA8hAWgDANL9zABlICwCY5ncOIANpAQDT/M4BZCAtAGCa3zmADKQFAEzzOweQgbQAgGl+5wAykBYAMM3vHEAG0gIApvmdA8hAWgDANL9zABlICwCY5ncOIANpAQDT/M4BZCAtAGCa3zmADKQFAEzzOweQgbQAgGl+5wAykBYAMM3vHEAG0gIApvmdA8hAWgDANL9zABlICwCY5ncOIANpAQDT/M4BZCAtAGCa3zmADKQFAEzzOweQgbQAgGl+5wAykBYAMM3vHEAG0gIApvmdA8hAWgDANL9zABlICwCY5ncOIANpAQDT/M4BZCAtAGCa3zmADKQFAEzzOweQgbQAgGl+5wAykBYAMM3vHEAG0gIApvmdA8hAWgDANL9zABlICwCY5ncOIANpAQDT/M4BZCAtAGCa3zmADKQFAEzzOweQgbQAgGl+5wAykBYAMM3vHEAG0gIApvmdA8hAWgDANL9zABlICwCY5ncOIANpAQDT/M4BZCAtAGCa3zmADKQFAEzzOweQgbQAgGl+5wAykBYAMM3vHEAG0gIApvmdA8hAWgDANL9zABlICwCY5ncOIANpAQDT/M4BZCAtAGCa3zmADKQFAEzzOweQgbQAgGl+5wAykBYAMM3vHEAG0gIApvmdA8hAWgDANL9zABlICwCY5ncOIANpAQDT/M4BZCAtAGCa3zmADKQFAEzzOweQgbQAgGl+5wAykBYAMM3vHEAG0gIApvmdA8hAWgDANL9zABlICwCY5ncOIANpAQDT/M4BZCAtAGCa3zmADKQFAEzzOweQgbQAgGl+5w+Y5gDJeUDbHQAAAABJRU5ErkJggg=="
      },
{
        "layertype": "ModeFilterHi5OnKoala",
        "name": "Hi5OnKoala (mode filter)",
        "uuid": "7966ba30-c1df-4ea0-850c-52508989e43d",
        "visible": true,
        "d021": 0,
        "fivePal": [
          11,
          0,
          12,
          15,
          1
        ],
        "detailColour": 12
      }
    ]
  }
]"#;

pub fn test_deserialize() {
    println!("Hello, world!");
    let p: PlxFile = serde_json::from_str(EG).unwrap();
    println!("p = {:?}", p);

    let file = std::fs::File::open("Deadlock repixel.plx").unwrap();
    let _q: PlxFile = serde_json::from_reader(file).unwrap();
    // println!("\n\n\nq = {:?}", q);
}
