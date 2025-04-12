use log::{Level, debug, error, info, log_enabled, trace};
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashMap;
use std::{io, sync::mpsc::TrySendError};

fn main() {
    env_logger::init();
    let mut game: Game = Game::new();
    debug!("Adding players");
    game.add_player(Player::new("michael".to_string()));
    game.add_player(Player::new("computer".to_string()));
    debug!("Config ready, starting game loop");
    game.start_game();
    while game.is_running {
        let mut selection = String::new();
        io::stdin()
            .read_line(&mut selection)
            .expect("failed to read line");
        trace!("User input: {selection}");
        match selection.as_str() {
            "h\n" => {
                trace!("Hit triggered");
                game.hit();
                if game.players[game.active_player].state == PlayerState::Busted {
                    game.bust();
                }
            }
            "s\n" => {
                trace!("Stay triggered");
                game.stay();
            }
            "q\n" => {
                trace!("Quit triggered");
                game.is_running = false;
            }
            "round\n" => {
                trace!("Round triggered");
                println!("Round:{round}", round = game.round);
            }
            "hand\n" => {
                trace!("Hand triggered");
                game.players[game.active_player].print_cards();
            }
            _ => {
                trace!("Invalid input");
                selection = "invalid_input".to_string();
            }
        }
        if game.is_end_of_round() && selection != "invalid_input" {
            trace!("End of round triggered");
            game.next_round();
            println!("Round Number:{round}", round = game.round);
        } else {
            trace!("Going to next player");
            game.next_player();
        }
    }
    trace!("EOS");
}

pub struct Deck {
    cards: Vec<usize>,
}
impl Default for Deck {
    fn default() -> Self {
        Self::new()
    }
}
impl Deck {
    pub fn new() -> Self {
        let mut cards: Vec<usize> = Vec::new();
        for i in 1..13 {
            trace!("adding {i} card {i} times");
            cards.extend(vec![i; i]);
        }
        info!("Adding specialty cards");
        cards.push(0);
        cards.extend([
            SpecialCards::Plus2 as usize,
            SpecialCards::Plus4 as usize,
            SpecialCards::Plus6 as usize,
            SpecialCards::Plus8 as usize,
            SpecialCards::Plus10 as usize,
            SpecialCards::Times2 as usize,
            SpecialCards::Freeze as usize,
            SpecialCards::Freeze as usize,
            SpecialCards::Freeze as usize,
        ]);
        cards.extend([SpecialCards::FlipThree as usize; 3]);
        cards.extend([SpecialCards::SecondChance as usize; 3]);
        let mut rng = thread_rng();
        cards.shuffle(&mut rng);
        Self { cards }
    }

    pub fn count(&self) -> usize {
        let count: usize = self.cards.len() as usize;
        trace!("deck count checked: {count}");
        count
    }

    fn draw_card(&mut self) -> usize {
        debug!("drawing card from deck");
        self.cards
            .pop()
            .expect("tried to draw from the deck while empty")
    }
}

pub struct Player {
    cards: HashMap<usize, usize>,
    score: u16,
    total: u16,
    cards_in_hand: usize,
    state: PlayerState,
    name: String,
}
impl Default for Player {
    fn default() -> Self {
        Self::new("Manual Player".to_string())
    }
}
impl Player {
    pub fn new(player_name: String) -> Self {
        Self {
            cards: HashMap::new(),
            score: 0,
            total: 0,
            cards_in_hand: 0,
            state: PlayerState::Playing,
            name: player_name,
        }
    }

    pub fn add_to_hand(&mut self, card: usize) -> () {
        if card < SpecialCards::Plus2 as usize {
            self.total += card as u16;
        }
        trace!("Card drawn: [{card}]");
        let count = self.cards.entry(card).or_insert(0);
        *count += 1;
        self.cards_in_hand += 1;
        if *count > 1 as usize && card < SpecialCards::Plus2 as usize {
            self.state = PlayerState::Busted;
            println!("BUSTED");
        }
        if self.cards_in_hand >= 7 || self.total + self.score >= 200 {
            self.state = PlayerState::Win7;
        }
        trace!("Card copies: {count}");
    }

    pub fn get_hand(&mut self) -> &mut HashMap<usize, usize> {
        &mut self.cards
    }

    pub fn stay(&mut self) {
        self.score += self.total;
        let score = self.score;
        self.state = PlayerState::Stay;
        println!("Player stayed with {score} points ");
    }

    pub fn bust(&mut self) {
        trace!("player resetting hand total");
        self.total = 0;
    }

    pub fn is_still_in_round(&self) -> bool {
        return self.state != PlayerState::Busted && self.state != PlayerState::Stay;
    }

    pub fn print_cards(&self) {
        debug!("Printing cards");
        let total = self.total;
        trace!("Hand total: {total}");
        let mut hand: String = String::new();
        for (&card, &count) in self.cards.iter() {
            hand.push_str(format!("[{card_name}]", card_name = &(card.to_string())).as_str());
            for _i in 0..count - 1 {
                hand.push_str(format!("[{card_name}]", card_name = &(card.to_string())).as_str());
            }
        }
        println!("{name}{hand}", name = self.name);
        debug!("Card total:{total}");
    }
}
pub struct Game {
    players: Vec<Player>,
    active_player: usize,
    deck: Deck,
    discard: Vec<usize>,
    is_running: bool,
    round: usize,
}

impl Game {
    pub fn new() -> Self {
        Self {
            players: Vec::new(),
            deck: Deck::new(),
            discard: Vec::new(),
            is_running: false,
            active_player: 0,
            round: 0,
        }
    }
    pub fn add_player(&mut self, new_player: Player) {
        trace!("Adding player");
        self.players.push(new_player);
    }

    pub fn stay(&mut self) {
        trace!("player stays");
        self.active_player_discard_hand();
        self.players[self.active_player].stay();
    }

    pub fn next_round(&mut self) {
        self.round += 1;
        for player in self.players.iter_mut() {
            player.state = PlayerState::Playing;
        }
    }

    fn next_player(&mut self) {
        let mut i: usize = self.active_player;
        trace!(
            "Active player index: {idx} of {player_count}",
            idx = i,
            player_count = self.players.iter().count()
        );
        if i >= self.players.iter().count() - 1 {
            trace!("Active player is last player in list - going to beginning");
            i = 0;
        } else {
            i += 1;
            trace!("Going to next player idx")
        }
        while i <= self.players.iter().count() - 1 {
            trace!("Next player idx: {i}");
            if self.players[i].is_still_in_round() {
                self.active_player = i;
                trace!(
                    "Active player: {player}",
                    player = self.players[self.active_player].name
                );
                return;
            }
            i += 1;
        }
    }

    pub fn is_end_of_round(&self) -> bool {
        let mut player_still_in: bool;
        for player in self.players.iter() {
            player_still_in = player.is_still_in_round();
            trace!(
                "Player {name} still in: {player_still_in}",
                name = player.name
            );
            if player_still_in {
                return false;
            }
        }
        return true;
    }

    fn active_player_discard_hand(&mut self) {
        let cards = self.players[self.active_player].get_hand();
        for (&card, &count) in cards.iter() {
            for _i in 0..count {
                trace!("moving {card} to discard");
                self.discard.push(card);
            }
        }
        trace!("clearing player hand");
        cards.clear();
    }
    pub fn bust(&mut self) {
        trace!("Player busted");
        self.active_player_discard_hand();
        self.players[self.active_player].bust();
    }

    pub fn hit(&mut self) {
        trace!("hit");
        self.players[self.active_player].add_to_hand(self.deck.draw_card());
        self.players[self.active_player].print_cards();
    }

    pub fn start_game(&mut self) {
        self.is_running = true;
        let starting_player = &self.players[self.active_player].name;
        println!("| ==================== Welcome to Flip 7! ==================== |");
        println!("Player {starting_player} is going first!");
        println!("Press h to hit, s to stay");
        self.round = 1;
    }
}
#[derive(PartialEq, Eq)]
enum PlayerState {
    Playing,
    Busted,
    Win7,
    Stay,
}
#[derive(PartialEq, Eq)]
enum SpecialCards {
    Plus2 = 13,
    Plus4 = 14,
    Plus6 = 15,
    Plus8 = 16,
    Plus10 = 17,
    Times2 = 18,
    Freeze = 19,
    FlipThree = 20,
    SecondChance = 21,
}
