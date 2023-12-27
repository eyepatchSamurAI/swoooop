mod player;
use player::Player;

use one_eyed_jack::{Deck, Card};

pub struct Swoooop {
    players: Vec<Player>,
    pile: Vec<Card>,
    discard: Vec<Card>,
}

impl Swoooop {

    pub fn setup(number_of_players: usize) -> Self{
        let number_of_decks: usize = (number_of_players + 3) / 4;
        let mut deck = Deck::init_standard_multiple_decks(number_of_decks);
        deck.shuffle();
        let mut players = Vec::new();
        for _ in 0..number_of_players {
            let hand = deck.draw_x_cards(11);
            let mut face_up_cards = Vec::new(); // : [Card; 4];
            let mut face_down_cards = Vec::new(); //: [Card; 4];
            for _ in 0..4 {
                if let Some(card) = deck.draw() {
                    face_up_cards.push(card);
                }
                if let Some(card) = deck.draw() {
                    face_down_cards.push(card);
                }
            }
            let player = Player::new(hand, face_up_cards, face_down_cards);
            players.push(player);
        }
        let mut pile = Vec::new();
        pile.push(deck.draw().unwrap());

        Swoooop {
            players,
            pile,
            discard: Vec::new()
        }
    }

    pub fn player_won(player: &Player) -> bool {
        player.hand.len() == 0 && player.board.face_up_cards.len() == 0 && player.board.face_down_cards.len() == 0
    }

    pub fn take_turn(&mut self, player: &mut Player) {
        
    }

    pub fn play(&mut self) {
        let mut players_cycle = self.players.iter().cycle();

        loop {
            if let Some(player) = players_cycle.next() {
                if Swoooop::player_won(player) {
                    break;
                }
            }
        }
    }
}

fn main() {
    let deck = Deck::init_standard_multiple_decks(2);
    println!("Hello, world!");
    println!("{:#?}", deck);
}
