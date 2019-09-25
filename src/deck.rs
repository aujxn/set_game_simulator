/* This module builds a deck of cards and shuffles them so games of set can
 * be simulated.
 */
use rand::seq::SliceRandom;
use rand::thread_rng;
use crate::set::{Card, State};

/* Builds the deck for a game of set */
pub fn shuffle_cards() -> Vec<Card> {
    let mut deck: Vec<Card> = vec![];

    /* Builds all the cards in the deck */
    for i in 0..3 {
        for j in 0..3 {
            for k in 0..3 {
                for l in 0..3 {
                    deck.push(Card(
                        State::new(i),
                        State::new(j),
                        State::new(k),
                        State::new(l),
                    ));
                }
            }
        }
    }

    /* Randomize the cards */
    deck.shuffle(&mut thread_rng());

    deck
}

