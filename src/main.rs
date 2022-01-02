use mnist::readfile;
use ndarray::IxDyn;
use std::env::args;
use std::fs::File;
use std::io::BufWriter;
use std::io::Error;
use std::path::Path;
// To use encoder.set()

use ndarray::{Array, Axis};

fn main() -> Result<(), Error> {
    let mut arguments = args().skip(1);
    let filename = match arguments.next() {
        Some(s) => s,
        None => panic!("usage: mnist file"),
    };

    let a: Array<u8, IxDyn> = readfile(&filename)?;
    let image = a.index_axis(Axis(0), 0);

    let path = Path::new(r"./image.png");
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, 28, 28); // Width is 2 pixels and height is 1.
    encoder.set_color(png::ColorType::Grayscale);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(image.as_slice().unwrap()).unwrap();

    Ok(())
}
