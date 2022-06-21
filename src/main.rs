#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

pub mod client;
pub mod decimal;
pub mod engine;
pub mod errors;
pub mod transaction;

use std::env::args;
use std::error::Error;
use std::fs::File;
use std::io::{stdout, BufReader};
use std::process;

use csv::Writer;
use engine::Engine;
use errors::EngineError;

use crate::transaction::Transaction;

fn main() -> Result<(), EngineError> {
    let args: Vec<String> = args().collect();
    if args.len() < 2 {
        println!("Usage: {} <input csv>", args[0]);
        process::exit(1);
    }

    let mut engine = Engine::new();
    load_transactions(&mut engine, &args[1])
        .map_err(|_| EngineError::IOError("Could not open input file."))?;
    dump_accounts(&mut engine).map_err(|_| EngineError::IOError("Could not write output file."))
}

fn load_transactions(engine: &mut Engine, filename: &String) -> Result<(), Box<dyn Error>> {
    let mut rdr = File::open(filename).map(|f| csv::Reader::from_reader(BufReader::new(f)))?;

    for result in rdr.deserialize() {
        // Ignore invalid entries
        if result.is_err() {
            continue;
        }

        let transaction: Transaction = result.unwrap();
        if let Err(_) = engine.execute(&transaction) {
            // Ignore invalid transactions
        }
    }

    Ok(())
}

fn dump_accounts(engine: &mut Engine) -> Result<(), Box<dyn Error>> {
    let mut wtr = Writer::from_writer(stdout());

    for (_, client) in engine.iter_clients() {
        wtr.serialize(client)?;
    }

    wtr.flush()?;
    Ok(())
}
