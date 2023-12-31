use crate::deck::{ActionCard, ActionType, BaseCard, CardColor, CardValue, DeckCard};

mod deck;
mod player;
mod game;

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
