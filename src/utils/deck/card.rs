#[derive(Debug, PartialEq)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Rank {
    TWO,
    THREE,
    FOUR,
    FIVE,
    SIX,
    SEVEN,
    EIGHT,
    NINE,
    TEN,
    JACK,
    QUEEN,
    KING,
    ACE,
}

#[derive(Debug, PartialEq)]
pub enum Suit {
    CLUBS,
    // DIAMONDS,
    // HEARTS,
    // SPADES,
}
