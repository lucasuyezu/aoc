#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
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
    JOKER,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Suit {
    CLUBS,
    // DIAMONDS,
    // HEARTS,
    // SPADES,
}
