pub fn open_or_create(file: &str, content: &str) {
    File::open(file).unwrap_or_else(|_| {
        File::create(file).unwrap();
        File::open(file).unwrap()
    });
}
