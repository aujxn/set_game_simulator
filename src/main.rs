/* Author: Austen Nelson
 * A Set game simulator
 *
 * 8/19/2019
 */

use itertools::Itertools;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rayon::prelude::*;
use std::ops::Add;

/* Each card has 4 characteristics that can have 3 states */
struct Card(usize, usize, usize, usize);

/* Reports how many of each hand were encountered in what part of the game */
#[derive(Clone)]
struct GameResult {
    set12: i64,
    set15: i64,
    set18: i64,
    set21: i64,
    setless12: Vec<i64>,
    setless15: Vec<i64>,
    setless18: Vec<i64>,
}

impl Default for GameResult {
    /* The first vec is all the 12's, second is 15's, third is 18's, and last is 21's
     * The tuples inside the vecs are the setless and with set counts
     * The index of the tuples is the number of remaining cards in the deck divided by 3
     */
    fn default() -> Self {
        GameResult {
            set12: 0,
            set15: 0,
            set18: 0,
            set21: 0,
            setless12: vec![0; 24],
            setless15: vec![0; 24],
            setless18: vec![0; 24],
        }
    }
}

impl Add for GameResult {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            set12: self.set12 + other.set12,
            set15: self.set15 + other.set15,
            set18: self.set18 + other.set18,
            set21: self.set21 + other.set21,
            setless12: self
                .setless12
                .iter()
                .zip(other.setless12.iter())
                .map(|x| x.0 + x.1)
                .collect(),
            setless15: self
                .setless15
                .iter()
                .zip(other.setless15.iter())
                .map(|x| x.0 + x.1)
                .collect(),
            setless18: self
                .setless18
                .iter()
                .zip(other.setless18.iter())
                .map(|x| x.0 + x.1)
                .collect(),
        }
    }
}
/*
impl AddAssign for GameResult {
    fn add_assign(&mut self, other: Self) {
        self.0 = self.0 + other.0;
        self.1 = self.1 + other.1;
        self.2 = self.2 + other.2;
        self.3 = self.3 + other.3;
        self.4 = self.4 + other.4;
        self.5 = self.5 + other.5;
        self.6 = self.6 + other.6;
    }
}
*/
/* Reports findings of a single hand */
enum Set {
    /* First 3 are the indices of the cards in the hand */
    Found(usize, usize, usize), // was there a set in the hand?
    NotFound(),
}

/* Searches entire hand for sets */
fn find_set_all(hand: &[Card]) -> Set {
    let indices = (0..hand.len()).tuple_combinations::<(_, _, _)>();

    for (x, y, z) in indices {
        if is_set(&hand[x], &hand[y], &hand[z]) {
            return Set::Found(x, y, z);
        }
    }
    Set::NotFound()
}

/* Searches for sets but only for sets that include at least one of
 * the last three cards in the hand
 */
fn find_set_part(hand: &[Card]) -> Set {
    for i in hand.len() - 3..hand.len() {
        let indices = (0..hand.len() - 3).tuple_combinations();
        for (x, y) in indices {
            if is_set(&hand[x], &hand[y], &hand[i]) {
                return Set::Found(x, y, i);
            }
        }
    }
    Set::NotFound()
}

/* Function to check if a trio of cards is a set. In the game of
 * set every card has 4 attributes with 3 classes. 3 cards make a
 * set if for each attribute the cards are of the same class or all
 * different class. When the classes are represented by the numbers
 * 0, 1, and 2 modulo 3 can be used to determine if each attribute
 * passes the set requirements.
 */
fn is_set(first: &Card, second: &Card, third: &Card) -> bool {
    if (first.0 + second.0 + third.0) % 3 == 0
        && (first.1 + second.1 + third.1) % 3 == 0
        && (first.2 + second.2 + third.2) % 3 == 0
        && (first.3 + second.3 + third.3) % 3 == 0
    {
        return true;
    }
    false
}

/* Plays an entire game of set and returns some information */
fn play_game() -> GameResult {
    let mut result = GameResult::default();

    /* This is the data structure for the full deck */
    let mut deck: Vec<Card> = vec![];

    /* Builds all the cards in the deck */
    for i in 0..3 {
        for j in 0..3 {
            for k in 0..3 {
                for l in 0..3 {
                    deck.push(Card(i, j, k, l));
                }
            }
        }
    }

    /* Randomize the cards */
    deck.shuffle(&mut thread_rng());

    let mut hand = vec![]; //cards in the hand

    /* Get the first 12 cards for the hand from the deck */
    for _i in 0..12 {
        match deck.pop() {
            Some(x) => hand.push(x),
            None => unreachable!(), //there will always be 81 cards in the deck to start
        }
    }

    /* Find the first set, primes the game loop for how it is currently structured. */
    let mut set = find_set_all(&hand);

    /* This is the loop that plays one entire game of set. */
    loop {
        match set {
            Set::Found(x, y, z) => {
                /* Add to the count that a set was found */
                match hand.len() {
                    12 => {
                        /* If the deck has 12 cards add 3 cards to the hand */
                        for _i in 0..3 {
                            match deck.pop() {
                                Some(x) => hand.push(x),
                                None => return result,
                            }
                        }
                        result.set12 += 1;
                    }
                    15 => result.set15 += 1,
                    18 => result.set18 += 1,
                    21 => result.set21 += 1,
                    _ => {
                        println!("Unreachable hand size: {:?}", hand.len());
                        unreachable!();
                    }
                }

                /* Cards have to be removed in reverse order to prevent mixing up indices */
                let mut remove = vec![x, y, z];
                remove.sort();

                /* remove the cards from the hand that made a set */
                hand.swap_remove(remove[2]);
                hand.swap_remove(remove[1]);
                hand.swap_remove(remove[0]);

                /* search for the next set in all the cards */
                set = find_set_all(&hand);
            }
            Set::NotFound() => {
                /* update the count of hand with no sets */
                match hand.len() {
                    12 => result.setless12[deck.len() / 3] += 1,
                    15 => result.setless15[deck.len() / 3] += 1,
                    18 => result.setless18[deck.len() / 3] += 1,
                    _ => {
                        println!("Unreachable hand size: {:?}", hand.len());
                        unreachable!();
                    }
                }

                /* add three more cards to the hand - because no set was found */
                for _i in 0..3 {
                    match deck.pop() {
                        Some(x) => hand.push(x),
                        None => return result,
                    }
                }

                /* find the next set, but only look at combinations that use new cards */
                set = find_set_part(&hand);
            }
        }
    }
}

fn main() {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let games = 1_000_000;

    /* plays the game and sums all the results in parallel */
    let results: GameResult = (0..games)
        .into_par_iter()
        .fold(|| GameResult::default(), |acc, _| acc + play_game())
        .reduce(|| GameResult::default(), |acc, x| acc + x);

    /* Report the findings about the games */
    log::info!("After {:?} games of simulated... \n\n", games);

    let setless12: i64 = results.setless12.iter().sum();
    let setless15: i64 = results.setless15.iter().sum();
    let setless18: i64 = results.setless18.iter().sum();

    log::info!("12 card hands with no sets: {:?}", setless12);
    log::info!("12 card hands where set was found: {:?}", results.set12);
    log::info!(
        "proportion of 12s w/out sets: {:.3}%\n",
        100.0 * setless12 as f64 / results.set12 as f64
    );

    log::info!("15 card hands with no sets: {:?}", setless15);
    log::info!("15 card hands where set was found: {:?}", results.set15);
    log::info!(
        "proportion of 15s w/out sets: {:.3}%\n",
        100.0 * setless15 as f64 / results.set15 as f64
    );

    log::info!("18 card hands with no sets: {:?}", setless18);
    log::info!("18 card hands where set was found: {:?}", results.set18);
    log::info!(
        "proportion of 18s w/out sets: {:.3}%\n",
        100.0 * setless18 as f64 / results.set18 as f64
    );

    log::info!(
        "21 cards hands encountered: {:?}\n ({:?} games per 21 card hand)",
        results.set21,
        if results.set21 != 0 {
            games as i64 / results.set21
        } else {
            0
        }
    );
}
