#[macro_use(EthabiContract)]
extern crate ethabi_derive;

mod cli;
mod contracts;
mod counter;
mod error;
mod stats;
mod util;
mod validator;

use std::fs::File;
use std::time::SystemTime;

fn main() {
    let matches = cli::get_matches();

    let url = matches.value_of("url").unwrap_or("http://127.0.0.1:8545");
    let contract_file = matches
        .value_of("contracts")
        .unwrap_or("contracts/core.json");
    let file = File::open(contract_file).expect("open contracts file");
    let contract_addrs = serde_json::from_reader(file).expect("parse contracts file");

    let mut counter = counter::Counter::new(url, contract_addrs);

    if matches.is_present("verbose") {
        counter.set_verbose();
    }

    if let Some(period) = matches.value_of("period") {
        let duration = parse_duration::parse(period)
            .expect("period must be in the format '5 days', '2 months', etc.");
        counter.set_start_time(SystemTime::now() - duration);
    }

    if let Some(start_block) = matches.value_of("block") {
        counter.set_start_block(
            start_block
                .parse()
                .expect("block number must be a non-negative integer"),
        );
    }

    let stats = counter.count_votes().expect("count votes");
    println!("{}", stats);
}
