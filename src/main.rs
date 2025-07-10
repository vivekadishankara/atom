mod file_names;
mod habit;
mod setup;
mod manager;

use std::env::args;
use crate::manager::Manager;

fn main() {
    let this_args: Vec<String> = args().collect();  
    Manager::enact(&this_args);
}
