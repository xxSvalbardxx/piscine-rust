use std::fs::File;
use std::io::Write;

pub fn open_or_create(file: &str, content: &str) {
    let mut f = File::create(file).expect("Unable to create file");
    f.write_all(content.as_bytes()).expect("Unable to write data");
}
        