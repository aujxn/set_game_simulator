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

 /* A set can be graphically represented as a group of 3 collinear
 * vectors in in 4d space. The line these vectors make falls on
 * an array of 81 hypercubes on a 4-torus. These lines intersect
 * the hypercube based on how many of the attributes are the
 * same/different:
 *
 * "cube" line: 3 same and 1 different
 * "face" line: 2 same and 2 different
 * "edge" line: 1 same and 3 different
 * "vertex" line: all 4 different
 */
#[derive(Clone, Copy, Debug)]
pub enum SetType {
    Cube,
    Face,
    Edge,
    Vertex,
}

/* Constructor for creating a State of a card characteristic */
impl SetType {
    pub fn new(diff: usize) -> Self {
        match diff {
            1 => SetType::Cube,
            2 => SetType::Face,
            3 => SetType::Edge,
            4 => SetType::Vertex,
            _ => panic!("Impossible set type!"),
        }
    }
}

/* A set is 3 indices in the hand */
#[derive(Clone, Copy, Debug)]
pub struct Set {
    pub indices: [usize; 3],
    pub class: SetType,
}

impl Set {
    /* Only need two cards to determine the SetType */
    pub fn new(set: [usize; 3], hand: &Vec<Card>) -> Self {
        let mut diff_count = 0;

        if hand[set[0]].0 as i32 != hand[set[1]].0 as i32 {
            diff_count += 1;
        }

        if hand[set[0]].1 as i32 != hand[set[1]].1 as i32 {
            diff_count += 1
        }

        if hand[set[0]].2 as i32 != hand[set[1]].2 as i32 {
            diff_count += 1
        }

        if hand[set[0]].3 as i32 != hand[set[1]].3 as i32 {
            diff_count += 1
        }

        Set {
            indices: set,
            class: SetType::new(diff_count),
        }
    }
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
    (first as i32 + second as i32 + third as i32) % 3 == 0
}
