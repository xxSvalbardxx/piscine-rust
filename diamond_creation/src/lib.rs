pub fn get_diamond(c: char) -> Vec<String> {
    let mut rslt = Vec::new();
    let n = (c as u8 - b'A') as usize;

    for i in 0..n {
        let mut line : Vec<char> = vec![' '; 2*n+1];
        line[n-i] = (b'A' + i as u8 ) as char;
        line[n+i] = (b'A' + i as u8 ) as char;
        rslt.push(line.iter().collect());
    }
    for i in (0..n).rev() {
        let mut line : Vec<char> = vec![' '; 2*n+1];
        line[n-i] = (b'A' + i as u8 ) as char;
        line[n+i] = (b'A' + i as u8 ) as char;
        rslt.push(line.iter().collect());
    }
    rslt
}

