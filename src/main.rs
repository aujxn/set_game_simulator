/* Author: Austen Nelson
 * A Set game simulator
 *
 * 8/19/2019
 */

use std::time::Duration;
use set_simulator::{find_all_sets::run, consolidate};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "set_simulator", about = "Monte Carlo Analysis of Set")]
enum Opt {
    /// Run the simulation
    Run {
        /// Time to run in hours
        #[structopt(short, long, default_value = "0")]
        hours: u64,

        /// Time to run in minutes
        #[structopt(short, long, default_value = "0")]
        minutes: u64,

        /// Time to run in seconds 
        #[structopt(short, long, default_value = "0")]
        seconds: u64,

        /// Number of threads to use
        #[structopt(short, long, default_value = "20")]
        threads: usize,
    },
    /// Consolidate all data files into one
    Consolidate
}

fn main() {
    /* Initialization */
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "full");
    env_logger::init();

    match Opt::from_args() {
        Opt::Run{hours, minutes, seconds, threads} => run(Duration::from_secs(hours * 3600 + minutes * 60 + seconds), threads),
        Opt::Consolidate => consolidate().expect("failed to consolidate"),
    }
}
