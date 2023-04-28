# Rust Deck of cards

Implementation of a deck of cards in rust.

## Installation

Place the following in your `Cargo.toml` file under `[dependencies]`:

```toml
rs-deck-of-cards = { git = "https://github.com/EteimZ/rs_deck_of_cards" }
```

Then run `cargo build` or `cargo run` to build or run your project respectively.

## Usage

Sample usage of the library:

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

## API

The following APIs are available:

### Modules

- `cards` - The main module that contains submodules: `card` and `deck`.

### Structs

- `Card` - Represents a card in the deck. It contains a `Suit` and a `Value` enum.
- `Deck` - Represents a deck of cards.

### Enums

- `Suit` - Represents the suit of a card.
- `Value` - Represents the value of a card.
