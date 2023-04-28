use rs_deck_of_cards::{Card, Suit, Value};
use rs_deck_of_cards::Deck;

fn main() {

    // create a new deck of cards 
    let mut deck = Deck::new();

    // shuffle the deck
    deck.shuffle();
    
    // create a card 
    let ace_of_spades =  Card {
        suit: Suit::Spades,
        value: Value::Ace,
    };

    // add the card to the deck
    deck.add_card(ace_of_spades);

    // check deck length (it should be 53)
    assert_eq!(53, deck.len());

    // print deck
    deck.print();

    // split the original deck at position 3
    let mut deck2 = deck.split_deck(3).unwrap();
    
    // original deck now of length 3
    assert_eq!(3, deck.len());

    // new deck should be of length 3
    assert_eq!(50, deck2.len());

    // create 2 cards
    let ace_of_hearts =  Card {
        suit: Suit::Hearts,
        value: Value::Ace,
    };

    let four_of_spades =  Card {
        suit: Suit::Spades,
        value: Value::Four,
    };

    // Add the cards to a vector
    let cards = vec![ace_of_hearts, four_of_spades];

    // Add the cards to the second deck
    deck2.add_cards(cards);

    // the deck should be 52
    assert_eq!(52, deck2.len());

    // reset the original deck
    deck.reset();

    // print it
    deck.print();

    // It should be 52
    assert_eq!(52, deck2.len());

}
