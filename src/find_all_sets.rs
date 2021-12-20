/* This module records a lot more data than the rm_first_set module.
 * This will simulate the game of set but it finds every set in
 * each hand it plays. The number of cards in the hand, how many deals
 * have been made, how many sets were found, and the relationship
 * between those sets are all recorded. This version of the simulation
 * also removes a random set instead of the first set found.
 */

use crate::{deck, set, set::Card, set::Set, write_out, HandType, Info};
use indexmap::set::IndexSet;
use itertools::Itertools;
use rand::Rng;
use std::{
    collections::HashMap,
    sync::Arc,
    sync::Mutex,
    sync::{
        mpsc,
        mpsc::{channel, Receiver},
    },
    thread,
    time::Duration,
};

/* represents the current cards that are in play */
struct Hand {
    cards: IndexSet<Card>,
    sets: Vec<Set>,
}

impl Hand {
    /* creates a hand by getting 12 cards from the deck */
    fn new(deck: &mut Vec<Card>) -> Self {
        let cards: IndexSet<Card> = deck.drain(69..).collect();

        let sets: Vec<Set> = cards
            .iter()
            .tuple_combinations::<(_, _, _)>()
            .filter(|(x, y, z)| set::is_set(x, y, z))
            .map(|(x, y, z)| Set::new([*x, *y, *z]))
            .collect();

        Hand { cards, sets }
    }

    /* finds all the sets in the hand that contain any of the new cards */
    fn find_new_sets(&mut self, new_cards: IndexSet<Card>) {
        /* searches for sets that have one of the new cards */
        for x in new_cards.iter() {
            for (y, z) in self.cards.iter().tuple_combinations() {
                if set::is_set(x, y, z) {
                    self.sets.push(Set::new([*x, *y, *z]));
                }
            }
        }

        /* searches for sets that have two of the new cards */
        for (x, y) in new_cards.iter().tuple_combinations() {
            for z in self.cards.iter() {
                if set::is_set(x, y, z) {
                    self.sets.push(Set::new([*x, *y, *z]));
                }
            }
        }

        /* do the three new cards make a set? */
        if set::is_set(&new_cards[0], &new_cards[1], &new_cards[2]) {
            self.sets
                .push(Set::new([new_cards[0], new_cards[1], new_cards[2]]));
        }

        self.cards = self.cards.union(&new_cards).cloned().collect();
    }

    /* removes a set from the list of sets */
    fn rm_set(&mut self) {
        let index = rand::thread_rng().gen_range(0, self.sets.len());
        let set_to_rm = self.sets.swap_remove(index);

        self.sets
            .retain(|set| set.cards.is_disjoint(&set_to_rm.cards));
        self.cards = self.cards.difference(&set_to_rm.cards).cloned().collect();
    }

    fn num_sets(&self) -> usize {
        self.sets.len()
    }

    fn size(&self) -> usize {
        self.cards.len()
    }
}

/* plays an entire game of set and finds every set in every hand */
fn play_game(data_store: &Arc<Mutex<HashMap<Info, u64>>>) {
    let mut deck = deck::shuffle_cards();
    let mut hand = Hand::new(&mut deck);
    let mut deals = 0;
    let mut hand_type = HandType::Ascending;

    let mut data: Vec<Info> = Vec::with_capacity(30);

    loop {
        let set_count = hand.num_sets();
        let hand_size = hand.size();
        let info = Info {
            set_count,
            hand_size,
            deals,
            hand_type,
        };
        data.push(info);

        // a game of set always deals 3 cards 23 times
        // 12 (start) + 23 (deals) * 3 (cards per deal) = 81 cards
        if deals == 23 {
            break;
        }

        if set_count == 0 {
            let new_cards: IndexSet<Card> = deck.split_off(66 - deals * 3).into_iter().collect();
            hand.find_new_sets(new_cards);
            deals += 1;
            hand_type = HandType::Ascending;
        } else {
            hand.rm_set();

            if hand.size() < 12 {
                let new_cards: IndexSet<Card> =
                    deck.split_off(66 - deals * 3).into_iter().collect();
                hand.find_new_sets(new_cards);
                deals += 1;
                hand_type = HandType::Ascending;
            } else {
                hand_type = HandType::Descending;
            }
        }
    }
    match data_store.lock() {
        Ok(mut map) => {
            for key in data {
                let count = map.entry(key).or_insert(0);
                *count += 1;
            }
        }
        Err(_) => panic!("poisoned mutex"),
    }
}

fn run_thread(data: Arc<Mutex<HashMap<Info, u64>>>, kill_switch: Receiver<bool>) {
    loop {
        match kill_switch.try_recv() {
            Ok(_) => return,
            Err(error) => match error {
                mpsc::TryRecvError::Empty => (),
                mpsc::TryRecvError::Disconnected => panic!("channel disconnect"),
            },
        }
        for _ in 0..1000 {
            play_game(&data);
        }
    }
}

pub fn run(run_time: Duration, num_threads: usize) {
    let data: Arc<Mutex<HashMap<Info, u64>>> = Arc::new(Mutex::new(HashMap::new()));
    let mut senders = vec![];
    let mut join_handles = vec![];

    for _ in 0..num_threads {
        let (send, recv) = channel();
        senders.push(send);
        let threadsafe_data = data.clone();
        let handle = thread::spawn(move || run_thread(threadsafe_data, recv));
        join_handles.push(handle);
    }

    thread::sleep(run_time);

    for sender in senders {
        sender.send(true).expect("failed to send killswitch");
    }

    for handle in join_handles {
        handle.join().expect("couldn't join threads");
    }

    let output = std::process::Command::new("hostname")
        .output()
        .expect("failed to get host")
        .stdout;
    let hostname = &std::str::from_utf8(&output).unwrap();
    let pathname = format!(
        "data/data-{}-{:?}.csv",
        hostname,
        chrono::prelude::Utc::now()
    );

    let data = data.lock().expect("couldn't unlock data").clone();

    write_out(&data, &pathname);
}
