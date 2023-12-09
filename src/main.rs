use crate::deck::{ActionCard, ActionType, BaseCard, CardColor, CardValue, DeckCard};

mod deck;

fn main() {
    let cards: Vec<DeckCard> = vec![
        DeckCard::ActionCard(ActionCard::new (ActionType::Mutation)),
        DeckCard::BaseCard(BaseCard::new (CardColor::Blue, CardValue::Cytosine)),
        DeckCard::BaseCard(BaseCard::new (CardColor::Blue, CardValue::Cytosine)),
        DeckCard::BaseCard(BaseCard::new (CardColor::Blue, CardValue::ThymineUracil)),
        DeckCard::BaseCard(BaseCard::new (CardColor::Yellow, CardValue::Adenine)),
        DeckCard::BaseCard(BaseCard::new (CardColor::Red, CardValue::Adenine)),
        DeckCard::BaseCard(BaseCard::new (CardColor::Green, CardValue::Guanine)),
        DeckCard::ActionCard(ActionCard::new (ActionType::Stop)),
    ];
    for card in cards.iter() {
        println!("{}", card);
    }
}


#[cfg(test)]
mod tests {
    use crate::deck::{ActionCard, ActionType, BaseCard, CardColor, CardValue, Deck, DeckCard, is_stop_card};

    fn create_example_deck() -> Deck {
        let cards: Vec<DeckCard> = vec![
            DeckCard::ActionCard(ActionCard::new (ActionType::Mutation)),
            DeckCard::BaseCard(BaseCard::new (CardColor::Blue, CardValue::Cytosine)),
            DeckCard::BaseCard(BaseCard::new (CardColor::Blue, CardValue::Adenine)),
            DeckCard::BaseCard(BaseCard::new (CardColor::Blue, CardValue::Adenine)),
            DeckCard::BaseCard(BaseCard::new (CardColor::Green, CardValue::Guanine)),
            DeckCard::ActionCard(ActionCard::new (ActionType::Stop)),
        ];
        Deck::from_vec(cards)
    }

    #[test]
    fn new_deck_is_empty() {
        let deck = Deck::new();
        assert_eq!(deck.cards_left(), 0);
    }

    #[test]
    fn recognise_stop_cards() {
        let cards: Vec<DeckCard> = vec![
            DeckCard::BaseCard(BaseCard::new (CardColor::Blue, CardValue::Cytosine)),
            DeckCard::ActionCard(ActionCard::new (ActionType::Stop)),
            DeckCard::ActionCard(ActionCard::new (ActionType::Mutation)),
        ];
        assert!(!crate::deck::is_stop_card(&cards[0]));
        assert!(crate::deck::is_stop_card(&cards[1]));
        assert!(!crate::deck::is_stop_card(&cards[2]));
    }

    #[test]
    fn deck_from_vec() {
        let deck = create_example_deck();

        assert_eq!(deck.cards_left(), 6);
        assert_eq!(deck.stop_cards_left(), 1);
    }

    #[test]
    fn draw_two_cards() {
        let cards: Vec<DeckCard> = vec![
            DeckCard::ActionCard(ActionCard::new (ActionType::Mutation)),
            DeckCard::ActionCard(ActionCard::new (ActionType::Stop)),
            DeckCard::BaseCard(BaseCard::new (CardColor::Blue, CardValue::Cytosine)),
            DeckCard::BaseCard(BaseCard::new (CardColor::Blue, CardValue::Adenine)),
            DeckCard::BaseCard(BaseCard::new (CardColor::Blue, CardValue::Adenine)),
            DeckCard::BaseCard(BaseCard::new (CardColor::Green, CardValue::Guanine)),
        ];
        let mut deck = Deck::from_vec(cards);

        let new_cards = deck.draw(2);
        assert_eq!(new_cards.len(), 2);
        assert_eq!(deck.cards_left(), 4);
    }
}