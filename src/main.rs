use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Debug)]
struct Card(usize, usize, usize, usize);

#[derive(Debug)]
struct Set {
    found: bool,
    count: usize,
    card1: usize,
    card2: usize,
    card3: usize,
}

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
    count: usize,
    added: bool,
}

impl Hand {
    fn find_set(&self, added: bool) -> Set {
        let mut i = self.count - 1;
        let mut j = i - 1;
        let mut k = j - 1;

        match added {
            false => loop {
                match is_set(&self.cards[i], &self.cards[j], &self.cards[k]) {
                    true => {
                        return Set {
                            found: true,
                            count: self.count,
                            card1: i,
                            card2: j,
                            card3: k,
                        }
                    }
                    false => {
                        if i == 2 {
                            match is_set(&self.cards[2], &self.cards[1], &self.cards[0]) {
                                true => {
                                    return Set {
                                        found: true,
                                        count: self.count,
                                        card1: 2,
                                        card2: 1,
                                        card3: 0,
                                    }
                                }
                                false => {
                                    return Set {
                                        found: false,
                                        count: self.count,
                                        card1: 0,
                                        card2: 0,
                                        card3: 0,
                                    }
                                }
                            }
                        } else if j == 1 {
                            i -= 1;
                            j = i - 1;
                            k = j - 1;
                        } else if k == 0 {
                            j -= 1;
                            k = j - 1;
                        } else {
                            k -= 1;
                        }
                    }
                }
            },
            true => loop {
                match is_set(&self.cards[i], &self.cards[j], &self.cards[k]) {
                    true => {
                        return Set {
                            found: true,
                            count: self.count,
                            card1: i,
                            card2: j,
                            card3: k,
                        };
                    }
                    false => {
                        if i == self.count - 4 {
                            return Set {
                                found: false,
                                count: self.count,
                                card1: 0,
                                card2: 0,
                                card3: 0,
                            };
                        } else if j == 1 {
                            i -= 1;
                            j = i - 1;
                            k = j - 1;
                        } else if k == 0 {
                            j -= 1;
                            k = j - 1;
                        } else {
                            k -= 1;
                        }
                    }
                }
            },
        }
    }
}

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
    let mut setless12 = 0;
    let mut set12 = 0;
    let mut setless15 = 0;
    let mut set15 = 0;
    let mut setless18 = 0;
    let mut set18 = 0;

    for x in 0..1000000 {
        let mut deck: Vec<Card> = vec![];

        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    for l in 0..3 {
                        deck.push(Card(i, j, k, l));
                    }
                }
            }
        }

        deck.shuffle(&mut thread_rng());

        let mut cards = vec![];
        let mut count = 12;
        let mut added = false;
        let mut game = true;

        let mut hand = Hand {
            cards,
            count,
            added,
        };

        for i in 0..12 {
            match deck.pop() {
                Some(x) => hand.cards.push(x),
                None => println!("out of cards!"),
            }
        }

        //println!("{:?}", hand);

        let mut set = hand.find_set(false);

        let mut found = 0;

        while game == true {
            match set.found {
                true => match set.count {
                    12 => set12 += 1,
                    15 => set15 += 1,
                    18 => set18 += 1,
                    _ => (),
                },
                false => match set.count {
                    12 => setless12 += 1,
                    15 => setless15 += 1,
                    18 => setless18 += 1,
                    _ => (),
                },
            }

            if hand.count < 13 || set.found == false {
                for i in 0..3 {
                    match deck.pop() {
                        Some(x) => hand.cards.push(x),
                        None => {
                            /*
                            println!("Game is over");
                            println!("Sets found: {:?}", found);
                            */
                            game = false;
                            continue;
                        }
                    }
                    hand.count += 1;
                }
            }
            match set.found {
                true => {
                    /*
                    println!("{:?}", set);
                    println!("{:?}", hand.cards[set.card1]);
                    println!("{:?}", hand.cards[set.card2]);
                    println!("{:?}", hand.cards[set.card3]);
                    */
                    hand.cards.swap_remove(set.card1);
                    hand.cards.swap_remove(set.card2);
                    hand.cards.swap_remove(set.card3);
                    hand.count -= 3;
                    set = hand.find_set(false);
                    found += 1;
                }
                false => {
                    //println!("{:?}", set);
                    set = hand.find_set(true);
                }
            }
        }
    }
    println!("setless 12's {:?}", setless12);
    println!("set 12's {:?}", set12);
    println!("proportion of 12's {:?}", setless12 as f64 / set12 as f64);

    println!("setless 15's {:?}", setless15);
    println!("set 15's {:?}", set15);
    println!("proportion of 15's {:?}", setless15 as f64 / set15 as f64);

    println!("setless 18's {:?}", setless18);
    println!("set 18's {:?}", set18);
    println!("proportion of 18's {:?}", setless18 as f64 / set18 as f64);
}
