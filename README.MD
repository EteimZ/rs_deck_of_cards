# Rust Deck of cards

Implementation of a deck of cards in rust.

## Installation

Place the following in your `Cargo.toml` file under `[dependencies]`:

```toml
rs-deck-of-cards = { git = "https://github.com/EteimZ/rs_deck_of_cards" }
```

Then run `cargo build` or `cargo run` to build or run your project respectively.

## Usage

After installing the crate, you can use it in your project by adding the following to your `main.rs` file:

```rust
use rs_deck_of_cards::Deck;

fn main() {
    let mut deck = Deck::new();
    
    // print the deck
    deck.print();

    // shuffle the deck
    deck.shuffle();

    // print the deck again
    deck.print();
}
```

For a more comprehensive example, check this [file](https://github.com/EteimZ/rs_deck_of_cards/blob/main/src/main.rs).

