use std::error::Error;
use std::fmt;
use std::fmt::Formatter;
use rand::thread_rng;
use rand::seq::SliceRandom;
use crate::deck::{CardColor, Deck, DeckCard};
use crate::player::Player;

const STOP_CARDS_IN_DECK: u8 = 10;
const MAX_CELLS_2_PLAYERS: u8 = 6;
const MAX_CELLS_3_PLAYERS: u8 = 7;
const MAX_CELLS_4_PLAYERS: u8 = 8;
const MIN_PLAYERS: u8 = 2;
const MAX_PLAYERS: u8 = 4;

pub struct CardPair {
    pub top: DeckCard,
    pub bottom: DeckCard,
}

pub struct Game {
    pub num_players: u8,
    pub players: Vec<Player>,
    stopped_cells: u8,
    stop_cards_played: u8,
    pub deck: Deck,
    pub revealed_cards: Vec<DeckCard>,
    pub dna: Vec<CardPair>,
}

pub struct WrongNumberPlayersError;

impl fmt::Display for WrongNumberPlayersError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Can't create game with this many players!")
    }
}

impl fmt::Debug for WrongNumberPlayersError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!())
    }
}

impl Game {
    pub fn new(
        num_players: u8,
        deck: Deck,
    ) -> Result<Game, WrongNumberPlayersError> {
        if num_players < MIN_PLAYERS || num_players > MAX_PLAYERS {
            return Err(WrongNumberPlayersError)
        }
        let mut rng = thread_rng();
        let players = vec![
            CardColor::Green, CardColor::Yellow, CardColor::Blue, CardColor::Red
        ]
            .choose_multiple(&mut rng, num_players as usize)
            .map(|color| Player::new(*color)).collect();
        let game = Game {
            num_players: num_players,
            players: players,
            stopped_cells: 0,
            stop_cards_played: 0,
            deck: deck,
            revealed_cards: vec![],
            dna: vec![],
        };
        Ok(game)
    }

    pub fn is_over(&self) -> bool {
        if self.num_players == 2 && self.stopped_cells >= MAX_CELLS_2_PLAYERS {
            true
        } else if self.num_players == 3 && self.stopped_cells >= MAX_CELLS_3_PLAYERS {
            true
        } else if self.num_players == 4 && self.stopped_cells >= MAX_CELLS_4_PLAYERS {
            true
        } else {
            self.deck.cards_left() < 1 || self.stop_cards_played >= STOP_CARDS_IN_DECK
        }
    }
}

#[cfg(test)]
mod test {
    use crate::deck::{ActionType, CardColor, Deck, DeckCard};
    use crate::player::Player;
    use crate::game::{Game, MAX_CELLS_2_PLAYERS, MAX_CELLS_3_PLAYERS, MAX_CELLS_4_PLAYERS, STOP_CARDS_IN_DECK, MIN_PLAYERS, MAX_PLAYERS};

    #[test]
    fn game_new() {
        let num_players: u8 = 2;
        let game = Game::new(num_players, Deck::from_vec(vec![]));
        assert!(game.is_ok());
        let game = game.unwrap();
        assert_eq!(game.num_players, 2);
        assert_eq!(game.players.len(), 2);
        assert_ne!(game.players.get(0).unwrap().color,
                   game.players.get(1).unwrap().color);
    }

    #[test]
    fn game_new_wrong_number_of_players() {
        assert!(Game::new(MIN_PLAYERS - 1, Deck::from_vec(vec![])).is_err());
        assert!(Game::new(MAX_PLAYERS + 1, Deck::from_vec(vec![])).is_err());
    }

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
            deck: Deck::from_vec(vec![
                DeckCard::ActionCard(crate::deck::ActionCard::new (ActionType::Mutation))
            ]),
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
            deck: Deck::from_vec(vec![]),
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
            deck: Deck::from_vec(vec![
                DeckCard::ActionCard(crate::deck::ActionCard::new (ActionType::Mutation))
            ]),
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
            deck: Deck::from_vec(vec![
                DeckCard::ActionCard(crate::deck::ActionCard::new (ActionType::Mutation))
            ]),
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
            deck: Deck::from_vec(vec![
                DeckCard::ActionCard(crate::deck::ActionCard::new (ActionType::Mutation))
            ]),
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
            deck: Deck::from_vec(vec![
                DeckCard::ActionCard(crate::deck::ActionCard::new (ActionType::Mutation))
            ]),
            revealed_cards: vec![],
            dna: vec![],
        };
        assert!(game.is_over())
    }
}