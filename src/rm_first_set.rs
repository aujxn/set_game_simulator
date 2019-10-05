/* This module plays Set by searching through the hand and removing the first set
 * that it encounters. It reports data by recording a count for each attempt at
 * searching for a set. For each attempt it records how far into the game it is,
 * how many cards are currently in the hand, and if a set was found or not. This
 * data is then written to an external file.
 */

use crate::{deck, set, set::Card};
use itertools::Itertools;
use rayon::prelude::*;
use std::{error::Error, fs, fs::File, io::prelude::*, path::Path};

/* Reports how many of each hand were encountered in what
 * part of the game. GameResult is a 3 dimensional array.
 *
 * The first dimension is representive of the game progress.
 * The index of this dimension is the number of times cards
 * have been dealt. The game starts at 0 deals (the initial
 * 12 card hand) and the last hand is played after the 23rd
 * deal.
 *
 * The second dimension is the hand size. Index 0 is 12
 * cards, 1 is 15 cards, and 2 is 18 cards.
 *
 * The third dimension is the sets vs setless counts. Index
 * 0 is the count of hands with sets and index 1 in setless.
 *
 * example: if 2500 is at index [9][1][1] then 2500
 * hands after 9 deals with 15 cards were found with no sets
 */
#[derive(Clone, Copy)]
struct GameResult {
    data: [[[i64; 2]; 3]; 24],
}

/* default is all 0 values */
impl Default for GameResult {
    fn default() -> Self {
        GameResult {
            data: [[[0; 2]; 3]; 24],
        }
    }
}

/* Pretty indexing for GameResults */
const SETS: usize = 0;
const SETLESS: usize = 1;
const SIZE12: usize = 0;
const SIZE15: usize = 1;
const SIZE18: usize = 2;

/* Combines GameResult values into a single result */
impl std::ops::Add for GameResult {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut sum = GameResult::default();
        for i in 0..24 {
            for j in 0..3 {
                for k in 0..2 {
                    sum.data[i][j][k] = self.data[i][j][k] + other.data[i][j][k];
                }
            }
        }
        sum
    }
}

/* Reports findings of a single set in a hand */
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
        .find(|(x, y, z)| set::is_set(&hand[*x], &hand[*y], &hand[*z]))
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
            .find(|(y, z)| set::is_set(&hand[x], &hand[*y], &hand[*z]))
        {
            return Set::Found(x, y, z);
        }
    }

    /* All combinations with 2 of the new cards */
    for x in 0..split {
        if let Some((y, z)) = (split..len)
            .tuple_combinations()
            .find(|(y, z)| set::is_set(&hand[x], &hand[*y], &hand[*z]))
        {
            return Set::Found(x, y, z);
        }
    }

    /* Check if the three new cards make a set */
    if set::is_set(&hand[split], &hand[split + 1], &hand[split + 2]) {
        return Set::Found(split, split + 1, split + 2);
    }

    /* Otherwise, there are no sets in the hand */
    Set::NotFound()
}

/* Plays an entire game of set and returns some information */
fn play_game() -> GameResult {
    let mut result = GameResult::default();

    /* This is the data structure for the cards that haven't been dealt to the hand */
    let mut deck = deck::shuffle_cards();

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
        /* number of times cards have been dealt */
        let deals = 23 - (deck.len() / 3);
        match set {
            Set::Found(x, y, z) => {
                /* Add to the count that a set was found */
                match hand.len() {
                    12 => {
                        result.data[deals][SIZE12][SETS] += 1;
                        /* If the deck has 12 cards add 3 cards to the hand */
                        for _i in 0..3 {
                            match deck.pop() {
                                Some(x) => hand.push(x),
                                None => return result,
                            }
                        }
                    }
                    15 => result.data[deals][SIZE15][SETS] += 1,
                    18 => result.data[deals][SIZE18][SETS] += 1,
                    21 => (),
                    _ => {
                        log::info!("Unreachable hand size: {:?}", hand.len());
                        unreachable!();
                    }
                }

                /* Cards have to be removed in reverse order to prevent mixing up indices */
                let mut remove = vec![x, y, z];
                remove.sort();

                /* remove the cards from the hand that made a set */
                (0..3).rev().for_each(|i| {
                    hand.swap_remove(remove[i]);
                });

                /* search for the next set in all the cards */
                set = find_set_all(&hand);
            }
            Set::NotFound() => {
                /* update the count of hand with no sets */
                match hand.len() {
                    12 => result.data[deals][SIZE12][SETLESS] += 1,
                    15 => result.data[deals][SIZE15][SETLESS] += 1,
                    18 => result.data[deals][SIZE18][SETLESS] += 1,
                    _ => {
                        log::info!("Unreachable hand size: {:?}", hand.len());
                        unreachable!(); //21 card hands always have sets
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

pub fn run(games: i64) {
    /* loads in data from previous executions */
    let old = load();

    /* plays the game and sums all the results in parallel */
    let new: GameResult = (0..games)
        .into_par_iter()
        /* The reason fold and reduce are needed here is because of the way Rayon's parallel
         * iterator works. Fold allows the identity function (its first argument) to return a type
         * that is different from the type of thing we are iterating over (integers in this case).
         * Fold results in an iterator over GameResults that were created by breaking the original
         * iterator into the pieces that were distributed over threads. This is why reduce is
         * required to get the final sum.
         */
        .fold(GameResult::default, |acc, _| acc + play_game())
        .reduce(GameResult::default, |acc, x| acc + x);

    let results = old + new;

    /* Finalize */
    report(&new, games);
    write_results(&results);
}

/* Report the findings about the games */
fn report(results: &GameResult, games: i64) {
    log::info!("After {:?} games of simulated... \n\n", games);

    let mut data = [[0; 2]; 3];

    /* add up the results into general stats */
    for (i, _) in results.data.iter().enumerate() {
        for (j, _) in results.data[i].iter().enumerate() {
            for (k, _) in results.data[i][j].iter().enumerate() {
                data[j][k] += results.data[i][j][k];
            }
        }
    }

    log::info!("12 card hands with no sets: {:?}", data[SIZE12][SETLESS]);
    log::info!(
        "12 card hands where set was found: {:?}",
        data[SIZE12][SETS]
    );
    log::info!(
        "proportion of 12s w/out sets: {:.3}%\n",
        100.0 * data[SIZE12][SETLESS] as f64 / data[SIZE12][SETS] as f64
    );

    log::info!("15 card hands with no sets: {:?}", data[SIZE15][SETLESS]);
    log::info!(
        "15 card hands where set was found: {:?}",
        data[SIZE15][SETS]
    );
    log::info!(
        "proportion of 15s w/out sets: {:.3}%\n",
        100.0 * data[SIZE15][SETLESS] as f64 / data[SIZE15][SETS] as f64
    );

    log::info!("18 card hands with no sets: {:?}", data[SIZE18][SETLESS]);
    log::info!(
        "18 card hands where set was found: {:?}",
        data[SIZE18][SETS]
    );
    log::info!(
        "proportion of 18s w/out sets: {:.3}%\n",
        100.0 * data[SIZE18][SETLESS] as f64 / data[SIZE18][SETS] as f64
    );
}

/* Loads data from previous executions into memory */
fn load() -> GameResult {
    let path = Path::new("python/data/rm_first/data.csv");
    let contents = fs::read_to_string(path).unwrap();

    let mut rdr = csv::Reader::from_reader(contents.as_bytes());

    let mut old = GameResult::default();

    /* index values for the csv records */
    const DEALS: usize = 0;
    const SETLESS12: usize = 1;
    const SET12: usize = 2;
    const SETLESS15: usize = 3;
    const SET15: usize = 4;
    const SETLESS18: usize = 5;
    const SET18: usize = 6;

    /* read each value into memory from csv */
    for result in rdr.records() {
        let record = result.unwrap();
        let deals: usize = record[DEALS].parse().unwrap();
        old.data[deals][SIZE12][SETS] = record[SET12].parse().unwrap();
        old.data[deals][SIZE12][SETLESS] = record[SETLESS12].parse().unwrap();
        old.data[deals][SIZE15][SETS] = record[SET15].parse().unwrap();
        old.data[deals][SIZE15][SETLESS] = record[SETLESS15].parse().unwrap();
        old.data[deals][SIZE18][SETS] = record[SET18].parse().unwrap();
        old.data[deals][SIZE18][SETLESS] = record[SETLESS18].parse().unwrap();
    }
    old
}

/* Exports results to an external data file */
fn write_results(results: &GameResult) {
    /* Serialize the data into a string so it can be published to an external file */
    let serialized: String =
        String::from("deals,setless12,set12,setless15,set15,setless18,set18\n")
            + &itertools::join(
                results
                    .data
                    .iter()
                    .enumerate()
                    .map(|(i, x)| {
                        i.to_string()
                            + ","
                            + &itertools::join(
                                x.iter()
                                    .map(|y| y[SETLESS].to_string() + "," + &y[SETS].to_string()),
                                ",",
                            )
                    })
                    .collect::<Vec<String>>(),
                "\n",
            );

    /* Create the output filename using the current date/time */
    let path_name = "python/data/rm_first/data.csv";

    /* Create the path to write file to */
    let path = Path::new(path_name);
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
