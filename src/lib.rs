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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_game_new() {
        let unit_1 = Unit::new("Chase".to_string(), 1i32, 1i32, 2i32, 1i32, 5i32, 5i32, 0i32);
        let unit_2 = Unit::new("Ryan".to_string(), 2i32, 2i32, 2i32, 2i32, 5i32, 5i32, 0i32);
        let game = Game::new(unit_1, unit_2);

        // need to set an equality for units?
        assert_eq!(game.unit_1.name(), "Chase".to_string());
        assert_eq!(game.unit_2.name(), "Ryan".to_string());
    }
}
