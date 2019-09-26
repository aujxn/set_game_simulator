/* Author: Austen Nelson
 * A Set game simulator
 *
 * 8/19/2019
 */

/* CLI crate macros */
#[macro_use]
extern crate clap;
extern crate set_simulator;

pub use crate::set_simulator::{find_all_sets, rm_first_set};
use clap::App;

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

    // rm_first_set::run(games);

    find_all_sets::run(games);
}
