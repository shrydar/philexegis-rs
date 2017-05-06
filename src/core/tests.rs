
const SAMPLE: &str = r#"[
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
        "imagedata": "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAACAAAACQCAYAAABpsGmBAAAA6ElEQVR4Xu3WsQ1BYRiF4f82OgWFQiKiFp1aaFnj7qAQiR1upTCGTogNbKBSssJviK/4mudMcPJUb1NrrSVxjQMECBAgQIBAusDjtQ31wH20CtVE4wABAgQIECCQLtB1XagHbrNhrAccIECAAAECBNIF2rYN9UD97mM94AABAgQIECCQLrA8b0I9cLyOYz3gAAECBAgQIJAucFz3Qz3Q+5xiPeAAAQIECBAgkC5wmM5DPfCcDGI94AABAgQIECCQLrBblFAPvH+XWA84QIAAAQIECKQLlBLrgVAMlFIaBwgQIECAAIFsgT9VV+RQNWmB5QAAAABJRU5ErkJggg=="
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
    use ::core::{load_from_reader, PlxFile};
    use std;
    use std::io::Cursor;

    println!("Hello, world!");
    match load_from_reader(Cursor::new(&SAMPLE)) {
        Ok(p) => println!("Ok! p = {:?}", p),
        Err(p) => println!("Err:  {:?}", p),
    }

    let file = std::fs::File::open("Deadlock repixel.plx").unwrap();
    let _q: PlxFile = load_from_reader(file).unwrap();
}
