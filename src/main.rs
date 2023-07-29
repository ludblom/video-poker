mod bank;

use core::fmt;
use std::fmt::write;

use bank::Bank;
use deckofcards::{Deck, Card};

fn main() {
    let bank: Bank = Bank::new();
    loop {
        let mut deck: Deck = Deck::new();

        let hand: Vec<Card> = deck.deal(5);

        println!("Hold cards:")
    }
}
