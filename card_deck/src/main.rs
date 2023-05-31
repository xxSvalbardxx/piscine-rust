use card_deck::{Card, Rank, Suit};

fn main() {
	let your_card = Card {
		rank: Rank::random(),
		suit: Suit::random(),
	};

	println!("Your card is {:?}", your_card);

	// Now if the card is an Ace of Spades print "You are the winner"
	if card_deck::winner_card(&your_card) {
		println!("You are the winner!");
	}
}