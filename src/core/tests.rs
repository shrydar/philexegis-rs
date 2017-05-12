
const SAMPLE: &str = concat!(r#"[
  "philexegis",
  {
    "formatversion": 1,
    "layers": [
      {
        "layertype": "ImageLayer",
        "name": "peptoTest.png",
        "uuid": "997d9c6b-8d2c-443a-b511-2a9d9ef8ee5c",
        "visible": false,
        "delta": {
          "x": 0,
          "y": 0
        },
        "pixel_scale": {
          "x": 1,
          "y": 1
        },
        "delta_snap": {
          "x": 1,
          "y": 1
        },
        "imagedata": "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAACAAAACQCAYAAABpsGmBAAAA6ElE"#,
                             r#"QVR4Xu3WsQ1BYRiF4f82OgWFQiKiFp1aaFnj7qAQiR1upTCGTogNbKBSssJviK/4"#,
                             r#"mudMcPJUb1NrrSVxjQMECBAgQIBAusDjtQ31wH20CtVE4wABAgQIECCQLtB1XagH"#,
                             r#"brNhrAccIECAAAECBNIF2rYN9UD97mM94AABAgQIECCQLrA8b0I9cLyOYz3gAAEC"#,
                             r#"BAgQIJAucFz3Qz3Q+5xiPeAAAQIECBAgkC5wmM5DPfCcDGI94AABAgQIECCQLrBb"#,
                             r#"lFAPvH+XWA84QIAAAQIECKQLlBLrgVAMlFIaBwgQIECAAIFsgT9VV+RQNWmB5QAA"#,
                             r#"AABJRU5ErkJggg=="
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
]"#);

pub fn test_deserialize() {
    use ::core::{save_to_writer, load_from_reader, Layer};
    extern crate serde_json;
    use std;
    use std::io::Cursor;

    println!("Hello, world!");
    match load_from_reader(Cursor::new(&SAMPLE)) {
        Ok(p) => {
            println!("Ok! p.len() = {:?}", p.len());
        }
        Err(p) => println!("Err:  {:?}", p),
    }

    let file = std::fs::File::open("Deadlock repixel.plx").unwrap();
    let q: Vec<Box<Layer>> = load_from_reader(file).unwrap();
    for j in &q {
        println!(": {:20}", j.get_name());
    }

    let fo = std::fs::File::create("Deadlock export.plx").unwrap();
    save_to_writer(fo, &q);
}
