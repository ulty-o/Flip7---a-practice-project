use std::io;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashMap;

fn main() {
    println!("Press h to hit, s to stay");
    let mut deck:Deck = Deck::new();
    let mut player:Player = Player::new();
    let mut play_game = true;
    while play_game {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("failed to read");
        if guess == "h\n"{
            player.add_to_hand(deck.draw_card());
            player.print_cards();
        }else if guess == "s\n" {
           
        
        }else {
            play_game = false;
        }
        
    }
}

pub struct Deck{
    cards:Vec<u8>
}
impl Default for Deck{
    fn default() -> Self {
        Self::new()
    }
}
impl Deck{

    pub fn new() -> Self{
        let mut cards:Vec<u8> = Vec::new();
        for i in 0..13{
            for _j in 0..i{
                cards.push(i); 
            }
        }
        cards.push(0);
        cards.push(SpecialCards::Plus2 as u8);
        cards.push(SpecialCards::Plus4 as u8);
        cards.push(SpecialCards::Plus6 as u8);
        cards.push(SpecialCards::Plus8 as u8);
        cards.push(SpecialCards::Plus10 as u8);
        cards.push(SpecialCards::Times2 as u8);
        cards.push(SpecialCards::Freeze as u8);
        cards.push(SpecialCards::Freeze as u8);
        cards.push(SpecialCards::Freeze as u8);
        cards.push(SpecialCards::FlipThree as u8);
        cards.push(SpecialCards::FlipThree as u8);
        cards.push(SpecialCards::FlipThree as u8);
        cards.push(SpecialCards::SecondChance as u8);
        cards.push(SpecialCards::SecondChance as u8);
        cards.push(SpecialCards::SecondChance as u8);
        let mut rng = thread_rng();
        cards.shuffle(&mut rng);
        Self {cards}
    }
  
  pub fn count(&self) -> u8{
    self.cards.len() as u8
  }

  pub fn draw_card(&mut self) -> u8{
        self.cards.pop().expect("tried to draw from the deck while empty")
  }
}

pub struct Player{
    cards:HashMap<u8,u8>,
    score:u16,
    total:u16,
    state:PlayerState
}
impl Player{
    pub fn new() -> Self{
        Self{
            cards:HashMap::new(),
            score:0,
            total:0,
            state:PlayerState::Playing
        }
    }      

    pub fn add_to_hand(&mut self, card:u8) -> (){
        self.total+= card as u16;
        println!("Card drawn: {card}");
        let count = self.cards.entry(card).or_insert(0);
        *count += 1;
        if *count > 1 as u8 && card < SpecialCards::Plus2 as u8 {
            self.state = PlayerState::Busted;
            println!("BUSTED");
        }
        println!("Card copies: {count}");
    }

    pub fn discard_hand(&mut self){
        
    }

    pub fn stay(&mut self){
        self.score+= self.total;
        let score = self.score; 
        self.state = PlayerState::Stay;
        println!("Player stayed with {score} points ")
    }

    pub fn print_cards(&self){
        let total = self.total;
        println!("Card total:{total}");
    }
}
pub struct Game{
    players:Vec<Player>,
    deck:Deck
}

impl Game{
    pub fn new() -> Self{
        Self{ 
            players:Vec::new(),
            deck:Deck::new()
        }
    }
    pub fn add_player(&mut self, new_player:Player){
        self.players.push(new_player);
    }

}
enum PlayerState{
    Playing,
    Busted,
    Stay
}

enum SpecialCards{
    Plus2 = 13,
    Plus4 = 14,
    Plus6 = 15,
    Plus8 = 16,
    Plus10 = 17,
    Times2 = 18,
    Freeze = 19,
    FlipThree = 20,
    SecondChance = 21
    
}

