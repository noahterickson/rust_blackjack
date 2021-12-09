/// Deck module, holds deck functions!
use crate::card::Card;
use rand::thread_rng;
use rand::seq::SliceRandom;

pub struct Deck {
    pub cards: Vec<Card>
}

impl Deck {
    /// Deal function, pops a single card off the deck
    pub fn deal(&mut self) -> Card {
        return self.cards.pop().unwrap();
    }

    #[allow(dead_code)]
    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut thread_rng());
    }

}

/// Create a new deck based on standard card deck. Supports multiple decks and shuffling.
pub fn create_deck(deck_count: u8, shuffle: bool) -> Vec<Card> {
    const FACES: &[&str] = &["A", "K", "Q", "J", "10", "9", "8", "7", "6", "5", "4", "3", "2"];
    const SUITS: &[&str] = &["Hearts", "Spades", "Diamonds", "Clubs"];
    let mut cards: Vec<Card> = Vec::new();
    for _ in 0..deck_count {
        for s in SUITS {
            for f in FACES {
                cards.push(Card{face: f.to_string(), suit:s.to_string()});
            }
        }
    }
    if shuffle {
        cards.shuffle(&mut thread_rng());
    }
    return cards;
}
