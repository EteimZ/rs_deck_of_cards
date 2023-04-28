use rand::seq::SliceRandom;

use crate::cards::card::{Card, Suit, Value};

#[derive(Debug)]
pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Deck {
        let mut cards = Vec::new();
        for suit in Suit::iter() {
            for value in Value::iter() {
                cards.push(Card {
                    suit: *suit,
                    value: *value,
                });
            }
        }

        Deck { cards }
    }

    pub fn reset(&mut self) {
        self.cards.clear();
        
        let new_deck = Deck::new();

        self.cards = new_deck.cards;
    }

    // deal a card from the deck
    pub fn deal_card(&mut self) -> Option<Card> {
        if self.is_empty() {
            return None;
        }

        // Remove and return the last card from the deck
        self.cards.pop()
    }

    // shuffle the deck
    pub fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        self.cards.shuffle(&mut rng);
    }

    // add a card to the deck
    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    // add multiple cards
    pub fn add_cards(&mut self, cards: Vec<Card>){
        for card in cards{
            self.cards.push(card);
        }
    }

    // return the number of cards in the deck
    pub fn len(&self) -> usize {
        self.cards.len()
    }

    // return true if the deck is empty
    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    // print the deck
    pub fn print(&self) {
        for card in &self.cards {
            println!("{:?}", card);
        }
    }

    // deal n amount of cards from deck
    pub fn split_deck(&mut self, at: usize) -> Option<Deck>{
        if self.cards.is_empty() {
            return None;
        }

        Some(Deck { cards: self.cards.split_off(at) })
    }


}