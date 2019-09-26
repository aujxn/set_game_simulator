/* This module contains structs for representing a card and methods to determine if three cards
 * form a set. It also contains an enum for the result of looking for a set in a hand.
 */

/* State for each characteristic of a card */
#[derive(Copy, Clone, Debug)]
pub enum State {
    Zero,
    One,
    Two,
}

/* A card has 4 characteristics that each have a state */
#[derive(Debug)]
pub struct Card(pub State, pub State, pub State, pub State);

/* Constructor for creating a State of a card characteristic */
impl State {
    pub fn new(state: usize) -> Self {
        match state {
            0 => State::Zero,
            1 => State::One,
            2 => State::Two,
            _ => panic!("Impossible card state!"),
        }
    }
}

/* Reports findings of a single set in a hand */
pub enum Set {
    /* Set was found. Values are indices in hand of cards that complete a set */
    Found(usize, usize, usize),
    /* Set wasn't found */
    NotFound(),
}

/* Function to check if a trio of cards is a set. In the game of
 * set every card has 4 attributes with 3 states. 3 cards make a
 * set if for each attribute the cards are of the same state or all
 * different states. When the states are represented by the numbers
 * 0, 1, and 2 modulo 3 can be used to determine if each attribute
 * passes the set requirements.
 */
pub fn is_set(first: &Card, second: &Card, third: &Card) -> bool {
    check(first.0, second.0, third.0)
        && check(first.1, second.1, third.1)
        && check(first.2, second.2, third.2)
        && check(first.3, second.3, third.3)
}

/* Checks the states for a single characteristic of a card */
fn check(first: State, second: State, third: State) -> bool {
    if (first as i32 + second as i32 + third as i32) % 3 == 0 {
        true
    } else {
        false
    }
}
