/* Author: Austen Nelson
 * A Set game simulator
 *
 * 8/19/2019
 */

use itertools::Itertools;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::ops::AddAssign;

/* Each card has 4 characteristics that can have 3 states */
struct Card(usize, usize, usize, usize);

/* Reports how many of each hand were encountered in the game */
struct GameResult(i64, i64, i64, i64, i64, i64, i64);

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

/* Reports findings of a single hand */
enum Set {
    /* First 3 are the indices of the cards in the hand */
    Found(usize, usize, usize), // was there a set in the hand?
    NotFound(),
}

/* Searches entire hand for sets */
fn find_set_all(hand: &Vec<Card>) -> Set {
    let indices = (0..hand.len()).combinations(3);

    for trio in indices {
        if is_set(&hand[trio[0]], &hand[trio[1]], &hand[trio[2]]) {
            return Set::Found(trio[0], trio[1], trio[2]);
        }
    }
    Set::NotFound()
}

/* Searches for sets but only for sets that include at least one of
 * the last three cards in the hand
 */
fn find_set_part(hand: &Vec<Card>) -> Set {
    for i in hand.len() - 3..hand.len() {
        let indices = (0..hand.len() - 3).combinations(2);
        for duo in indices {
            if is_set(&hand[duo[0]], &hand[duo[1]], &hand[i]) {
                return Set::Found(duo[0], duo[1], i);
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
    if (first.0 + second.0 + third.0) % 3 != 0 {
        return false;
    } else if (first.1 + second.1 + third.1) % 3 != 0 {
        return false;
    } else if (first.2 + second.2 + third.2) % 3 != 0 {
        return false;
    } else if (first.3 + second.3 + third.3) % 3 != 0 {
        return false;
    }
    true
}

/* Plays an entire game of set and returns some information */
fn play_game() -> GameResult {
    let mut result = GameResult(0, 0, 0, 0, 0, 0, 0);

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
                        result.1 += 1;
                    }
                    15 => result.3 += 1,
                    18 => result.5 += 1,
                    21 => result.6 += 1,
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
                    12 => result.0 += 1,
                    15 => result.2 += 1,
                    18 => result.4 += 1,
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
    let mut results = GameResult(0, 0, 0, 0, 0, 0, 0);

    /* Each iteration of this loop plays a game. */
    for _x in 0..10_000 {
        results += play_game();
    }

    /* Report the findings about the games */
    println!("\nsetless 12's {:?}", results.0);
    println!("set 12's {:?}", results.1);
    println!(
        "proportion of 12's {:?}\n",
        results.0 as f64 / results.1 as f64
    );

    println!("setless 15's {:?}", results.2);
    println!("set 15's {:?}", results.3);
    println!(
        "proportion of 15's {:?}\n",
        results.2 as f64 / results.3 as f64
    );

    println!("setless 18's {:?}", results.4);
    println!("set 18's {:?}", results.5);
    println!(
        "proportion of 18's {:?}\n",
        results.4 as f64 / results.5 as f64
    );

    println!("set 21's {:?}", results.6);
}
