use one_eyed_jack::{Deck, Card};

pub struct BoardState {
    pub face_up_cards: Vec<Card>,
    pub face_down_cards: Vec<Card>
}

impl BoardState {
    pub fn new(face_up_cards: Vec<Card>, face_down_cards: Vec<Card>) -> Self {
        BoardState {
            face_down_cards,
            face_up_cards
        }
    }
}

pub struct Player {
    pub hand: Vec<Card>,
    pub board: BoardState,
}

impl Player {
    pub fn new(hand: Vec<Card>, face_up_cards: Vec<Card>, face_down_cards: Vec<Card>) -> Self {
        Player {
            hand,
            board : BoardState::new(face_up_cards, face_down_cards)
        }
    }
}