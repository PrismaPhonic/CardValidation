/** 
 * Pre-defined card data
 */

static VISA_PATTERN: CardPattern = CardPattern {
    card_type: CardType::Visa,
    lengths: vec![13, 14, 15, 16],
    prefixes: vec!["4"],
};

static MASTERCARD_PATTERN: CardPattern = CardPattern {
    card_type: CardType::Mastercard,
    lengths: vec![16],
    prefixes: vec!["51", "52", "53", "54", "55"],
};

static DINERSCLUB_PATTERN: CardPattern = CardPattern {
    card_type: CardType::DinersClub,
    lengths: vec![14],
    prefixes: vec!["3"],
};

#[derive(Debug, PartialEq)]
struct CardPattern {
    card_type: CardType,
    lengths: Vec<usize>,
    prefixes: Vec<&'static str>,
}

impl CardPattern {
    pub fn validate(&self) -> bool {

    }
}

#[derive(Debug, PartialEq)]
pub enum CardType {
    Visa,
    Mastercard,
    DinersClub,
}

#[derive(Debug, PartialEq)]
pub enum ValidationError {
    InvalidPrefix,
    InvalidLength,
}

impl CardType {
    fn all_cards() -> Vec<CardType> {
        vec![
            CardType::Visa,
            CardType::Mastercard,
            CardType::DinersClub,
        ]
    }

    fn match_pattern(&self) -> bool {
        match *self {
            CardType::Visa => 
        }
    }
}

pub fn get_card_type(card_number: &str) -> Result<CardType, ValidationError> {
   let cards =  
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
