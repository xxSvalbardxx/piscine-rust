use std::fs::File;

pub fn open_file(s: &str) -> File {
   return File::open(s).expect("File not found");
}