#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CipherError {
    // expected public fields
    pub validation : bool,
    pub expected : String,
}
impl CipherError {
    pub fn new(validation: bool, expected: String) -> CipherError {
        CipherError{
            validation : validation,
            expected : expected,
        }
    }
}
pub fn cipher(original: &str, ciphered: &str) -> Option<Result<bool, CipherError>> {
    if (original.len() == 0) && (ciphered.len() == 0) {
        return None;
    }
        let mut cipher_test = String::new();

        for c in original.chars() {
            if c.is_alphabetic() {
                if c.is_uppercase() {
                    //let mut ascii_original = c as u8;
                    let ascii_ciphered = 'Z' as u8 - c as u8 + 'A' as u8;
                    cipher_test.push(ascii_ciphered as char);
                } else {
                    //let mut ascii_original = c as u8;
                    let ascii_ciphered = 'z' as u8 - c as u8 + 'a' as u8;
                    cipher_test.push(ascii_ciphered as char);
                }
            } else {
                cipher_test.push(c);
            }
        }
        if cipher_test == ciphered {
            return Some(Ok(true));
        } else {
            return Some(Err(CipherError::new(false, cipher_test.to_string())));
        }
    
}