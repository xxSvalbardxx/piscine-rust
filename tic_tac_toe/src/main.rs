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