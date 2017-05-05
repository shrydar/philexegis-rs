
extern crate serde;
extern crate serde_json;
use std;

#[derive(Serialize, Deserialize, Debug)]
struct P2d {
    x: i32,
    y: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum PlxFile {
   ComponentList(Vec<FileComponent>)
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum FileComponent {
    FormatName (String),
    VersionedLayerList (VersionedLayerList),
}

#[derive(Serialize, Deserialize, Debug)]
struct VersionedLayerList {
    formatversion: u32,
    layers: Vec<Layer>
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
        imagedata: String,
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
    let q: PlxFile = serde_json::from_reader(file).unwrap();
    println!("\n\n\nq = {:?}", q);
}
