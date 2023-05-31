
pub fn tic_tac_toe(table: Vec<Vec<&str>>) -> String {
    if horizontal("X", &table) || vertical("X", &table) || diagonals("X", &table) {
        return "player X won".to_string();
    } else if horizontal("O", &table) || vertical("O", &table) || diagonals("O", &table) {
        return "player O won".to_string();
    } else {
        return "tie".to_string();
    }
}

pub fn diagonals(player: &str, table: &Vec<Vec<&str>>) -> bool {
    if table[0][0] == player && table[1][1] == player && table[2][2] == player {
        return true;
    } else if table[0][2] == player && table[1][1] == player && table[2][0] == player {
        return true;
    }
    false
}

pub fn horizontal(player: &str, table: &Vec<Vec<&str>>) -> bool {
    if table[0][0] == player && table[0][1] == player && table[0][2] == player {
        return true;
    } else if table[1][0] == player && table[1][1] == player && table[1][2] == player {
        return true;
    } else if table[2][0] == player && table[2][1] == player && table[2][2] == player {
        return true;
    }
    false
}

pub fn vertical(player: &str, table: &Vec<Vec<&str>>) -> bool {
    if table[0][0] == player && table[1][0] == player && table[2][0] == player {
        return true;
    } else if table[0][1] == player && table[1][1] == player && table[2][1] == player {
        return true;
    } else if table[0][2] == player && table[1][2] == player && table[2][2] == player {
        return true;
    }
    false
}

#[cfg(test)]
mod tests {
    use tic_tac_toe::*;

fn main() {
    println!(
        "{:?}",
        tic_tac_toe(vec![
            vec!["O", "X", "O"],
            vec!["O", "O", "X"],
            vec!["X", "#", "X"]
        ])
    );

    println!(
        "{:?}",
        tic_tac_toe(vec![
            vec!["X", "O", "O"],
            vec!["X", "O", "O"],
            vec!["#", "O", "X"]
        ])
    );

    let dig = vec![
            vec!["O", "O", "X"],
            vec!["O", "X", "O"],
            vec!["X", "#", "X"]
        ];

    println!("{:?}",tic_tac_toe(dig));
}
}
