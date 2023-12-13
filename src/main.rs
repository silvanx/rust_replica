use crate::deck::{ActionCard, ActionType, BaseCard, CardColor, CardValue, DeckCard};

mod deck;

fn main() {
    let cards: Vec<DeckCard> = vec![
        DeckCard::ActionCard(ActionCard::new (ActionType::Mutation)),
        DeckCard::ActionCard(ActionCard::new (ActionType::Mutation)),
        DeckCard::ActionCard(ActionCard::new (ActionType::Mutation)),
        DeckCard::ActionCard(ActionCard::new (ActionType::Mutation)),
        DeckCard::ActionCard(ActionCard::new (ActionType::Mutation)),
        DeckCard::ActionCard(ActionCard::new (ActionType::Mutation)),
        DeckCard::ActionCard(ActionCard::new (ActionType::Mutation)),
        DeckCard::ActionCard(ActionCard::new (ActionType::Mutation)),
        DeckCard::ActionCard(ActionCard::new (ActionType::Mutation)),
        DeckCard::ActionCard(ActionCard::new (ActionType::Mutation)),
        DeckCard::ActionCard(ActionCard::new (ActionType::Mutation)),
        DeckCard::ActionCard(ActionCard::new (ActionType::Mutation)),
        DeckCard::ActionCard(ActionCard::new (ActionType::Mutation)),
        DeckCard::ActionCard(ActionCard::new (ActionType::Mutation)),
        DeckCard::ActionCard(ActionCard::new (ActionType::Stop)),
        DeckCard::ActionCard(ActionCard::new (ActionType::Stop)),
        DeckCard::ActionCard(ActionCard::new (ActionType::Stop)),
        DeckCard::ActionCard(ActionCard::new (ActionType::Stop)),
        DeckCard::ActionCard(ActionCard::new (ActionType::Stop)),
        DeckCard::ActionCard(ActionCard::new (ActionType::Stop)),
        DeckCard::ActionCard(ActionCard::new (ActionType::Stop)),
        DeckCard::ActionCard(ActionCard::new (ActionType::Stop)),
        DeckCard::ActionCard(ActionCard::new (ActionType::Stop)),
        DeckCard::ActionCard(ActionCard::new (ActionType::Stop)),
        DeckCard::BaseCard(BaseCard::new (CardColor::Blue, CardValue::Adenine)),
        DeckCard::BaseCard(BaseCard::new (CardColor::Blue, CardValue::Cytosine)),
        DeckCard::BaseCard(BaseCard::new (CardColor::Blue, CardValue::ThymineUracil)),
        DeckCard::BaseCard(BaseCard::new (CardColor::Blue, CardValue::Guanine)),
        DeckCard::BaseCard(BaseCard::new (CardColor::Blue, CardValue::Adenine)),
        DeckCard::BaseCard(BaseCard::new (CardColor::Blue, CardValue::Cytosine)),
        DeckCard::BaseCard(BaseCard::new (CardColor::Blue, CardValue::ThymineUracil)),
        DeckCard::BaseCard(BaseCard::new (CardColor::Blue, CardValue::Guanine)),
        DeckCard::BaseCard(BaseCard::new (CardColor::Blue, CardValue::Adenine)),
        DeckCard::BaseCard(BaseCard::new (CardColor::Blue, CardValue::Cytosine)),
        DeckCard::BaseCard(BaseCard::new (CardColor::Blue, CardValue::ThymineUracil)),
        DeckCard::BaseCard(BaseCard::new (CardColor::Blue, CardValue::Guanine)),
        DeckCard::BaseCard(BaseCard::new (CardColor::Blue, CardValue::Adenine)),
        DeckCard::BaseCard(BaseCard::new (CardColor::Blue, CardValue::Cytosine)),
        DeckCard::BaseCard(BaseCard::new (CardColor::Blue, CardValue::ThymineUracil)),
        DeckCard::BaseCard(BaseCard::new (CardColor::Blue, CardValue::Guanine)),
        DeckCard::BaseCard(BaseCard::new (CardColor::Blue, CardValue::Adenine)),
        DeckCard::BaseCard(BaseCard::new (CardColor::Blue, CardValue::Cytosine)),
        DeckCard::BaseCard(BaseCard::new (CardColor::Blue, CardValue::ThymineUracil)),
        DeckCard::BaseCard(BaseCard::new (CardColor::Blue, CardValue::Guanine)),
        DeckCard::BaseCard(BaseCard::new (CardColor::Blue, CardValue::Adenine)),
        DeckCard::BaseCard(BaseCard::new (CardColor::Blue, CardValue::Cytosine)),
        DeckCard::BaseCard(BaseCard::new (CardColor::Blue, CardValue::ThymineUracil)),
        DeckCard::BaseCard(BaseCard::new (CardColor::Blue, CardValue::Guanine)),
        DeckCard::BaseCard(BaseCard::new (CardColor::Green, CardValue::Adenine)),
        DeckCard::BaseCard(BaseCard::new (CardColor::Green, CardValue::Cytosine)),
        DeckCard::BaseCard(BaseCard::new (CardColor::Green, CardValue::ThymineUracil)),
        DeckCard::BaseCard(BaseCard::new (CardColor::Green, CardValue::Guanine)),
        DeckCard::BaseCard(BaseCard::new (CardColor::Green, CardValue::Adenine)),
        DeckCard::BaseCard(BaseCard::new (CardColor::Green, CardValue::Cytosine)),
        DeckCard::BaseCard(BaseCard::new (CardColor::Green, CardValue::ThymineUracil)),
        DeckCard::BaseCard(BaseCard::new (CardColor::Green, CardValue::Guanine)),
        DeckCard::BaseCard(BaseCard::new (CardColor::Green, CardValue::Adenine)),
        DeckCard::BaseCard(BaseCard::new (CardColor::Green, CardValue::Cytosine)),
        DeckCard::BaseCard(BaseCard::new (CardColor::Green, CardValue::ThymineUracil)),
        DeckCard::BaseCard(BaseCard::new (CardColor::Green, CardValue::Guanine)),
        DeckCard::BaseCard(BaseCard::new (CardColor::Green, CardValue::Adenine)),
        DeckCard::BaseCard(BaseCard::new (CardColor::Green, CardValue::Cytosine)),
        DeckCard::BaseCard(BaseCard::new (CardColor::Green, CardValue::ThymineUracil)),
        DeckCard::BaseCard(BaseCard::new (CardColor::Green, CardValue::Guanine)),
        DeckCard::BaseCard(BaseCard::new (CardColor::Green, CardValue::Adenine)),
        DeckCard::BaseCard(BaseCard::new (CardColor::Green, CardValue::Cytosine)),
        DeckCard::BaseCard(BaseCard::new (CardColor::Green, CardValue::ThymineUracil)),
        DeckCard::BaseCard(BaseCard::new (CardColor::Green, CardValue::Guanine)),
        DeckCard::BaseCard(BaseCard::new (CardColor::Green, CardValue::Adenine)),
        DeckCard::BaseCard(BaseCard::new (CardColor::Green, CardValue::Cytosine)),
        DeckCard::BaseCard(BaseCard::new (CardColor::Green, CardValue::ThymineUracil)),
        DeckCard::BaseCard(BaseCard::new (CardColor::Green, CardValue::Guanine)),
        DeckCard::BaseCard(BaseCard::new (CardColor::Red, CardValue::Adenine)),
        DeckCard::BaseCard(BaseCard::new (CardColor::Red, CardValue::Cytosine)),
        DeckCard::BaseCard(BaseCard::new (CardColor::Red, CardValue::ThymineUracil)),
        DeckCard::BaseCard(BaseCard::new (CardColor::Red, CardValue::Guanine)),
        DeckCard::BaseCard(BaseCard::new (CardColor::Red, CardValue::Adenine)),
        DeckCard::BaseCard(BaseCard::new (CardColor::Red, CardValue::Cytosine)),
        DeckCard::BaseCard(BaseCard::new (CardColor::Red, CardValue::ThymineUracil)),
        DeckCard::BaseCard(BaseCard::new (CardColor::Red, CardValue::Guanine)),
        DeckCard::BaseCard(BaseCard::new (CardColor::Red, CardValue::Adenine)),
        DeckCard::BaseCard(BaseCard::new (CardColor::Red, CardValue::Cytosine)),
        DeckCard::BaseCard(BaseCard::new (CardColor::Red, CardValue::ThymineUracil)),
        DeckCard::BaseCard(BaseCard::new (CardColor::Red, CardValue::Guanine)),
        DeckCard::BaseCard(BaseCard::new (CardColor::Red, CardValue::Adenine)),
        DeckCard::BaseCard(BaseCard::new (CardColor::Red, CardValue::Cytosine)),
        DeckCard::BaseCard(BaseCard::new (CardColor::Red, CardValue::ThymineUracil)),
        DeckCard::BaseCard(BaseCard::new (CardColor::Red, CardValue::Guanine)),
        DeckCard::BaseCard(BaseCard::new (CardColor::Red, CardValue::Adenine)),
        DeckCard::BaseCard(BaseCard::new (CardColor::Red, CardValue::Cytosine)),
        DeckCard::BaseCard(BaseCard::new (CardColor::Red, CardValue::ThymineUracil)),
        DeckCard::BaseCard(BaseCard::new (CardColor::Red, CardValue::Guanine)),
        DeckCard::BaseCard(BaseCard::new (CardColor::Red, CardValue::Adenine)),
        DeckCard::BaseCard(BaseCard::new (CardColor::Red, CardValue::Cytosine)),
        DeckCard::BaseCard(BaseCard::new (CardColor::Red, CardValue::ThymineUracil)),
        DeckCard::BaseCard(BaseCard::new (CardColor::Red, CardValue::Guanine)),
        DeckCard::BaseCard(BaseCard::new (CardColor::Yellow, CardValue::Adenine)),
        DeckCard::BaseCard(BaseCard::new (CardColor::Yellow, CardValue::Cytosine)),
        DeckCard::BaseCard(BaseCard::new (CardColor::Yellow, CardValue::ThymineUracil)),
        DeckCard::BaseCard(BaseCard::new (CardColor::Yellow, CardValue::Guanine)),
        DeckCard::BaseCard(BaseCard::new (CardColor::Yellow, CardValue::Adenine)),
        DeckCard::BaseCard(BaseCard::new (CardColor::Yellow, CardValue::Cytosine)),
        DeckCard::BaseCard(BaseCard::new (CardColor::Yellow, CardValue::ThymineUracil)),
        DeckCard::BaseCard(BaseCard::new (CardColor::Yellow, CardValue::Guanine)),
        DeckCard::BaseCard(BaseCard::new (CardColor::Yellow, CardValue::Adenine)),
        DeckCard::BaseCard(BaseCard::new (CardColor::Yellow, CardValue::Cytosine)),
        DeckCard::BaseCard(BaseCard::new (CardColor::Yellow, CardValue::ThymineUracil)),
        DeckCard::BaseCard(BaseCard::new (CardColor::Yellow, CardValue::Guanine)),
        DeckCard::BaseCard(BaseCard::new (CardColor::Yellow, CardValue::Adenine)),
        DeckCard::BaseCard(BaseCard::new (CardColor::Yellow, CardValue::Cytosine)),
        DeckCard::BaseCard(BaseCard::new (CardColor::Yellow, CardValue::ThymineUracil)),
        DeckCard::BaseCard(BaseCard::new (CardColor::Yellow, CardValue::Guanine)),
        DeckCard::BaseCard(BaseCard::new (CardColor::Yellow, CardValue::Adenine)),
        DeckCard::BaseCard(BaseCard::new (CardColor::Yellow, CardValue::Cytosine)),
        DeckCard::BaseCard(BaseCard::new (CardColor::Yellow, CardValue::ThymineUracil)),
        DeckCard::BaseCard(BaseCard::new (CardColor::Yellow, CardValue::Guanine)),
        DeckCard::BaseCard(BaseCard::new (CardColor::Yellow, CardValue::Adenine)),
        DeckCard::BaseCard(BaseCard::new (CardColor::Yellow, CardValue::Cytosine)),
        DeckCard::BaseCard(BaseCard::new (CardColor::Yellow, CardValue::ThymineUracil)),
        DeckCard::BaseCard(BaseCard::new (CardColor::Yellow, CardValue::Guanine)),
    ];
    let mut deck = deck::Deck::from_vec(cards);
    deck.shuffle();
    println!("Created deck with {} cards", deck.cards_left());
    let cards = deck.draw(3);
    println!("Top three cards:");
    for card in cards.iter() {
        println!("{}", card);
    }
}


#[cfg(test)]
mod tests {
    use std::fmt::format;
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
        let mut deck = create_example_deck();
        let new_cards = deck.draw(2);
        assert_eq!(new_cards.len(), 2);
        assert_eq!(deck.cards_left(), 4);
    }

    #[test]
    fn draw_update_stop_cards() {
        let mut deck = create_example_deck();
        let new_cards = deck.draw(2);
        let stop_cards_in_hand = new_cards
            .iter()
            .filter(|c| is_stop_card(c))
            .count() as u8;
        assert_eq!(deck.stop_cards_left(), 0);
        assert_eq!(deck.cards_left(), 4);
        assert_eq!(stop_cards_in_hand, 1);
    }

    #[test]
    fn draw_too_many_cards() {
        let mut deck = create_example_deck();
        let new_cards = deck.draw(deck.cards_left() + 1);
        assert_eq!(new_cards.len(), 6);
        assert_eq!(deck.cards_left(), 0);
    }

    #[test]
    fn replace_card_into_deck() {
        let mut deck = create_example_deck();
        let new_card = DeckCard::ActionCard(ActionCard::new(ActionType::Mutation));
        deck.replace(new_card);
        assert_eq!(deck.cards_left(), 7);
        assert_eq!(deck.stop_cards_left(), 1);
        if let Some(top_card) = deck.draw(1).get(0) {
            let card_description = format!("{}", top_card);
            assert_eq!(String::from("Action Mutation"), card_description);
        } else {
            panic!("Could not draw replaced card!");
        }
    }

    #[test]
    fn replace_stop_card_into_deck() {
        let mut deck = create_example_deck();
        let new_card = DeckCard::ActionCard(ActionCard::new(ActionType::Stop));
        deck.replace(new_card);
        assert_eq!(deck.stop_cards_left(), 2);
    }

    #[test]
    fn shuffle_deck() {
        let mut deck = create_example_deck();
        let new_card = DeckCard::BaseCard(BaseCard::new (CardColor::Yellow, CardValue::Adenine));
        deck.replace(new_card);
        deck.shuffle();
        // TODO: Test the randomness more robustly; shuffling the deck once could still result in the last card staying
        //       where it was...
        let card_description = format!("{}", deck.draw(1).get(0).unwrap());
        assert_eq!(deck.cards_left(), 6);
        assert_ne!(card_description, "Base Yellow A");
    }

    #[test]
    fn shuffle_two_decks() {
        // TODO: Write a more robust test (this will sometimes fail)
        let mut deck1 = create_example_deck();
        let mut deck2 = create_example_deck();
        deck1.shuffle();
        deck2.shuffle();
        let mut all_cards_identical = true;
        while let card = deck1.draw(1).get(0) {
            if card.is_some() &&
                (format!("{}", card.unwrap()) != format!("{}", deck2.draw(1).get(0).unwrap())) {
                all_cards_identical = false;
            } else {
                break;
            }
        }
        if all_cards_identical {
            panic!("Two decks shuffled separately result in identical order")
        }
    }
}
