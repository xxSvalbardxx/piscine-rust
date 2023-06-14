pub fn delete_prefix<'a>(prefix: &'a str, s: &'a str) -> Option<&'a str> {
    if s.starts_with(prefix) {
        Some(&s[prefix.len()..])
    } else {
        None
    }
}

// we need a lifetime because we are returning a reference to a part of the string, and we need to tell the compiler that the reference is valid for the lifetime of the string because the string is the owner of the data and the reference is pointing to a part of the data owned by the string and the string is the only owner of the data so if the string is dropped the reference will be invalid and we don't want that to happen