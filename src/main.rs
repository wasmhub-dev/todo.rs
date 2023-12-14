mod state;
mod todo;

use wasm_bindgen::prelude::*;
use todo::TodoApp;
use gloo::console::log;


#[wasm_bindgen(main)]
pub fn main() {
    log!("Welcome to the wasm world!");
    TodoApp::new().show_task();
}