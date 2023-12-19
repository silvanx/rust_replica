use std::fmt;
use std::fmt::Formatter;

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum CardColor {
    Blue,
    Red,
    Yellow,
    Green,
}

impl fmt::Display for CardColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let color = match self {
            CardColor::Blue => "Blue",
            CardColor::Green => "Green",
            CardColor::Yellow => "Yellow",
            CardColor::Red => "Red",
        };
        write!(f, "{}", color)
    }
}

#[derive(Copy, Clone)]
pub enum CardValue {
    Adenine,
    ThymineUracil,
    Guanine,
    Cytosine,
}

impl fmt::Display for CardValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let value = match self {
            CardValue::Cytosine => "C",
            CardValue::Adenine => "A",
            CardValue::ThymineUracil => "T/U",
            CardValue::Guanine => "G",
        };
        write!(f, "{}", value)
    }
}

#[derive(Copy, Clone)]
pub enum ActionType {
    Stop,
    Mutation,
}

impl fmt::Display for ActionType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let action_type = match self {
            ActionType::Stop => "Stop",
            ActionType::Mutation => "Mutation",
        };
        write!(f, "{}", action_type)
    }
}

impl ActionCard {
    pub fn new(action: ActionType) -> ActionCard {
        ActionCard { action }
    }
}

#[derive(Copy, Clone)]
pub struct BaseCard {
    pub color: CardColor,
    pub value: CardValue
}

impl BaseCard {
    pub fn new(color: CardColor, value: CardValue) -> BaseCard {
        BaseCard { color, value }
    }
}

#[derive(Copy, Clone)]
pub struct ActionCard {
    action: ActionType,
}

#[derive(Copy, Clone)]
pub enum DeckCard {
    ActionCard(ActionCard),
    BaseCard(BaseCard),
}

impl fmt::Display for DeckCard {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let t = match self {
            DeckCard::ActionCard(c) => format!("Action {}", c.action),
            DeckCard::BaseCard(c) => format!("Base {} {}", c.color, c.value),
        };
        write!(f, "{}", t)
    }
}

impl DeckCard {
    pub fn is_stop_card(&self) -> bool {
        match self {
            DeckCard::ActionCard(action_card) => match action_card.action {
                ActionType::Stop => true,
                ActionType::Mutation => false
            },
            DeckCard::BaseCard(_) => false
        }
    }

    pub fn is_base_card(&self) -> bool {
        if let DeckCard::BaseCard(_) = self {
            true
        } else {
            false
        }
    }
}

pub struct Deck {
    cards: Vec<DeckCard>,
    initial_size: usize,
    stop_cards: u8,
}

impl Deck {
    pub fn new() -> Deck {
        Deck {
            cards: vec![],
            initial_size: 0,
            stop_cards: 0,
        }
    }

    pub fn from_vec(cards: Vec<DeckCard>) -> Deck {
        let size = &cards.len();
        let stop_cards = &cards
            .iter()
            .filter(|c| c.is_stop_card())
            .count();
        Deck {
            cards: cards,
            initial_size: *size,
            stop_cards: *stop_cards as u8,
        }
    }

    pub fn cards_left(&self) -> u8 {
        self.cards.len() as u8
    }

    pub fn stop_cards_left(&self) -> u8 {
        self.stop_cards
    }

    pub fn draw(&mut self, how_many: u8) -> Vec<DeckCard> {
        let mut new_cards= vec![];
        let how_many = how_many as usize;
        let mut stop_cards_drawn: u8 = 0;
        while new_cards.len() < how_many {
            let card = self.cards.pop();
            match card {
                Some(c) => {
                    if c.is_stop_card() {
                        stop_cards_drawn += 1;
                    }
                    new_cards.push(c);
                },
                None => break
            }
        }
        self.stop_cards -= stop_cards_drawn;
        new_cards
    }

    pub fn replace(&mut self, card: DeckCard) {
        if card.is_stop_card() {
            self.stop_cards += 1;
        }
        self.cards.push(card);
    }

    pub fn shuffle(&mut self) {
        use rand::seq::SliceRandom;
        let mut rng = rand::thread_rng();
        self.cards.shuffle(&mut rng);
    }

}

#[cfg(test)]
mod tests {
    use crate::deck::{ActionCard, ActionType, BaseCard, CardColor, CardValue, Deck, DeckCard};

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
        assert!(!cards[0].is_stop_card());
        assert!(cards[1].is_stop_card());
        assert!(!cards[2].is_stop_card());
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
            .filter(|c| c.is_stop_card())
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
