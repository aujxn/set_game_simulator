/* This module records a lot more data than the rm_first_set module.
 * This will simulate the game of set but it finds every set in
 * each hand it plays. The number of cards in the hand, how many deals
 * have been made, how many sets were found, and the relationship
 * between those sets are all recorded. This version of the simulation
 * also removes a random set instead of the first set found.
 */

use crate::{deck, set, set::Card, set::Set, thread_pool::ThreadPool};
use csv;
use itertools::Itertools;
use rand::Rng;
use std::{
    collections::HashMap, error::Error, fs, fs::File, io::prelude::*, path::Path, sync::Arc,
    sync::Mutex,
};

impl Set {
    /* other is being removed from the hand.
     * known sets that have indices that come after the cards that
     * are being removed must be decremented so they are still valid.
     */
    fn shift(&mut self, other: Self) {
        /* determines how much and which indices need changing */
        let shift: Vec<usize> = self
            .indices
            .iter()
            .map(|x| {
                other
                    .indices
                    .iter()
                    .filter(|y| x > y)
                    .fold(0, |acc, _| acc + 1)
            })
            .collect();

        /* updates indices */
        shift
            .iter()
            .enumerate()
            .for_each(|(i, x)| self.indices[i] -= x);
    }
}

/* represents the current cards that are in play */
struct Hand {
    cards: Vec<Card>,

    /* All known sets in the hand.
     * Tracking this prevents duplication of work
     */
    sets: Vec<Set>,
}

impl Hand {
    /* creates a hand by getting 12 cards from the deck */
    fn new(deck: &mut Vec<Card>) -> Self {
        let mut cards = vec![];

        for _i in 0..12 {
            match deck.pop() {
                Some(x) => cards.push(x),
                None => unreachable!(), //there will always be 81 cards in the deck to start
            }
        }

        /* finds all the sets in the first 9 cards of the hand.
         * the reasoning behind this is making the game loop more simple.
         * as the game is played, 3 cards are added to the hand.
         * because the hand struct keeps track of all of the sets,
         * the find_sets function only has to search for sets that contain
         * the new cards. so to start the game loop, this sets up the hand
         */
        let sets: Vec<Set> = (0..9)
            .tuple_combinations::<(_, _, _)>()
            .filter(|(x, y, z)| set::is_set(&cards[*x], &cards[*y], &cards[*z]))
            .map(|(x, y, z)| Set::new([x, y, z], &cards))
            .collect();

        Hand { cards, sets }
    }

    /* finds all the sets in the hand that contain any of the new cards */
    fn find_and_rm(&mut self, deck_len: usize) -> Info {
        let hand_size = self.cards.len();
        let split = hand_size - 3; //the divider between old and new cards

        /* calculates how many times cards have been removed from the deck */
        let deals = 23 - (deck_len / 3);

        /* searches for sets that have one of the new cards */
        for x in split..hand_size {
            self.sets.append(
                &mut (0..split)
                    .tuple_combinations()
                    .filter(|(y, z)| set::is_set(&self.cards[x], &self.cards[*y], &self.cards[*z]))
                    .map(|(y, z)| Set::new([x, y, z], &self.cards))
                    .collect(),
            );
        }

        /* searches for sets that have two of the new cards */
        for x in 0..split {
            self.sets.append(
                &mut (split..hand_size)
                    .tuple_combinations()
                    .filter(|(y, z)| set::is_set(&self.cards[x], &self.cards[*y], &self.cards[*z]))
                    .map(|(y, z)| Set::new([x, y, z], &self.cards))
                    .collect(),
            );
        }

        /* do the three new cards make a set? */
        if set::is_set(
            &self.cards[split],
            &self.cards[split + 1],
            &self.cards[split + 2],
        ) {
            self.sets.push(Set::new([split, split + 1, split + 2], &self.cards));
        }

        let mut sets = [0; 4];

        self.sets.iter().for_each(|x| sets[x.class as usize] += 1);

        /* if sets were found then choose a random set and remove it */
        if self.sets.len() > 0 {
            self.rm_set();
        }

        Info {
            cubes: sets[0],
            faces: sets[1],
            edges: sets[2],
            vertices: sets[3],
            hand_size,
            deals,
        }
    }

    /* removes a set from the list of sets */
    fn rm_set(&mut self) {
        /* pick a random set and destructor indices */
        let to_rm = self.sets[rand::thread_rng().gen_range(0, self.sets.len())];
        let (x, y, z) = (to_rm.indices[0], to_rm.indices[1], to_rm.indices[2]);

        /* remove all sets that share cards with set to remove */
        self.sets.retain(|set| {
            !set.indices
                .iter()
                .any(|&x| to_rm.indices.iter().any(|&y| x == y))
        });

        /* shift indices to correct for removal */
        for set in &mut self.sets {
            set.shift(to_rm);
        }

        /* remove in reverse to keep indices consistent */
        let mut a = [x, y, z];
        a.sort();
        a.iter().rev().for_each(|i| {
            self.cards.remove(*i);
        });
    }
}

/* records info about a hand */
#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
struct Info {
    cubes: usize,       //number of "cube" sets
    faces: usize,       //number of "face" sets
    edges: usize,       //number of "edge" sets
    vertices: usize,    //number of "vertex" sets
    hand_size: usize,   //number of cards in the hand
    deals: usize,       //how many times cards have been removed from deck
}

impl Info {
    fn serialize(&self) -> String {
        self.cubes.to_string()
            + ","
            + &self.faces.to_string()
            + ","
            + &self.edges.to_string()
            + ","
            + &self.vertices.to_string()
            + ","
            + &self.hand_size.to_string()
            + ","
            + &self.deals.to_string()
    }

    fn no_sets(&self) -> bool {
        self.cubes == 0 && self.faces == 0 && self.edges == 0 && self.vertices == 0
    }
}

/* plays an entire game of set and finds every set in every hand */
fn play_game(data_store: Arc<Mutex<HashMap<Info, i64>>>) {
    let mut deck = deck::shuffle_cards();
    let mut hand = Hand::new(&mut deck);
    let mut data: Vec<Info> = Vec::with_capacity(30);

    loop {
        let info = hand.find_and_rm(deck.len());
        data.push(info);

        /* condition where hand needs a deal */
        if info.no_sets()  || hand.cards.len() < 12 {
            for _i in 0..3 {
                match deck.pop() {
                    Some(x) => hand.cards.push(x),
                    None => {
                        /* when no cards remain add all the data to the map */
                        let mut map = data_store.lock().unwrap();
                        for key in data {
                            let count = map.entry(key).or_insert(0);
                            *count += 1;
                        }
                        return;
                    }
                }
            }
        }
    }
}

/* exports data to the file */
fn write_out(data: Arc<Mutex<HashMap<Info, i64>>>) {
    let map = data.lock().unwrap();

    /* Convert data to csv file */
    let serialized = String::from("cubes,faces,edges,vertices,hand_size,deals,count\n")
        + &itertools::join(
            map.iter()
                .map(|(info, count)| info.serialize() + "," + &count.to_string()),
            "\n",
        );

    /* Create the path to write file to */
    let path = Path::new("python/data/find_all/data.csv");
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

/* loads in the data from previous executions */
fn load(data: &Arc<Mutex<HashMap<Info, i64>>>) -> Result<(), Box<dyn Error + 'static>> {
    let path = Path::new("python/data/find_all/data.csv");
    let contents = fs::read_to_string(path)?;

    let mut rdr = csv::Reader::from_reader(contents.as_bytes());

    for result in rdr.records() {
        let record = result.unwrap();
        let info = Info {
            cubes: record[0].parse()?,
            faces: record[1].parse()?,
            edges: record[2].parse()?,
            vertices: record[3].parse()?,
            hand_size: record[4].parse()?,
            deals: record[5].parse()?,
        };
        let count: i64 = record[6].parse()?;

        let mut map = data.lock().unwrap();
        let val = map.entry(info).or_insert(0);
        *val += count;
    }
    Ok(())
}

/* runs the simulation */
pub fn run(games: i64) {
    /* number of threads playing set games */
    let workers = 4;

    /* chunk the work so the work queue doesn't consume memory */
    let chunk_size = 1000;
    let chunks = games / chunk_size;

    /* concurrent safe hash table to store the game result info in */
    let data: Arc<Mutex<HashMap<Info, i64>>> = Arc::new(Mutex::new(HashMap::new()));

    /* loads a file with existing data into memory */
    load(&data).unwrap_or_else(|_| log::info!("No file loaded"));

    for _ in 0..chunks {
        let pool = ThreadPool::new(workers);

        for _ in 0..chunk_size {
            let safe_map = data.clone();
            pool.execute(move || play_game(safe_map));
        }
    }

    write_out(data);
}
