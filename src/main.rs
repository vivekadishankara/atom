mod habit;
mod setup;

use crate::setup::Setup;

fn main() {
    let mut setup = Setup::new();
    setup.start();
}
