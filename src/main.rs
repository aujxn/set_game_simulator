/* Author: Austen Nelson
 * A Set game simulator
 *
 * 8/19/2019
 */

use chrono::prelude::*;
use clap::App;
use itertools::Itertools;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rayon::prelude::*;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::ops::Add;
use std::path::Path;

/* State for each characteristic of a card */
#[derive(Copy, Clone)]
enum State {
    Zero,
    One,
    Two,
}

/* A card has 4 characteristics that each have a state */
struct Card(State, State, State, State);

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

/* Function to check if a trio of cards is a set. In the game of
 * set every card has 4 attributes with 3 states. 3 cards make a
 * set if for each attribute the cards are of the same state or all
 * different states. When the states are represented by the numbers
 * 0, 1, and 2 modulo 3 can be used to determine if each attribute
 * passes the set requirements.
 */
fn is_set(first: &Card, second: &Card, third: &Card) -> bool {
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

/* Reports how many of each hand were encountered in what part of the game
 * See default implementation for description
 */
#[derive(Clone)]
struct GameResult {
    sets: Vec<Vec<i64>>,
    setless: Vec<Vec<i64>>,
}

/* The sets vector contain 4 vectors. Index 0 to 3 have data for 12, 15, 18, and 21
 * card hands respectively. The index of these 4 vectors represents the number of times
 * cards have been dealt from the deck. The setless vector is the same, except without
 * a vector for 21 card hands. This is because a 21 card hand must contain a set.
 */
impl Default for GameResult {
    fn default() -> Self {
        GameResult {
            sets: vec![vec![0; 24], vec![0; 24], vec![0; 24], vec![0; 24]],
            setless: vec![vec![0; 24], vec![0; 24], vec![0; 24]],
        }
    }
}

/* Simply combines GameResult values into a single result */
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
        }
    }
}

/* Reports findings of a single hand */
enum Set {
    /* Set was found. Values are indices in hand of cards that complete a set */
    Found(usize, usize, usize),
    /* Set wasn't found */
    NotFound(),
}

/* Searches entire hand for sets */
fn find_set_all(hand: &[Card]) -> Set {
    if let Some((x, y, z)) = (0..hand.len())
        .tuple_combinations::<(_, _, _)>()
        .find(|(x, y, z)| is_set(&hand[*x], &hand[*y], &hand[*z]))
    {
        return Set::Found(x, y, z);
    }

    Set::NotFound()
}

/* Searches for sets but only for sets that include at least one of
 * the last three cards in the hand
 */
fn find_set_part(hand: &[Card]) -> Set {
    let len = hand.len();
    let split = len - 3;

    /* All combinations with 1 of the new cards */
    for x in split..len {
        if let Some((y, z)) = (0..split)
            .tuple_combinations()
            .find(|(y, z)| is_set(&hand[x], &hand[*y], &hand[*z]))
        {
            return Set::Found(x, y, z);
        }
    }

    /* All combinations with 2 of the new cards */
    for x in 0..split {
        if let Some((y, z)) = (split..len)
            .tuple_combinations()
            .find(|(y, z)| is_set(&hand[x], &hand[*y], &hand[*z]))
        {
            return Set::Found(x, y, z);
        }
    }

    /* Check if the three new cards make a set */
    if is_set(&hand[split], &hand[split + 1], &hand[split + 2]) {
        return Set::Found(split, split + 1, split + 2);
    }

    /* Otherwise, there are no sets in the hand */
    Set::NotFound()
}

/* Builds the deck for a game of set */
fn shuffle_cards() -> Vec<Card> {
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

/* Plays an entire game of set and returns some information */
fn play_game() -> GameResult {
    let mut result = GameResult::default();

    /* This is the data structure for the cards that haven't been dealt to the hand */
    let mut deck = shuffle_cards();

    let mut hand = vec![]; //cards in the hand

    /* Get the first 12 cards for the hand from the deck */
    for _i in 0..12 {
        match deck.pop() {
            Some(x) => hand.push(x),
            None => unreachable!(), //there will always be 81 cards in the deck to start
        }
    }

    /* Find the first set, primes the game loop */
    let mut set = find_set_all(&hand);

    /* This is the loop that plays one entire game of set. */
    loop {
        match set {
            Set::Found(x, y, z) => {
                /* Add to the count that a set was found */
                match hand.len() {
                    12 => {
                        /* 23 - (deck.len() / 3) calculates how many times cards have been dealt */
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

/* Report the findings about the games */
fn report(results: &GameResult, games: i64) {
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
}

/* Exports results to an external data file */
fn write_results(results: &GameResult) {
    /* Serialize the data into a string so it can be published to an external file */
    let serialized: String = itertools::join(
        results
            .setless
            .iter()
            .map(|x| itertools::join(x, " "))
            .collect::<Vec<String>>(),
        "\n",
    ) + "\n"
        + &itertools::join(
            results
                .sets
                .iter()
                .map(|x| itertools::join(x, " "))
                .collect::<Vec<String>>(),
            "\n",
        );

    /* Create the output filename using the current date/time */
    let date: DateTime<Local> = Local::now();
    let path_name =
        "python/data/".to_string() + &date.format("%Y-%m-%d_%H:%M:%S").to_string() + ".txt";

    /* Create the path to write file to */
    let path = Path::new(&path_name);
    let display = path.display();

    /* Make the file */
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why.description()),
        Ok(file) => file,
    };

    /* Write the data to the file */
    match file.write_all(serialized.as_bytes()) {
        Err(why) => panic!("couldn't create {}: {}", display, why.description()),
        Ok(_) => log::info!("wrote data to {}", display),
    }
}

/* CLI crate macros */
#[macro_use]
extern crate clap;

fn main() {
    /* Initialization */
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "full");
    env_logger::init();

    /* CLI configuration and parsing */
    let yml = load_yaml!("cli.yml");
    let args = App::from_yaml(yml).get_matches();
    let games: i64 = if let Some(games) = args.value_of("games") {
        games.parse().unwrap()
    } else {
        panic!("number of games not provided");
    };

    /* plays the game and sums all the results in parallel */
    let results: GameResult = (0..games)
        .into_par_iter()
        /* The reason fold and reduce are needed here is because of the way Rayon's parallel
         * iterator works. Fold allows the identity function (its first argument) to return a type
         * that is different from the type of thing we are iterating over (integers in this case).
         * Fold results in an iterator over GameResults that were created by breaking the original
         * iterator into the pieces that were distributed over threads. This is why reduce is
         * required to get the final sum.
         */
        .fold(|| GameResult::default(), |acc, _| acc + play_game())
        .reduce(|| GameResult::default(), |acc, x| acc + x);

    /* Finalize */
    report(&results, games);
    write_results(&results);
}
