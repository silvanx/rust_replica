use crate::deck::DeckCard;
use crate::player::Player;

const STOP_CARDS_IN_DECK: u8 = 10;
const MAX_CELLS_2_PLAYERS: u8 = 6;
const MAX_CELLS_3_PLAYERS: u8 = 7;
const MAX_CELLS_4_PLAYERS: u8 = 8;

pub struct CardPair {
    pub top: DeckCard,
    pub bottom: DeckCard,
}

pub struct Game {
    num_players: u8,
    players: Vec<Player>,
    stopped_cells: u8,
    stop_cards_played: u8,
    deck: Vec<DeckCard>,
    revealed_cards: Vec<DeckCard>,
    dna: Vec<CardPair>,
}

impl Game {
    fn is_over(&self) -> bool {
        if self.num_players == 2 && self.stopped_cells >= MAX_CELLS_2_PLAYERS {
            true
        } else if self.num_players == 3 && self.stopped_cells >= MAX_CELLS_3_PLAYERS {
            true
        } else if self.num_players == 4 && self.stopped_cells >= MAX_CELLS_4_PLAYERS {
            true
        } else {
            self.deck.is_empty() || self.stop_cards_played >= STOP_CARDS_IN_DECK
        }
    }
}

#[cfg(test)]
mod test {
    use crate::deck::{ActionType, CardColor, DeckCard};
    use crate::player::Player;
    use crate::game::{Game, MAX_CELLS_2_PLAYERS, MAX_CELLS_3_PLAYERS, MAX_CELLS_4_PLAYERS, STOP_CARDS_IN_DECK};

    #[test]
    fn game_not_over() {
        let game = Game {
            num_players: 2,
            players: vec![
                Player::new(CardColor::Red),
                Player::new(CardColor::Blue),
            ],
            stopped_cells: 0,
            stop_cards_played: 0,
            deck: vec![
                DeckCard::ActionCard(crate::deck::ActionCard::new (ActionType::Mutation))
            ],
            revealed_cards: vec![],
            dna: vec![],
        };
        assert!(!game.is_over())
    }

    #[test]
    fn empty_deck_game_over() {
        let game = Game {
            num_players: 2,
            players: vec![
                Player::new(CardColor::Red),
                Player::new(CardColor::Blue),
            ],
            stopped_cells: 2,
            stop_cards_played: 2,
            deck: vec![],
            revealed_cards: vec![],
            dna: vec![],
        };
        assert!(game.is_over())
    }

    #[test]
    fn played_all_stop_cards_game_over() {
        let game = Game {
            num_players: 2,
            players: vec![
                Player::new(CardColor::Red),
                Player::new(CardColor::Blue),
            ],
            stopped_cells: 2,
            stop_cards_played: STOP_CARDS_IN_DECK,
            deck: vec![
                DeckCard::ActionCard(crate::deck::ActionCard::new (ActionType::Mutation))
            ],
            revealed_cards: vec![],
            dna: vec![],
        };
        assert!(game.is_over())
    }

    #[test]
    fn max_cells_stopped_2_players_game_over() {
        // not a valid game state
        let game = Game {
            num_players: 2,
            players: vec![
                Player::new(CardColor::Red),
                Player::new(CardColor::Blue),
            ],
            stopped_cells: MAX_CELLS_2_PLAYERS,
            stop_cards_played: 0,
            deck: vec![
                DeckCard::ActionCard(crate::deck::ActionCard::new (ActionType::Mutation))
            ],
            revealed_cards: vec![],
            dna: vec![],
        };
        assert!(game.is_over())
    }

    #[test]
    fn max_cells_stopped_3_players_game_over() {
        // not a valid game state
        let game = Game {
            num_players: 3,
            players: vec![
                Player::new(CardColor::Red),
                Player::new(CardColor::Blue),
                Player::new(CardColor::Yellow),
            ],
            stopped_cells: MAX_CELLS_3_PLAYERS,
            stop_cards_played: 0,
            deck: vec![
                DeckCard::ActionCard(crate::deck::ActionCard::new (ActionType::Mutation))
            ],
            revealed_cards: vec![],
            dna: vec![],
        };
        assert!(game.is_over())
    }

    #[test]
    fn max_cells_stopped_4_players_game_over() {
        // not a valid game state
        let game = Game {
            num_players: 4,
            players: vec![
                Player::new(CardColor::Red),
                Player::new(CardColor::Blue),
                Player::new(CardColor::Yellow),
                Player::new(CardColor::Green),
            ],
            stopped_cells: MAX_CELLS_4_PLAYERS,
            stop_cards_played: 0,
            deck: vec![
                DeckCard::ActionCard(crate::deck::ActionCard::new (ActionType::Mutation))
            ],
            revealed_cards: vec![],
            dna: vec![],
        };
        assert!(game.is_over())
    }
}