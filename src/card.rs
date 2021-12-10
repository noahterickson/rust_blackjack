/// Card module, holds card functions!
#[derive(Clone)]
pub struct Card {
    pub face: String,
    pub suit: String,
}

/// Card functions
impl Card {
    /// Return card name as "number of suit" ex: 10 of Diamonds
    #[allow(dead_code)]
    pub fn get_name(&self) -> String{
        return [self.face.as_ref(), self.suit.as_ref()].join(" of ");
    }

    /// Return face value of card
    // TODO: Add option for Ace to be 1 or 11.
    pub fn value(&self) -> u8 {
        match self.face.as_ref() {
            "A" => return 11,
            "K" | "Q" | "J" => return 10,
            _ => return self.face.parse::<u8>().unwrap()
        }
    }

    /// Returns unicode character for suit type
    fn get_code(self) -> String {
        match self.suit.as_ref() {
            "Diamonds" => return "\u{25C7}".to_string(),
            "Hearts" => return "\u{2661}".to_string(),
            "Spades" => return "\u{2664}".to_string(),
            "Clubs" => return "\u{2667}".to_string(),
            _ => return "".to_string()
        }
    }

    /// Gets a line of the card to display, 6 lines total.
    /// Example of iterating over all 6 lines for "2 of Hearts":
    /// --------
    /// |2 ♡   |
    /// |      |
    /// |      |
    /// |    2♡|
    /// --------
    pub fn get_display_line(self, line: u8) -> String {
        let val: String = self.face.to_string();
        let code: String = self.get_code().to_string();
        match line {
            0 | 5 => return "--------".to_string(),
            1 => return format!("|{:<2}{}   |", val, code),
            2 | 3 => return "|      |".to_string(),
            4 => return format!("|   {:>2}{}|", val, code),
            _ => return "".to_string()
        }
    }
}
