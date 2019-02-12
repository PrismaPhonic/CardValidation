#[macro_use]
extern crate lazy_static;

use std::fmt;

/** 
 * Pre-defined card data
 */

lazy_static! {
    static ref VISA_PATTERN: CardPattern = CardPattern {
        card_type: CardType::Visa,
        lengths: vec![13, 14, 15, 16],
        prefixes: vec!["4"],
    };

    static ref MASTERCARD_PATTERN: CardPattern = CardPattern {
        card_type: CardType::Mastercard,
        lengths: vec![16],
        prefixes: vec!["51", "52", "53", "54", "55"],
    };

    static ref DINERSCLUB_PATTERN: CardPattern = CardPattern {
        card_type: CardType::DinersClub,
        lengths: vec![14],
        prefixes: vec!["3"],
    };
}

#[derive(Debug, PartialEq)]
pub struct CardPattern {
    card_type: CardType,
    lengths: Vec<usize>,
    prefixes: Vec<&'static str>,
}

impl CardPattern {
    pub fn validate(&self, card_number: &'static str) -> Result<CardType, ValidationError> {
        if !self.is_length_valid(card_number.len()) {
            return Err(ValidationError::InvalidLength);
        }

        if !self.is_prefix_valid(card_number) {
            return Err(ValidationError::InvalidPrefix);
        }

        Ok(self.card_type)
    }

    pub fn is_length_valid(&self, length: usize) -> bool {
        self.lengths.contains(&length)
    }

    pub fn is_prefix_valid(&self, card_number: &'static str) -> bool {
        for prefix in &self.prefixes {
            if card_number.starts_with(prefix) {
                return true;
            }
        }

        false
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CardType {
    Visa,
    Mastercard,
    DinersClub,
}

impl CardType {
    pub fn all_cards() -> Vec<CardType> {
        vec![
            CardType::Visa,
            CardType::Mastercard,
            CardType::DinersClub,
        ]
    }

    pub fn validate(&self, card_number: &'static str) -> Result<CardType, ValidationError> {
        match *self {
            CardType::Visa => VISA_PATTERN.validate(card_number),
            CardType::Mastercard => MASTERCARD_PATTERN.validate(card_number),
            CardType::DinersClub => DINERSCLUB_PATTERN.validate(card_number),
        }
    }
}

pub fn get_card_type(card_number: &'static str) -> Result<CardType, ValidationError> {
   let cards = CardType::all_cards();

   for card in cards {
       if let Ok(card_type) = card.validate(card_number) {
           return Ok(card_type);
       }
   }

   Err(ValidationError::InvalidPattern)
}

// This simply instructs Rust on how to turn our custom types back into strings for string
// formatting.
impl fmt::Display for CardType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CardType::Visa => write!(f, "Visa"),
            CardType::Mastercard => write!(f, "Mastercard"),
            CardType::DinersClub => write!(f, "Dinersclub"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ValidationError {
    InvalidPrefix,
    InvalidLength,
    InvalidPattern,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visa_valid() {
        let valid_number = "41234987654637";
        assert_eq!(get_card_type(valid_number), Ok(CardType::Visa));
    }

    #[test]
    fn test_visa_valid_string_formatting() {
        let valid_number = "41234987654637";
        assert_eq!(format!("{}", get_card_type(valid_number).unwrap()), "Visa");
    }

    #[test]
    fn test_visa_invalid_length() {
        let invalid_number = "42938478231";
        assert_eq!(get_card_type(invalid_number), Err(ValidationError::InvalidPattern));
    }

    #[test]
    fn test_visa_invalid_prefix() {
        let invalid_number = "32938478231";
        assert_eq!(get_card_type(invalid_number), Err(ValidationError::InvalidPattern));
    }

    #[test]
    fn test_mastercard_valid() {
        let valid_number = "5219847562398473";
        assert_eq!(get_card_type(valid_number), Ok(CardType::Mastercard));
    }

    #[test]
    fn test_mastercard_invalid_length() {
        let invalid_number = "521984756239847339482910";
        assert_eq!(get_card_type(invalid_number), Err(ValidationError::InvalidPattern));
    }

    #[test]
    fn test_mastercard_invalid_prefix() {
        let invalid_number = "571984756239847339482910";
        assert_eq!(get_card_type(invalid_number), Err(ValidationError::InvalidPattern));
    }

    #[test]
    fn test_dinersclub_valid() {
        let valid_number = "30938475869475";
        assert_eq!(get_card_type(valid_number), Ok(CardType::DinersClub));
    }

    #[test]
    fn test_dinersclub_invalid_length() {
        let invalid_number = "309384758694756";
        assert_eq!(get_card_type(invalid_number), Err(ValidationError::InvalidPattern));
    }

    #[test]
    fn test_dinersclub_invalid_prefix() {
        let invalid_number = "10938475869475";
        assert_eq!(get_card_type(invalid_number), Err(ValidationError::InvalidPattern));
    }
}
