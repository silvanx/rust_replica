use crate::deck::{DeckCard, CardColor};

const CARD_POINT_VALUE: u8 = 1;
const TOKEN_POINT_VALUE: u8 = 3;

pub struct Player {
    hand: Vec<DeckCard>,
    collected_cards: Vec<DeckCard>,
    tokens_collected: u8,
    color: CardColor,
}

impl Player {
    pub fn calculate_score(&self) -> u8 {
        let card_points = self.collected_cards
            .iter()
            .filter(|card| {
                match card {
                    DeckCard::BaseCard(c) => c.color == self.color,
                    DeckCard::ActionCard(_) => false
                }})
            .count() as u8 * CARD_POINT_VALUE;
        let token_points = self.tokens_collected * TOKEN_POINT_VALUE;
        card_points + token_points
    }
}

#[cfg(test)]
mod tests {
    use crate::deck::{BaseCard, CardColor, CardValue, DeckCard};
    use crate::player::Player;

    #[test]
    fn test_player_score() {
        let player = Player {
            hand: vec![],
            collected_cards: vec![
                DeckCard::BaseCard(BaseCard::new (CardColor::Blue, CardValue::Adenine)),
                DeckCard::BaseCard(BaseCard::new (CardColor::Blue, CardValue::Adenine)),
                DeckCard::BaseCard(BaseCard::new (CardColor::Green, CardValue::Guanine)),
            ],
            tokens_collected: 2,
            color: CardColor::Green,
        };
        assert_eq!(player.calculate_score(), 7);
    }
}