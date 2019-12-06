use std::fmt::Write;
use todotxt::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn parse(input: &str) -> String {
    let mut output = String::new();
    let tasks: Vec<Task> = input.tasks().collect();

    if !tasks.is_empty() {
        write!(output, "{:#?}", tasks).unwrap();
    }

    output
}
