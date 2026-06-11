use std::fmt::format;
use rand::{rng, seq::SliceRandom};
#[derive(Debug)]  // provide extra instructions to the compiler 

struct Deck {
    cards: Vec<String>,

}

impl Deck { // inherent Implemention 
    
    fn new() -> Self{ // Associated fucntion not a method methos would be like --> fn shuffle(&self)
        // List of 'suits' - 'hearts', 'spades'

        let suits = ["Spades", "Diamonds", "Hearts", "Clubs"];

        // List of 'values' - 'ace', 'two', 'three'

        let values = ["Ace", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten", "Jack", "Queen", "King"];

        // empty vector cards

        let mut cards = vec![]; // in Rust its called bindign not varible binding is static by default (immutable) we need to add mut to make the binging mut

        // Double nested for loop to define deck cards
        for s in suits {

            for v in values {

                let card = format!("{} of {}", v, s);

                cards.push(card);

            }
            
        }
        // return can be done as follows 

        // -- Option 1

        // let deck = Deck{ cards : cards};
        // // while the field have identical name value we can can do below
        // //    let deck = Deck{ cards };
        // return deck;

        // -- Option 2

        // return Deck { cards };

        // -- Option 3

        Deck { cards } // no "return" no ";"


    }

    fn shuffle (&mut self) {

        let mut rng = rng();

        self.cards.shuffle(&mut rng);

    }

    fn deal(&mut self, num_cards: usize) -> Vec<String> {

        self.cards.split_off(self.cards.len()- num_cards)

    }

}

fn main() {

    let mut deck = Deck::new();

    deck.shuffle();

    // need to add error handling fro below
    let cards = deck.deal(3);

    println!("Heres your deck: {:#?}", deck);
    
    println!("Heres your hand: {:#?}", cards);

}
