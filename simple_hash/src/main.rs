use simple_hash::*;

fn main() {
    let sentence = "this is a very basic sentence with only few \
                repetitions. once again this is very basic and \
                but it should be enough for basic tests".to_string();
    let words = sentence.split(" ").collect::<Vec<&str>>();

    let frequency_count = word_frequency_counter(words);
    println!("{:?}", frequency_count);
    println!("{}", nb_distinct_words(&frequency_count));
}
