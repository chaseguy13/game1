// set up and run Game

#![allow(dead_code)]
#![allow(unused)]

mod backend;

use backend::Unit;

pub struct Game {
    unit_1: Unit,
    unit_2: Unit,
}

impl Game {
    pub fn new(unit_1: Unit, unit_2: Unit) -> Self {
        Self {
            unit_1,
            unit_2,
        }
    }


}
