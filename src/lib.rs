use byteorder::BigEndian;
use byteorder::ByteOrder;
use ndarray::Array;
use ndarray::Dim;
use ndarray::IxDyn;
use std::io::BufReader;
use std::io::{Error, ErrorKind};

#[derive(Debug)]
pub enum MnistType {
    UnsignedByte = 0x8,
    SignedByte = 0x9,
    Short = 0xB,
    Integer = 0xC,
    Float = 0xD,
    Double = 0xE,
}

impl From<u8> for MnistType {
    fn from(int: u8) -> MnistType {
        match int {
            0x8 => MnistType::UnsignedByte,
            0x9 => MnistType::SignedByte,
            0xB => MnistType::Short,
            0xC => MnistType::Integer,
            0xD => MnistType::Float,
            0xE => MnistType::Double,
            _ => unreachable!(),
        }
    }
}

// #[derive(Debug)]
// enum MnistOutput {
//     UnsignedByte(Vec<u8>),
//     SignedByte(Vec<i8>),
//     Short(Vec<i16>),
//     Integer(Vec<i32>),
//     Float(Vec<f32>),
//     Double(Vec<f64>),
// }

pub fn readfile(filename: &String) -> Result<Array<u8, IxDyn>, Error> {
    //use std::fs::metadata;
    use std::fs::File;
    use std::io::prelude::*;

    let file = File::open(filename)?;
    //let metadata = metadata(&filename).expect("Unable to read metadata");
    let mut reader = BufReader::new(file);
    let mut header = [0; 4];

    reader.read(&mut header[..])?;
    // TODO check magic
    let mut dbuf: Vec<u8> = vec![0; header[3] as usize * 4];
    reader.read(&mut dbuf[..])?;
    let d: Vec<usize> = dbuf
        .chunks(4)
        .map(|c| BigEndian::read_u32(c) as usize)
        .collect();

    let shape = Dim(d);
    let mut buffer = Vec::new();
    // read the whole file
    reader.read_to_end(&mut buffer)?;
    let rc = Array::from_shape_vec(shape, buffer);
    match rc {
        Ok(a) => Ok(a),
        Err(_e) => Err(Error::new(ErrorKind::Other, "oh no!")),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test2() {
        assert_ne!(3, 4);
    }
}
