# Rust Credit Card Validator

This crate is a simple proof of concept on how to validate credit card numbers
based on length and prefix.

### Setup

You may install this in one of two ways. If you have `cargo` installed then it's very easy. If
not, you can install rust and cargo by following this very simple [cargo setup](https://doc.rust-lang.org/cargo/getting-started/installation.html) process.

### Running

Clone this repo and in the root directory run it (which will compile it first):

```terminal
$ git clone https://github.com/PrismaPhonic/CardValidation.git
$ cd CardValidation
$ cargo run
```

### Testing

Simple run the following in the root project directory:

```terminal
$ cargo test
```

### Project Layout

All of the core logic is in `src/lib.rs`.  You can find the main binary in
`src/main.rs`.  The only thing that main is doing currently, is taking a valid
visa credit card number, and then validating it to return the custom type of
`CardType::Visa`.  This is then printed to the terminal as simply the string
`Visa`.  
