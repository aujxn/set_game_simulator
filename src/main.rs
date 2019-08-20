/* Author: Austen Nelson
 * A Set game simulator
 *
 * 8/19/2019
 */

use itertools::Itertools;
use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Debug)]
/* Names tuple for a single card. I feel like this should use
 * an enum since the card values can only be 0, 1, or 2 but
 * I was getting compiler errors because I don't know how to rust.
 */
struct Card(usize, usize, usize, usize);

#[derive(Debug)]
/* Used to report the findings of playing a single hand.
 * Returned by the Hand method find_set()
 */
enum Set {
    Found(usize, usize, usize, usize), // was there a set in the hand?
    NotFound(usize),
}

/* Searches for a set in the hand and returns an object with details
 * about what was found (or not found).
 */
fn find_set_all(hand: &Vec<Card>) -> Set {
    let indices = (0..hand.len()).combinations(3);

    for trio in indices {
        if is_set(&hand[trio[0]], &hand[trio[1]], &hand[trio[2]]) {
            return Set::Found(trio[0], trio[1], trio[2], hand.len());
        }
    }
    Set::NotFound(hand.len())
}

fn find_set_part(hand: &Vec<Card>) -> Set {
    for i in hand.len() - 3..hand.len() {
        let indices = (0..hand.len() - 3).combinations(2);
        for duo in indices {
            if is_set(&hand[duo[0]], &hand[duo[1]], &hand[i]) {
                return Set::Found(duo[0], duo[1], i, hand.len());
            }
        }
    }
    Set::NotFound(hand.len())
}

/* Function to check if a trio of cards is a set. In the game of
 * set every card has 4 attributes with 3 classes. 3 cards make a
 * set if for each attribute the cards are of the same class or all
 * different class. When the classes are represented by the numbers
 * 0, 1, and 2 this means modulo 3 can be used to determine if each
 * attribute passes the set requirements.
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

fn main() {
    /* These store information about how many of each hand type were encountered by the simulation.
     * Setless# means there was no sets in the hand and set# means a set was found. The number is
     * how many cards were in the hand.
     */
    let mut setless12 = 0;
    let mut set12 = 0;
    let mut setless15 = 0;
    let mut set15 = 0;
    let mut setless18 = 0;
    let mut set18 = 0;

    /* Each iteration of this loop plays a game. */
    for _x in 0..1_000_000 {
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

        let mut game = true; //indicates the game isn't complete

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
        while game {
            match set {
                Set::Found(x, y, z, l) => {
                    /* Add to the count that a set was found */
                    match l {
                        12 => set12 += 1,
                        15 => set15 += 1,
                        18 => set18 += 1,
                        _ => unreachable!(),
                    }

                    /* If the deck has 12 cards add 3 cards to the hand */
                    if l == 12 {
                        for _i in 0..3 {
                            match deck.pop() {
                                Some(x) => hand.push(x),
                                None => {
                                    game = false;
                                    continue;
                                }
                            }
                        }
                    }

                    /* remove the cards from the hand that made a set */
                    hand.swap_remove(x);
                    hand.swap_remove(y);
                    hand.swap_remove(z);

                    /* search for the next set in all the cards */
                    set = find_set_all(&hand);
                }
                Set::NotFound(l) => {
                    /* update the count of hand with no sets */
                    match l {
                        12 => setless12 += 1,
                        15 => setless15 += 1,
                        18 => setless18 += 1,
                        _ => unreachable!(),
                    }

                    /* add three more cards to the hand */
                    for _i in 0..3 {
                        match deck.pop() {
                            Some(x) => hand.push(x),
                            None => {
                                game = false;
                                continue;
                            }
                        }
                    }

                    /* find the next set, but only look at combinations that use new cards */
                    set = find_set_part(&hand);
                }
            }
        }
    }

    /* Report the findings about the games */
    println!("setless 12's {:?}", setless12);
    println!("set 12's {:?}", set12);
    println!("proportion of 12's {:?}\n", setless12 as f64 / set12 as f64);

    println!("setless 15's {:?}", setless15);
    println!("set 15's {:?}", set15);
    println!("proportion of 15's {:?}\n", setless15 as f64 / set15 as f64);

    println!("setless 18's {:?}", setless18);
    println!("set 18's {:?}", set18);
    println!("proportion of 18's {:?}", setless18 as f64 / set18 as f64);
}
