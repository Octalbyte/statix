use std::io::BufReader;
use std::fs::File;
use std::io::Read;
use std::convert::TryInto;
pub fn isBin(filename: String) -> bool{
    let f = File::open(filename.as_str()).unwrap();
    let len = f.metadata().unwrap().len();
    let mut reader = BufReader::new(f);
    let bytes_to_read = 128;
    let mut bytes_to_read = bytes_to_read;
    if len < bytes_to_read {
        bytes_to_read = len;
    }
    let mut buf = vec![0u8; bytes_to_read.try_into().unwrap()];
    reader.read_exact(&mut buf).unwrap();
    let mut is_bin = false;
    for i in buf {
        if i > 127 {
            is_bin = true;
        }
    }
    return is_bin;
}