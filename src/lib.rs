pub mod crt {

    pub fn public(_filename: String) -> Vec<u8> {
        use std::io;
        use std::io::Read;
        use std::io::BufReader;
        use std::fs::File;

        let _filename = _filename.as_str();
        let f = File::open("./".to_owned()+_filename).expect("Error opening public certificate file");
        let mut reader = BufReader::new(f);
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer).expect("Failed to tread to buffer");
        return buffer;
    }
    pub fn private(_filename: String) -> Vec<u8> {
        use std::io;
        use std::io::Read;
        use std::io::BufReader;
        use std::fs::File;

        let _filename = _filename.as_str();
        let f = File::open("./".to_owned()+_filename).expect("Error opening private key file");
        let mut reader = BufReader::new(f);
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer).expect("Failed to tread to buffer");
        return buffer;
    }
}
