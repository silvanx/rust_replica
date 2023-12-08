pub enum CardColor {
    Blue,
    Red,
    Yellow,
    Green,
}

pub enum CardValue {
    Adenine,
    ThymineUracil,
    Guanine,
    Cytosine,
}

pub enum ActionType {
    Stop,
    Mutation,
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
        while new_cards.len() < how_many {
            let card = self.cards.pop();
            match card {
                Some(card) => new_cards.push(card),
                None => return new_cards,
            }
        }
        new_cards
    }

}
