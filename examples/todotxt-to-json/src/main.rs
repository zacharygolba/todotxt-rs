extern crate failure;
extern crate serde_json;
extern crate todotxt;

use std::io::{self, Read};

use failure::Error;
use todotxt::prelude::*;

fn main() -> Result<(), Error> {
    let mut input = String::new();

    io::stdin().read_to_string(&mut input)?;
    serde_json::to_writer_pretty(io::stdout(), &input.tasks().collect::<Vec<_>>())?;

    Ok(())
}
