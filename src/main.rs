/* Author: Austen Nelson
 * A Set game simulator
 *
 * 8/19/2019
 */

use itertools::Itertools;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rayon::prelude::*;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::ops::Add;
use std::path::Path;
use chrono::prelude::*;

/* State for each characteristic of a card */
#[derive(Copy, Clone)]
enum State {
    Zero,
    One,
    Two,
}

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

fn check(first: State, second: State, third: State) -> bool {
    if (first as i32 + second as i32 + third as i32) % 3 == 0 {
        return true;
    } else {
        false
    }
}

/* Function to check if a trio of cards is a set. In the game of
 * set every card has 4 attributes with 3 classes. 3 cards make a
 * set if for each attribute the cards are of the same class or all
 * different class. When the classes are represented by the numbers
 * 0, 1, and 2 modulo 3 can be used to determine if each attribute
 * passes the set requirements.
 */
fn is_set(first: &Card, second: &Card, third: &Card) -> bool {
    check(first.0, second.0, third.0)
        && check(first.1, second.1, third.1)
        && check(first.2, second.2, third.2)
        && check(first.3, second.3, third.3)
}

impl Add for State {
    type Output = i32;

    fn add(self, other: Self) -> i32 {
        self as i32 + other as i32
    }
}

struct Card(State, State, State, State);

/* Reports how many of each hand were encountered in what part of the game */
#[derive(Clone)]
struct GameResult {
    sets: Vec<Vec<i64>>,
    setless: Vec<Vec<i64>>,
}

impl Default for GameResult {
    fn default() -> Self {
        GameResult {
            sets: vec![vec![0; 24], vec![0; 24], vec![0; 24], vec![0; 24]],
            setless: vec![vec![0; 24], vec![0; 24], vec![0; 24]],
        }
    }
}

impl Add for GameResult {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            sets: self
                .sets
                .iter()
                .zip(other.sets.iter())
                .map(|x| x.0.iter().zip(x.1.iter()).map(|y| y.0 + y.1).collect())
                .collect(),
            setless: self
                .setless
                .iter()
                .zip(other.setless.iter())
                .map(|x| x.0.iter().zip(x.1.iter()).map(|y| y.0 + y.1).collect())
                .collect(),
            /*
            set12: self
                .set12
                .iter()
                .zip(other.set12.iter())
                .map(|x| x.0 + x.1)
                .collect(),
            set15: self
                .set15
                .iter()
                .zip(other.set15.iter())
                .map(|x| x.0 + x.1)
                .collect(),
            set18: self
                .set18
                .iter()
                .zip(other.set18.iter())
                .map(|x| x.0 + x.1)
                .collect(),
            set21: self
                .set21
                .iter()
                .zip(other.set21.iter())
                .map(|x| x.0 + x.1)
                .collect(),
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
            */
        }
    }
}

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
                        result.sets[0][23 - (deck.len() / 3)] += 1;
                        /* If the deck has 12 cards add 3 cards to the hand */
                        for _i in 0..3 {
                            match deck.pop() {
                                Some(x) => hand.push(x),
                                None => return result,
                            }
                        }
                    }
                    15 => result.sets[1][23 - (deck.len() / 3)] += 1,
                    18 => result.sets[2][23 - (deck.len() / 3)] += 1,
                    21 => result.sets[3][23 - (deck.len() / 3)] += 1,
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
                    12 => result.setless[0][23 - (deck.len() / 3)] += 1,
                    15 => result.setless[1][23 - (deck.len() / 3)] += 1,
                    18 => result.setless[2][23 - (deck.len() / 3)] += 1,
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

    let games = 1_000;

    /* plays the game and sums all the results in parallel */
    let results: GameResult = (0..games)
        .into_par_iter()
        .fold(|| GameResult::default(), |acc, _| acc + play_game())
        .reduce(|| GameResult::default(), |acc, x| acc + x);

    /* Report the findings about the games */
    log::info!("After {:?} games of simulated... \n\n", games);

    let setless = results
        .setless
        .iter()
        .map(|x| x.iter().sum())
        .collect::<Vec<i64>>();
    let sets = results
        .sets
        .iter()
        .map(|x| x.iter().sum())
        .collect::<Vec<i64>>();

    log::info!("12 card hands with no sets: {:?}", setless[0]);
    log::info!("12 card hands where set was found: {:?}", sets[0]);
    log::info!(
        "proportion of 12s w/out sets: {:.3}%\n",
        100.0 * setless[0] as f64 / sets[0] as f64
    );

    log::info!("15 card hands with no sets: {:?}", setless[1]);
    log::info!("15 card hands where set was found: {:?}", sets[1]);
    log::info!(
        "proportion of 15s w/out sets: {:.3}%\n",
        100.0 * setless[1] as f64 / sets[1] as f64
    );

    log::info!("18 card hands with no sets: {:?}", setless[2]);
    log::info!("18 card hands where set was found: {:?}", sets[2]);
    log::info!(
        "proportion of 18s w/out sets: {:.3}%\n",
        100.0 * setless[2] as f64 / sets[2] as f64
    );

    log::info!(
        "21 cards hands encountered: {:?}\n ({:?} games per 21 card hand)",
        sets[3],
        if sets[3] != 0 {
            games as i64 / sets[3]
        } else {
            0
        }
    );

    let serialized: String =
        itertools::join(
            results
                .setless
                .iter()
                .map(|x| itertools::join(x, " "))
                .collect::<Vec<String>>(),
            "\n") + "\n" + &
        itertools::join(
            results
                .sets
                .iter()
                .map(|x| itertools::join(x, " "))
                .collect::<Vec<String>>(),
            "\n");

    //print!("{:?}", serialized);

    /*
    let serialized: String = format!(
        "{:?} {:?}",
        results
            .setless
            .iter()
            .flat_map(|x| format!(
                "{:?}\n",
                x.iter()
                    .flat_map(|y| format!("{:?} ", y.to_string()).chars())
                    .collect::<String>()
            )
            .chars())
            .collect::<String>(),
        results
            .sets
            .iter()
            .flat_map(|x| format!(
                "{:?}\n",
                x.iter()
                    .flat_map(|y| format!("{:?} ", y.to_string()).chars())
                    .collect::<String>()
            )
            .chars())
            .collect::<String>()
    );
    */

    /*
    let setless12: String = results
        .setless12
        .iter()
        .map(|x| format!("{:?} ", x.to_string()))
        .collect();
    let set12: String = results
        .set12
        .iter()
        .map(|x| format!("{:?} ", x.to_string()))
        .collect();
    let setless15: String = results
        .setless15
        .iter()
        .map(|x| format!("{:?} ", x.to_string()))
        .collect();
    let set15: String = results
        .set15
        .iter()
        .map(|x| format!("{:?} ", x.to_string()))
        .collect();
    let setless18: String = results
        .setless18
        .iter()
        .map(|x| format!("{:?} ", x.to_string()))
        .collect();
    let set18: String = results
        .set18
        .iter()
        .map(|x| format!("{:?} ", x.to_string()))
        .collect();
    */

    let date: DateTime<Local> = Local::now();
    let path_name = "python/".to_string() + &date.format("%Y-%m-%d_%H:%M:%S").to_string() + ".txt";

    let path = Path::new(&path_name);
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why.description()),
        Ok(file) => file,
    };

    match file.write_all(serialized.as_bytes()) {
        Err(why) => panic!("couldn't create {}: {}", display, why.description()),
        Ok(_) => log::info!("wrote data to {}", display),
    }
}
