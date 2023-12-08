mod deck;

fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
mod tests {
    use crate::deck::{ActionCard, ActionType, BaseCard, CardColor, CardValue, Deck, DeckCard};

    #[test]
    fn new_deck_is_empty() {
        let deck = Deck::new();
        assert_eq!(deck.cards_left(), 0);
    }

    #[test]
    fn recognise_stop_cards() {
        let cards: Vec<DeckCard> = vec![
            DeckCard::ActionCard(ActionCard::new (ActionType::Mutation)),
            DeckCard::ActionCard(ActionCard::new (ActionType::Stop)),
            DeckCard::BaseCard(BaseCard::new (CardColor::Blue, CardValue::Cytosine)),
        ];
        assert!(!crate::deck::is_stop_card(&cards[0]));
        assert!(crate::deck::is_stop_card(&cards[1]));
        assert!(!crate::deck::is_stop_card(&cards[2]));
    }

    #[test]
    fn deck_from_vec() {
        let cards: Vec<DeckCard> = vec![
            DeckCard::ActionCard(ActionCard::new (ActionType::Mutation)),
            DeckCard::ActionCard(ActionCard::new (ActionType::Stop)),
            DeckCard::BaseCard(BaseCard::new (CardColor::Blue, CardValue::Cytosine)),
            DeckCard::BaseCard(BaseCard::new (CardColor::Blue, CardValue::Adenine)),
            DeckCard::BaseCard(BaseCard::new (CardColor::Blue, CardValue::Adenine)),
            DeckCard::BaseCard(BaseCard::new (CardColor::Green, CardValue::Guanine)),
        ];
        let deck = Deck::from_vec(cards);

        assert_eq!(deck.cards_left(), 6);
        assert_eq!(deck.stop_cards_left(), 1);
    }
}