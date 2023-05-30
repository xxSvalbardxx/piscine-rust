pub fn arrange_phrase(phrase: &str) -> String {
    let mut new_ph = String::new();
    let mut new_word = String::new();
    let mut temp_ph = Vec::new();
    for ch in phrase.chars() {
        if ch.is_alphabetic() {
            new_word.push(ch);
        } else if ch.is_numeric() {
            // insert the number at the beginning of the word
            new_word.insert(0, ch);
        } else {
            // if the character is not alphanumeric, push the word to the vector
            temp_ph.push(new_word.clone()); // or | temp_ph.push(new_word);
            new_word.clear();               //    | new_word = String::new();
        }
    }
    // push the last word to the vector because there is no space after it to trigger the else statement above
    temp_ph.push(new_word);
    // sort the vector
    temp_ph.sort();
    // join the vector into a string and remove the numbers
    for word in temp_ph {
        new_ph.push_str(&word[1..]);
        new_ph.push(' ');
    }
    // remove the last space
    new_ph.pop();
    new_ph
}

        


        

        

        


#[cfg(test)]
mod tests {
    use arrange_it::*;

fn main() {
    println!("{:?}", arrange_phrase("is2 Thi1s T4est 3a"));
}
}
