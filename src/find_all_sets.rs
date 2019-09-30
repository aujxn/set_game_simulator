/* This module records a lot more data than the rm_first_set module.
 * This will simulate the game of set but it finds every set in
 * each hand it plays. The number of cards in the hand, how many deals
 * have been made, how many sets were found, and the relationship
 * between those sets are all recorded. This version of the simulation
 * also removes a random set instead of the first set found.
 */

use crate::{deck, set, set::Card};
use itertools::Itertools;
use rand::Rng;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;
use std::sync::mpsc;
use std::thread;

/* A set is 3 indices in the hand */
#[derive(Clone, Copy, Debug)]
struct Set {
    indices: [usize; 3],
}

impl Set {
    /* Checks if a set shares cards with another set */
    fn no_share(self, other: &Self) -> bool {
        if let Some(_) = self.indices.iter().find(|&&x| {
            if let Some(_) = other
                .indices
                .iter()
                .find(|&&y| self.indices[x] == other.indices[y])
            {
                true
            } else {
                false
            }
        }) {
            return false;
        }
        true
    }

    /* other is being removed from the hand.
     * known sets that have indices that come after the cards that
     * are being removed must be decremented so they are still valid.
     */
    fn shift(&mut self, other: &Self) {
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
            .map(|(x, y, z)| Set { indices: [x, y, z] })
            .collect();

        Hand { cards, sets }
    }

    /* finds all the sets in the hand that contain any of the new cards */
    fn find_sets(&mut self, deck_len: usize) -> Info {
        let len = self.cards.len();
        let split = len - 3; //the divider between old and new cards

        /* calculates how many times cards have been removed from the deck */
        let deals = 23 - (deck_len / 3);

        /* searches for sets that have one of the new cards */
        for x in split..len {
            self.sets.append(
                &mut (0..split)
                    .tuple_combinations()
                    .filter(|(y, z)| set::is_set(&self.cards[x], &self.cards[*y], &self.cards[*z]))
                    .map(|(y, z)| Set { indices: [x, y, z] })
                    .collect(),
            );
        }

        /* searches for sets that have two of the new cards */
        for x in 0..split {
            self.sets.append(
                &mut (split..len)
                    .tuple_combinations()
                    .filter(|(y, z)| set::is_set(&self.cards[x], &self.cards[*y], &self.cards[*z]))
                    .map(|(y, z)| Set { indices: [x, y, z] })
                    .collect(),
            );
        }

        /* do the three new cards make a set? */
        if set::is_set(
            &self.cards[split],
            &self.cards[split + 1],
            &self.cards[split + 2],
        ) {
            self.sets.push(Set {
                indices: [split, split + 1, split + 2],
            });
        }

        Info {
            /* TODO: change this from clone to catagorizing of some type.
             * currently needless information is logged and program consumes
             * insane memory very quickly
             */
            sets: self.sets.clone(),

            hand_size: len,
            deals: deals,
        }
    }

    fn rm_set(&mut self, to_rm: &Set) {
        self.sets.retain(|x| x.no_share(to_rm));
        for set in &mut self.sets {
            set.shift(to_rm);
        }
        let mut a = [to_rm.indices[0], to_rm.indices[1], to_rm.indices[2]];
        a.sort();
        self.cards.remove(a[2]);
        self.cards.remove(a[1]);
        self.cards.remove(a[0]);
    }
}

/* records info about a hand
 * TODO: change to catagorical type to reduce memory load
 */
#[derive(Debug)]
pub struct Info {
    sets: Vec<Set>,   //all of the found sets
    hand_size: usize, //number of cards in the hand
    deals: usize,     //how many times cards have been removed from deck
}

impl Info {
    /* serializes the info to be written to an external file
     * TODO: will have to change with info rework
     */
    fn serde(&self) -> String {
        let sets = itertools::join(
            self.sets.iter().map(|x| {
                x.indices[0].to_string()
                    + " "
                    + &x.indices[1].to_string()
                    + " "
                    + &x.indices[2].to_string()
            }),
            "|",
        );

        self.sets.len().to_string()
            + ","
            + &self.hand_size.to_string()
            + ","
            + &self.deals.to_string()
            + ","
            + &sets
    }
}

/* plays an entire game of set and finds every set in every hand */
pub fn play_game() -> Vec<Info> {
    let mut deck = deck::shuffle_cards();
    let mut hand = Hand::new(&mut deck);
    let mut data: Vec<Info> = Vec::with_capacity(30);

    loop {
        let info = hand.find_sets(deck.len());
        let sets = info.sets.len();

        /* if sets were found then choose a random set and remove it */
        if sets > 0 {
            let to_rm = rand::thread_rng().gen_range(0, info.sets.len());
            hand.rm_set(&info.sets[to_rm]);
        }

        data.push(info);

        /* condition where hand needs a deal */
        if sets == 0 || hand.cards.len() < 12 {
            for _i in 0..3 {
                match deck.pop() {
                    Some(x) => hand.cards.push(x),
                    None => return data,
                }
            }
        }
    }
}

/* exports data to the file. experimenting with channels to achieve concurrency.
 * channel does not seem to be the bottleneck of the program.
 */
pub fn write_out(info: mpsc::Receiver<Vec<Info>>, kill: Vec<mpsc::Sender<bool>>, games: i64) {
    let path_name = "python/data/find_all/data.csv";

    /* Create the path to write file to */
    let path = Path::new(path_name);
    let display = path.display();

    /* Make the file */
    let mut file = match OpenOptions::new().append(true).create(true).open(path) {
        Err(why) => panic!("couldn't create {}: {}", display, why.description()),
        Ok(file) => file,
    };

    /* counts games played */
    let mut counter = 0;
    loop {
        match info.recv() {
            Ok(data) => {
                let serialized: String = itertools::join(
                    data.iter().map(|x| x.serde()).collect::<Vec<String>>(),
                    "\n",
                ) + "\n";
                /* Write the data to the file */
                /* TODO: change to a buffer instead of doing millions of writes */
                match file.write_all(serialized.as_bytes()) {
                    Err(why) => panic!("couldn't create {}: {}", display, why.description()),
                    Ok(_) => (),
                }
                counter += 1;
            }
            Err(_) => (),
        }

        /* games played reaches games desired so kill the loop */
        if counter == games {
            break;
        }
    }

    /* kill all the worker (simulation) threads */
    /* TODO: ask someone who knows things if this is a sensible way to handle concurrency */
    for rx in kill {
        /* i don't care if this succeeds/fails because my work is all done
         * and im just trying to shut down nicely.
         * and i just want rust to be happy.
         * this doesn't seem very "correct"
         */
        match rx.send(true) {
            Err(_) => (),
            Ok(_) => (),
        }
    }
}

/* runs the simulation */
pub fn run(games: i64) {
    /* number of threads playing set games */
    let workers = 4;

    let (tx, rx) = mpsc::channel();

    /* clone the transmitter for each worker */
    let txs = (0..workers)
        .map(|_| mpsc::Sender::clone(&tx))
        .collect::<Vec<mpsc::Sender<Vec<Info>>>>();

    /* create a way to shut down each thread */
    let (kill_txs, kill_rxs): (Vec<mpsc::Sender<bool>>, Vec<mpsc::Receiver<bool>>) =
        (0..workers).map(|_| &mpsc::channel()).unzip();

    /* spawn the worker threads */
    for i in 0..workers {
        thread::spawn(move || loop {
            txs[i].send(play_game());
            match kill_rxs[i].try_recv() {
                Ok(_) => break,
                Err(_) => (),
            }
        });
    }

    /* create a thread to do the exporting. */
    let writer = thread::spawn(move || write_out(rx, kill_txs, games));
    writer.join().unwrap();
}
