use crate::deck::DeckCard;

pub struct Player {
    hand: Vec<DeckCard>,
    collected_cards: Vec<DeckCard>,
    score: u8,
    tokens_collected: u8,
}