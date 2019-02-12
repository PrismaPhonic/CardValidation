use card_validate::*;

// this main function demonstrates what is already being tested - that a successful validation
// will allow us to print the card type as text to stdout even though it retains its type
fn main() {
    let valid_number = "41234987654637";
    if let Ok(card_type) = get_card_type(valid_number) {
        println!("That card is a {}!", card_type);
    }
}
