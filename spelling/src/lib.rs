pub fn spell(n: u64) -> String {
    // Only positive numbers will be tested, up to "one million".
    match n {
        0..=9 => spell_ones(n),
        10..=19 => spell_teens(n),
        20..=99 => spell_tens(n),
        100..=999 => spell_hundreds(n),
        1000..=999999 => spell_thousands(n),

        1000000 => "one million".to_string(),
        _ => panic!("Number out of range 0 to 1M : {}", n),
    }
}

fn spell_ones(n: u64) -> String {
    match n {
        0 => "zero".to_string(),
        1 => "one".to_string(),
        2 => "two".to_string(),
        3 => "three".to_string(),
        4 => "four".to_string(),
        5 => "five".to_string(),
        6 => "six".to_string(),
        7 => "seven".to_string(),
        8 => "eight".to_string(),
        9 => "nine".to_string(),
        _ => panic!("Ones out of range: {}", n),
    }
}

fn spell_teens(n: u64) -> String {
    match n {
        10 => "ten".to_string(),
        11 => "eleven".to_string(),
        12 => "twelve".to_string(),
        13 => "thirteen".to_string(),
        14 => "fourteen".to_string(),
        15 => "fifteen".to_string(),
        16 => "sixteen".to_string(),
        17 => "seventeen".to_string(),
        18 => "eighteen".to_string(),
        19 => "nineteen".to_string(),
        _ => panic!("Teens out of range: {}", n),
    }
}

fn spell_tens(n: u64) -> String {
    let tens = match n / 10 {
        2 => "twenty".to_string(),
        3 => "thirty".to_string(),
        4 => "forty".to_string(),
        5 => "fifty".to_string(),
        6 => "sixty".to_string(),
        7 => "seventy".to_string(),
        8 => "eighty".to_string(),
        9 => "ninety".to_string(),
        _ => panic!("Tens out of range: {}", n),
    };

    let mut ones = match n % 10 {
        0 => "".to_string(),
        1 => "-one".to_string(),
        2 => "-two".to_string(),
        3 => "-three".to_string(),
        4 => "-four".to_string(),
        5 => "-five".to_string(),
        6 => "-six".to_string(),
        7 => "-seven".to_string(),
        8 => "-eight".to_string(),
        9 => "-nine".to_string(),
        _ => panic!("Tens's Ones out of range: {}", n),
    };
    // if there is 2 witespaces in "ones", remove one
    for _ in 0..ones.len() {
        if ones.contains("  ") {
            ones.remove(0);
        }
    }
    format!("{}{}", tens, ones)
}

fn spell_hundreds(n: u64) -> String {
    let hundreds = match n / 100 {
        0 => "".to_string(),
        1 => "one hundred".to_string(),
        2 => "two hundred".to_string(),
        3 => "three hundred".to_string(),
        4 => "four hundred".to_string(),
        5 => "five hundred".to_string(),
        6 => "six hundred".to_string(),
        7 => "seven hundred".to_string(),
        8 => "eight hundred".to_string(),
        9 => "nine hundred".to_string(),
        _ => panic!("Hundreds out of range: {}", n),
    };

    let mut tens = match n % 100 {
        0 => "".to_string(),
        1..=9 => format!(" {}", spell_ones(n % 100)),
        10..=19 => format!(" {}", spell_teens(n % 100)),
        20..=99 => format!(" {}", spell_tens(n % 100)),
        _ => panic!("Hundreds's Tens out of range: {}", n),
    };

    for _ in 0..tens.len() {
        if tens.contains("  ") {
            tens.remove(0);
        }
    }

    format!("{}{}", hundreds, tens)
}

fn spell_thousands(n: u64) -> String {

    let mut thousands = match n / 1000 {
        0..=9 => spell_ones(n/1000),
        10..=19 => spell_teens(n/1000),
        20..=99 => spell_tens(n/1000),
        100..=999 => spell_hundreds(n/1000),
        _ => panic!("Thousands out of range: {}", n),
    };
    
    
    let mut hundreds = match n % 1000 {
        0 => "".to_string(),
        1..=9 => spell_ones(n % 1000),
        10..=19 => spell_teens(n % 1000),
        20..=99 => spell_tens(n % 1000),
        100..=999 => spell_hundreds(n % 1000),
        _ => panic!("Thousands's Hundreds out of range: {}", n),
    };

    if hundreds.len() <= 0 {
        thousands.push_str(" thousand");
    } else {
        thousands.push_str(" thousand ");
    }

    for _ in 0..hundreds.len() {
        if hundreds.contains("  ") {
            hundreds.remove(0);
        } 
    }   

    
    format!("{}{}", thousands, hundreds)
}

