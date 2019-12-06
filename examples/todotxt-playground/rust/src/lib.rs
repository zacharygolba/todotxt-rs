use todotxt::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn parse(input: &str) -> String {
    let tasks: Vec<Task<'_>> = input.tasks().collect();

    if tasks.is_empty() {
        String::new()
    } else {
        format!("{:#?}", tasks)
    }
}
