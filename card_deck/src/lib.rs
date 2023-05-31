
use rand::Rng;

#[derive(Debug, PartialEq)]
pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}

#[derive(Debug, PartialEq)]
pub enum Rank {
    Ace,
    Number(u8),
    Jack,
    Queen,
    King,
}

impl Suit {
	pub fn random() -> Suit {
        let mut rng = rand::thread_rng();
        /* match rng.gen::<u8>() % 4 + 1 {
            1 => Suit::Heart,
            2 => Suit::Diamond,
            3 => Suit::Spade,
            4 => Suit::Club,
            _ => unreachable!(),
        } */
        Suit::translate(rng.gen::<u8>() % 4 + 1)
        
	}

	pub fn translate(value: u8) -> Suit {
        match value {
            1 => Suit::Heart,
            2 => Suit::Diamond,
            3 => Suit::Spade,
            4 => Suit::Club,
            _ => unreachable!(),
        }
	}
}

impl Rank {
	pub fn random() -> Rank {
        let mut rng = rand::thread_rng();
        /* match rng.gen::<u8>() % 13 + 1{
            1 => Rank::Ace,
            2..=10 => Rank::Number(rng.gen::<u8>() % 10 + 1),
            11 => Rank::Jack,
            12 => Rank::Queen,
            13 => Rank::King,
            _ => unreachable!(),
        } */
        Rank::translate(rng.gen::<u8>() % 13 + 1)
        
	}

	pub fn translate(value: u8) -> Rank {
        match value {
            1 => Rank::Ace,
            2..=10 => Rank::Number(value),
            11 => Rank::Jack,
            12 => Rank::Queen,
            13 => Rank::King,
            _ => unreachable!(),
        }
	}
}

#[derive(Debug, PartialEq)]
pub struct Card {
	pub suit: Suit,
	pub rank: Rank,
}

pub fn winner_card(card: &Card) -> bool {
    match (&card.suit, &card.rank) {
        (Suit::Spade, Rank::Ace) => true,
        _ => false,
    }
    
}
