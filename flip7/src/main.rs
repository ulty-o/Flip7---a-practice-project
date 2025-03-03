fn main() {
    println!("Hello, world!");
    let mut deck:Deck = Deck::new();
    let card = deck.draw_card();
    println!("Card Drawn: {card}");
    
}

pub struct Deck{
    cards:Vec<u8>
}

impl Deck{
    
    fn default() -> Self {
        Self::new()
    }
    

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
        Self {cards}
    }
  
  pub fn count(&self) -> u8{
    self.cards.len() as u8
  }

  pub fn draw_card(&mut self) -> u8{
        self.cards.pop().expect("tried to draw from the deck while empty")
  }
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