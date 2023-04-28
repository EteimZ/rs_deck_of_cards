#[derive(Debug, Clone, Copy)]
pub enum Suit {
    Hearts,
    Clubs,
    Diamonds,
    Spades,
}

#[derive(Debug, Clone, Copy)]
pub enum Value {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

#[derive(Debug)]
pub struct Card {
    pub suit: Suit,
    pub value: Value,
}

impl Suit {
    pub fn iter() -> std::slice::Iter<'static, Suit> {
        static SUITS: [Suit; 4] = [Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades];
        SUITS.iter()
    }
}

impl Value {
    pub fn iter() -> std::slice::Iter<'static, Value> {
        static VALUES: [Value; 13] = [
            Value::Ace,
            Value::Two,
            Value::Three,
            Value::Four,
            Value::Five,
            Value::Six,
            Value::Seven,
            Value::Eight,
            Value::Nine,
            Value::Ten,
            Value::Jack,
            Value::Queen,
            Value::King,
        ];
        VALUES.iter()
    }
}

