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

#[derive(Clone, Copy, Debug)]
struct Set(usize, usize, usize);

impl Set {
    fn no_share(self, other: &Self) -> bool {
        self.0 != other.0
            && self.0 != other.1
            && self.0 != other.2
            && self.1 != other.0
            && self.1 != other.1
            && self.1 != other.2
            && self.2 != other.0
            && self.2 != other.1
            && self.2 != other.2
    }

    fn shift(&mut self, other: &Self) {
        let mut shift0 = 0;
        let mut shift1 = 0;
        let mut shift2 = 0;

        if self.0 > other.0 {
            shift0 += 1;
        }
        if self.0 > other.1 {
            shift0 += 1;
        }
        if self.0 > other.2 {
            shift0 += 1;
        }
        if self.1 > other.0 {
            shift1 += 1;
        }
        if self.1 > other.1 {
            shift1 += 1;
        }
        if self.1 > other.2 {
            shift1 += 1;
        }
        if self.2 > other.0 {
            shift2 += 1;
        }
        if self.2 > other.1 {
            shift2 += 1;
        }
        if self.2 > other.2 {
            shift2 += 1;
        }

        self.0 -= shift0;
        self.1 -= shift1;
        self.2 -= shift2;
    }
}

struct Hand {
    cards: Vec<Card>,
    sets: Vec<Set>,
}

impl Hand {
    fn new(deck: &mut Vec<Card>) -> Self {
        let mut cards = vec![];

        /* Get the first 12 cards for the hand from the deck */
        for _i in 0..12 {
            match deck.pop() {
                Some(x) => cards.push(x),
                None => unreachable!(), //there will always be 81 cards in the deck to start
            }
        }

        let sets: Vec<Set> = (0..9)
            .tuple_combinations::<(_, _, _)>()
            .filter(|(x, y, z)| set::is_set(&cards[*x], &cards[*y], &cards[*z]))
            .map(|(x, y, z)| Set(x, y, z))
            .collect();

        Hand { cards, sets }
    }

    fn find_sets(&mut self, deck_len: usize) -> Info {
        let len = self.cards.len();
        let split = len - 3;
        let deals = 23 - (deck_len / 3);

        for x in split..len {
            self.sets.append(
                &mut (0..split)
                    .tuple_combinations()
                    .filter(|(y, z)| set::is_set(&self.cards[x], &self.cards[*y], &self.cards[*z]))
                    .map(|(y, z)| Set(x, y, z))
                    .collect(),
            );
        }

        for x in 0..split {
            self.sets.append(
                &mut (split..len)
                    .tuple_combinations()
                    .filter(|(y, z)| set::is_set(&self.cards[x], &self.cards[*y], &self.cards[*z]))
                    .map(|(y, z)| Set(x, y, z))
                    .collect(),
            );
        }

        if set::is_set(
            &self.cards[split],
            &self.cards[split + 1],
            &self.cards[split + 2],
        ) {
            self.sets.push(Set(split, split + 1, split + 2));
        }

        Info {
            sets: self.sets.clone(),
            hand_size: self.cards.len(),
            deals: deals,
        }
    }

    fn rm_set(&mut self, to_rm: &Set) {
        self.sets.retain(|x| x.no_share(to_rm));
        for set in &mut self.sets {
            set.shift(to_rm);
        }
        let mut a = [to_rm.0, to_rm.1, to_rm.2];
        a.sort();
        self.cards.remove(a[2]);
        self.cards.remove(a[1]);
        self.cards.remove(a[0]);
    }
}

#[derive(Debug)]
pub struct Info {
    sets: Vec<Set>,
    hand_size: usize,
    deals: usize,
}

impl Info {
    fn serde(&self) -> String {
        let sets = itertools::join(
            self.sets
                .iter()
                .map(|Set(x, y, z)| x.to_string() + " " + &y.to_string() + " " + &z.to_string()),
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

pub fn play_game() -> Vec<Info> {
    let mut deck = deck::shuffle_cards();
    let mut hand = Hand::new(&mut deck);
    let mut data: Vec<Info> = Vec::with_capacity(30);

    loop {
        let info = hand.find_sets(deck.len());
        let sets = info.sets.len();

        if sets > 0 {
            let to_rm = rand::thread_rng().gen_range(0, info.sets.len());
            hand.rm_set(&info.sets[to_rm]);
        }

        data.push(info);

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

pub fn write_out(info: mpsc::Receiver<Vec<Info>>, kill: Vec<mpsc::Sender<usize>>, games: i64) {

    let path_name = "python/data/find_all/data.csv";

    /* Create the path to write file to */
    let path = Path::new(path_name);
    let display = path.display();

    /* Make the file */
    let mut file = match OpenOptions::new().append(true).create(true).open(path) {
        Err(why) => panic!("couldn't create {}: {}", display, why.description()),
        Ok(file) => file,
    };

    let mut counter = 0;
    loop {
        match info.recv() {
            Ok(data) => {
                let serialized: String = itertools::join(
                    data.iter().map(|x| x.serde()).collect::<Vec<String>>(),
                    "\n",
                ) + "\n";
                /* Write the data to the file */
                match file.write_all(serialized.as_bytes()) {
                    Err(why) => panic!("couldn't create {}: {}", display, why.description()),
                    Ok(_) => (),
                }
                counter += 1;
            }
            Err(_) => ()
        }

        if counter == games {
            break;
        }
    }

    for rx in kill {
        rx.send(1);
    }
}

pub fn run(games: i64) {
    let (tx1, rx) = mpsc::channel();
    let tx2 = mpsc::Sender::clone(&tx1);
    let tx3 = mpsc::Sender::clone(&tx1);
    let tx4 = mpsc::Sender::clone(&tx1);

    let (kill1, r1) = mpsc::channel();
    let (kill2, r2) = mpsc::channel();
    let (kill3, r3) = mpsc::channel();
    let (kill4, r4) = mpsc::channel();

    thread::spawn(move || loop {
        tx1.send(play_game());
        match r1.try_recv() {
            Ok(_) => break,
            Err(_) => (),
        }
    });

    thread::spawn(move || loop {
        tx2.send(play_game());
        match r2.try_recv() {
            Ok(_) => break,
            Err(_) => (),
        }
    });

    thread::spawn(move || loop {
        tx3.send(play_game());
        match r3.try_recv() {
            Ok(_) => break,
            Err(_) => (),
        }
    });

    thread::spawn(move || loop {
        tx4.send(play_game());
        match r4.try_recv() {
            Ok(_) => break,
            Err(_) => (),
        }
    });

    let writer = thread::spawn(move || write_out(rx, vec![kill1, kill2, kill3, kill4], games));

    writer.join().unwrap();
}
