use failure::Error;
use std::io::{self, Read};
use todotxt::prelude::*;

fn main() -> Result<(), Error> {
    let mut input = String::new();

    io::stdin().read_to_string(&mut input)?;
    Ok(serde_json::to_writer_pretty(
        io::stdout(),
        &input.tasks().collect::<Vec<_>>(),
    )?)
}
