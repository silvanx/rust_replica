use std::fmt;
use std::fmt::Formatter;

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

pub struct BaseCard {
    color: CardColor,
    value: CardValue
}

impl BaseCard {
    pub fn new(color: CardColor, value: CardValue) -> BaseCard {
        BaseCard { color, value }
    }
}

pub struct ActionCard {
    action: ActionType,
}

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

pub fn is_stop_card(card: &DeckCard) -> bool {
    match card {
        DeckCard::ActionCard(action_card) => match action_card.action {
            ActionType::Stop => true,
            ActionType::Mutation => false
        },
        DeckCard::BaseCard(_) => false
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
            .filter(|c| is_stop_card(c))
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
                    if is_stop_card(&c) {
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
        if is_stop_card(&card) {
            self.stop_cards += 1;
        }
        self.cards.push(card);
    }

    pub fn shuffle(&mut self) {
        // todo
    }

}
