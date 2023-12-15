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

    pub fn new(color: CardColor) -> Player {
        Player {
            hand: vec![],
            collected_cards: vec![],
            tokens_collected: 0,
            color: color,
        }
    }

    pub fn count_cards_in_hand(&self) -> u8 {
        self.hand.len() as u8
    }

    pub fn count_collected_cards(&self) -> u8 {
        self.collected_cards.len() as u8
    }

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

    pub fn collect_cards(&mut self, mut cards: Vec<DeckCard>) {
        let mut filtered_cards: Vec<DeckCard> = cards
            .iter()
            .filter(|c| c.is_base_card())
            .map(|e| *e)
            .collect();
        self.collected_cards.append(&mut filtered_cards);
    }

    pub fn draw_cards(&mut self, mut cards: Vec<DeckCard>) {
        self.hand.append(&mut cards);
    }

}

#[cfg(test)]
mod tests {
    use crate::deck::{ActionCard, ActionType, BaseCard, CardColor, CardValue, DeckCard};
    use crate::player::Player;

    #[test]
    fn player_score() {
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

    #[test]
    fn player_new() {
        let player = Player::new(CardColor::Blue);
        assert_eq!(player.calculate_score(), 0);
        assert_eq!(player.count_cards_in_hand(), 0);
        assert_eq!(player.count_collected_cards(), 0);
    }

    #[test]
    fn player_collect_cards() {
        let mut player = Player::new(CardColor::Blue);
        let cards_to_collect = vec![
            DeckCard::BaseCard(BaseCard::new(CardColor::Blue, CardValue::Adenine)),
            DeckCard::BaseCard(BaseCard::new(CardColor::Blue, CardValue::ThymineUracil)),
            DeckCard::BaseCard(BaseCard::new(CardColor::Green, CardValue::Guanine)),
            DeckCard::ActionCard(ActionCard::new(ActionType::Mutation)),
            DeckCard::ActionCard(ActionCard::new(ActionType::Stop)),
        ];
        player.collect_cards(cards_to_collect);
        assert_eq!(player.calculate_score(), 2);
        assert_eq!(player.count_collected_cards(), 3);
    }

    #[test]
    fn player_draw_cards() {
        let mut player = Player::new(CardColor::Blue);
        let new_cards = vec![
            DeckCard::BaseCard(BaseCard::new (CardColor::Blue, CardValue::Adenine)),
            DeckCard::BaseCard(BaseCard::new (CardColor::Blue, CardValue::ThymineUracil)),
            DeckCard::BaseCard(BaseCard::new (CardColor::Green, CardValue::Guanine)),
            DeckCard::BaseCard(BaseCard::new (CardColor::Blue, CardValue::Adenine)),
            DeckCard::BaseCard(BaseCard::new (CardColor::Blue, CardValue::ThymineUracil)),
            DeckCard::BaseCard(BaseCard::new (CardColor::Green, CardValue::Guanine)),
        ];
        player.draw_cards(new_cards);
        assert_eq!(player.calculate_score(), 0);
        assert_eq!(player.count_collected_cards(), 0);
        assert_eq!(player.count_cards_in_hand(), 6);
    }
}