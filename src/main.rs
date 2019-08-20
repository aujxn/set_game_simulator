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
struct Set {
    found: bool,  // was there a set in the hand?
    count: usize, // how many cards were in the hand

    /* The indices of the Cards in the hand that make a set (if found) */
    card1: usize,
    card2: usize,
    card3: usize,
}

/* Searches for a set in the hand and returns an object with details
 * about what was found (or not found).
 */
fn find_set_all(hand: &Vec<Card>) -> Set {
    let indices = (0..hand.len()).combinations(3);

    for trio in indices {
        println!("indices: {:?}, {:?}, {:?}", trio[0], trio[1], trio[2]);
        if is_set(&hand[trio[0]], &hand[trio[1]], &hand[trio[2]]) {
            return Set {
                found: true,
                count: hand.len(),
                card1: trio[0],
                card2: trio[1],
                card3: trio[2],
            };
        }
    }

    Set {
        found: false,
        count: hand.len(),
        card1: 0,
        card2: 0,
        card3: 0,
    }
}

fn find_set_part(hand: &Vec<Card>) -> Set {
    for i in hand.len() - 3..hand.len() {
        let indices = (0..hand.len() - 3).combinations(2);
        for duo in indices {
            if is_set(&hand[duo[0]], &hand[duo[1]], &hand[i]) {
                return Set {
                    found: true,
                    count: hand.len(),
                    card1: duo[0],
                    card2: duo[1],
                    card3: i,
                };
            }
        }
    }

    Set {
        found: false,
        count: hand.len(),
        card1: 0,
        card2: 0,
        card3: 0,
    }
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

    /* Each iteration of this loop plays a game. This is where my thread fork should go.
     * Currently a million games takes about 15 seconds (one thread) compiled for release.
     */
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

        /* Randomize the cards, actually should probably thread fork after shuffle */
        deck.shuffle(&mut thread_rng());

        /* Some variables for creating the hand struct */
        let mut hand = vec![]; //cards in the hand

        /* Should be able to remove this variable when I refactor this loop
         * to be its own function. Right now its used to break to game loop.
         */
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

        /* This is the loop that plays one entire game of set. Currently it uses mutable variables
         * created in main but I would like this loop to be run on multiple threads so I plan on
         * figuring out how to use locking data structures to store game information.
         */
        while game {
            if set.found {
                match set.count {
                    12 => set12 += 1,
                    15 => set15 += 1,
                    18 => set18 += 1,
                    _ => (),
                }
            } else {
                match set.count {
                    12 => setless12 += 1,
                    15 => setless15 += 1,
                    18 => setless18 += 1,
                    _ => (),
                }
            }

            /* Add cards to the hand before removing the match
             * because swap_remove() replaces the card with
             * the cards at the end. This ensures that removing
             * a card doesn't change the position of the other
             * cards currently in the hand.
             */
            /* In Set you deal more cards if there is 9 cards
             * or if a set wasn't found in the hand. It is less
             * than 13 here because the set hasn't been removed
             * yet.
             */
            if hand.len() < 13 || !set.found {
                for _i in 0..3 {
                    match deck.pop() {
                        Some(x) => hand.push(x),
                        None => {
                            /* Currently the game ends immediatelly when the cards run out. I might
                             * change this to find all the remaining sets in the hand before
                             * terminating. This way I could even collect some data about the sub
                             * 12 card hands that result at the end of a game.
                             */
                            game = false;

                            /* Exits the while game loop. Eventually change game to a function and
                             * have this return some game data. This loop will be the part that runs
                             * in parallel.
                             */
                            continue;
                        }
                    }
                }
            }

            /* Cards have likely been added to the hand but the set hasn't been removed.
             * This kind of feels like it should be a job for find_set function.
             * Consider restructuring here.
             */
            if set.found {
                hand.swap_remove(set.card1);
                hand.swap_remove(set.card2);
                hand.swap_remove(set.card3);

                /* The bool arg to find_set indicates if cards were added and the
                 * previous hand had no sets. This means the find_set function only
                 * has to check the added cards for sets, preventing duplicate work
                 */
                set = find_set_all(&hand);
            } else {
                set = find_set_part(&hand);
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
