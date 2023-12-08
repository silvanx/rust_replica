enum CardColor {
    Blue,
    Red,
    Yellow,
    Green,
}

enum CardValue {
    Adenine,
    ThymineUracil,
    Guanine,
    Cytosine,
}

enum ActionType {
    Stop,
    Mutation,
}

struct BaseCard {
    color: CardColor,
    value: CardValue
}

struct ActionCard {
    action: ActionType,
}

enum DeckCard {
    ActionCard(ActionCard),
    BaseCard(BaseCard),
}

struct Deck {
    cards: Vec<DeckCard>,
    initial_size: usize,
    stop_cards: u8,
}

fn is_stop_card(card: &DeckCard) -> bool {
    match card {
        DeckCard::ActionCard(action_card) => match action_card.action {
            ActionType::Stop => true,
            ActionType::Mutation => false
        },
        DeckCard::BaseCard(_) => false
    }
}

impl Deck {
    fn new() -> Deck {
        Deck {
            cards: vec![],
            initial_size: 0,
            stop_cards: 0,
        }
    }

    fn from_vec(cards: Vec<DeckCard>) -> Deck {
        let size = &cards.len();
        let stop_cards = &cards
            .iter()
            .filter(|c| crate::is_stop_card(c))
            .count();
        Deck {
            cards: cards,
            initial_size: *size,
            stop_cards: *stop_cards as u8,
        }
    }

    fn cards_left(&self) -> usize {
        self.cards.len()
    }
}


fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
mod tests {
    use crate::{ActionCard, ActionType, BaseCard, CardColor, CardValue, Deck, DeckCard};

    #[test]
    fn new_deck_is_empty() {
        let deck = Deck::new();
        assert_eq!(deck.cards_left(), 0);
    }

    #[test]
    fn recognise_stop_cards() {
        let cards: Vec<DeckCard> = vec![
            DeckCard::ActionCard(ActionCard {
                action: ActionType::Mutation,
            }),
            DeckCard::ActionCard(ActionCard {
                action: ActionType::Stop,
            }),
            DeckCard::BaseCard(BaseCard {
                color: CardColor::Blue, value: CardValue::Cytosine,
            }),
        ];
        assert!(!crate::is_stop_card(&cards[0]));
        assert!(crate::is_stop_card(&cards[1]));
        assert!(!crate::is_stop_card(&cards[2]));
    }

    #[test]
    fn deck_from_vec() {
        let cards: Vec<DeckCard> = vec![
            DeckCard::ActionCard(ActionCard {
                action: ActionType::Mutation,
            }),
            DeckCard::ActionCard(ActionCard {
                action: ActionType::Stop,
            }),
            DeckCard::BaseCard(BaseCard {
                color: CardColor::Blue, value: CardValue::Cytosine,
            }),
            DeckCard::BaseCard(BaseCard {
                color: CardColor::Blue, value: CardValue::Adenine,
            }),
            DeckCard::BaseCard(BaseCard {
                color: CardColor::Blue, value: CardValue::Adenine,
            }),
            DeckCard::BaseCard(BaseCard {
                color: CardColor::Green, value: CardValue::Adenine,
            }),
        ];
        let deck = Deck::from_vec(cards);

        assert_eq!(deck.cards_left(), 6);
        assert_eq!(deck.stop_cards, 1);
    }
}