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
    match args.subcommmand() {
        Some("rmfirst", Some(games)) => rm_first_set::run(games.parse().unwrap()),
        Some("findall", Some(games)) => find_all_sets::run(games.parse().unwrap()),
        None => unreachable!(), //clap app settings displays usage if no subcommand is provided
    }
}
