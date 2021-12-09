// Module declarations
mod card;
mod deck;

fn main() {
    // Create a deck
    let mut deck: deck::Deck = deck::Deck {cards: deck::create_deck(4, true)};

    // Create a hand, just a list of cards
    let mut hand: Vec<card::Card> = Vec::new();
    // Deal four cards from the deck
    hand.push(deck.deal());
    hand.push(deck.deal());
    
    // Display lines in hand
    for n in 0..6 {
        for c in &hand {
            print!("{}   ", c.clone().get_display_line(n));
        }
        println!("");
    }
}