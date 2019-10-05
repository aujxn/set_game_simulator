/* Author: Austen Nelson
 * A Set game simulator
 *
 * 8/19/2019
 */

/* CLI crate macros */
#[macro_use]
extern crate clap;
use clap::App;

extern crate set_simulator;
pub use crate::set_simulator::{find_all_sets, rm_first_set};

fn main() {
    /* Initialization */
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "full");
    env_logger::init();

    /* CLI configuration and parsing */
    let yml = load_yaml!("cli.yml");
    let args = App::from_yaml(yml).get_matches();
    match args.subcommand() {
        ("rmfirst", Some(arg_matches)) => {
            rm_first_set::run(arg_matches.value_of("games").unwrap().parse().unwrap())
        }
        ("findall", Some(arg_matches)) => {
            find_all_sets::run(arg_matches.value_of("games").unwrap().parse().unwrap())
        }
        _ => unreachable!(), //clap app settings displays usage if no subcommand is provided
    }
}
