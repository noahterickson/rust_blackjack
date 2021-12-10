use std::io;

// Module declarations
mod card;
mod deck;

struct Player {
    pub money: f32,
}


fn get_hand_total(cards: &Vec<card::Card>) -> u8 {
    let mut total: u8 = 0;
    for c in cards {
        total = total + c.value();
    }
    return total;
}

fn check_blackjack(cards: &Vec<card::Card>) -> bool {
    println!("{}", cards.len());
    if cards.len() == 2 {
        match cards[0].face.as_ref() {
            "A" | "K" | "Q" | "J" | "10" => (), // do nothing
            _ => return false
        }
        match cards[1].face.as_ref() {
            "A" | "K" | "Q" | "J" | "10" => return true, // Both cards face cards!
            _ => return false
        }
    }
    return false;
}

enum Outcome {
    Blackjack,
    Win,
    Lose,
    Bust
}

fn play_round(deck: &mut deck::Deck, wager: f32, money: f32) -> Outcome {

    // Create a hand, just a list of cards
    let mut hand: Vec<card::Card> = Vec::new();
    let mut dealer: Vec<card::Card> = Vec::new();

    // Deal first two cards cards from the deck
    hand.push(deck.deal());
    //hand.push(card::Card{face: "A".to_string(), suit:"Hearts".to_string()});
    dealer.push(deck.deal());
    hand.push(deck.deal());
    //hand.push(card::Card{face: "10".to_string(), suit:"Hearts".to_string()});
    dealer.push(deck.deal());

    loop {
        let player_total: u8 = get_hand_total(&hand);
        println!("Player Money: ${:.2}", money);
        println!("Wager: ${:.2}", wager);
        println!("Player hand total: {}", player_total);

        // Display lines in hand
        for n in 0..6 {
            for c in &hand {
                print!("{}   ", c.clone().get_display_line(n));
            }
            println!("");
        }
        // Check for blackjack or bust
        if player_total == 21 && check_blackjack(&hand) {
            println!("Blackjack! Player wins ${:.2}!", wager * 1.5);
            return Outcome::Blackjack;
        }
        if player_total > 21 {
            println!("Bust! Player loses ${:.2}!", wager);
            return Outcome::Bust;
        }

        println!("(H)it or (S)tand?");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Uh oh!");
        let hit = match input.trim().as_ref() {
            "H" | "h" => true,
            _ => false
        };
        if hit {
            println!("Hitting!");
            hand.push(deck.deal());
        } else {
            println!("Player stands. Dealer's turn.");
            loop {
                dealer.push(deck.deal());
                let dealer_total = get_hand_total(&dealer);
                println!("Dealer hand total: {:.2}", dealer_total);
                // Display lines in hand
                for n in 0..6 {
                    for c in &dealer {
                        print!("{}   ", c.clone().get_display_line(n));
                    }
                    println!("");
                }
                if dealer_total > 21 {
                    println!("Dealer busts! Player wins ${:.2}!", wager);
                    return Outcome::Win;
                } else if dealer_total > 17 {
                    if dealer_total > player_total {
                        println!("Dealer wins! Player loses ${:.2}!", wager);
                        return Outcome::Lose;
                    } else {
                        println!("Dealer stands! Player wins ${:.2}!", wager);
                        return Outcome::Win;
                    }
                }

            }
        }
    }
}

fn get_wager(money: f32) -> f32 {
    loop {
        // Get player wager
        println!("Player Money: ${:.2}\nEnter Round Wager: $", money);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Uh oh!");
        let wager: f32 = input.trim().parse().unwrap();

        if wager > money {
            println!("Insufficient funds!");
        } else if wager < 0.01 {
            println!("Minimum wager is $0.01! No free rides, cheapskate!")
        } else {
            return wager;
        }
    }
}

fn main() {
    // Create a deck
    let mut deck: deck::Deck = deck::Deck {cards: deck::create_deck(4, true)};
    let mut player = Player {money: 100.00};

    let mut playing: bool = true;
    while playing {
        // Get player wager
        let wager: f32 = get_wager(player.money);

        // Play a round
        let outcome: Outcome = play_round(&mut deck, wager, player.money);

        // Process outcome and add to player money
        player.money = player.money + match outcome {
            Outcome::Blackjack => wager * 1.5,
            Outcome::Win => wager,
            Outcome::Lose | Outcome::Bust => wager * -1.0
        };

        // Ask user if they want to keep playing
        println!("Play again? (Y)es or (N)o?");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Uh oh!");
        playing = match input.trim().as_ref() {
            "Y" | "y" => true,
            "N" | "n" => false,
            _ => false
        };
    }
}