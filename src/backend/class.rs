use std::vec::Vec;

pub struct Class {
    name: String,
    level: i32, // 0 - 50

    modifiers: Vec<[i32; 2]>,
}

/*
DEFAULT CLASSES:
- Ranger
    1: 1.5x acc
    5: 1.25x crit
    10: 2x att

- Assassin
- Warrior
- Tank
*/

impl Class {
    /* CONSTRUCTORS */
    pub fn none() -> Self {
        Self {
            name: "None".to_string(),
            level: 1i32,
            modifiers: vec![[1, 1]],
        }
    }

    pub fn make_ranger() -> Self {
        Self {
            name: "Ranger".to_string(),
            level: 1i32,
            modifiers: vec![[1, 1], [5, 1], [10, 1]],
        }
    }

    pub fn make_assassin() -> Self {
        Self {
            name: "Assassin".to_string(),
            level: 1i32,
            modifiers: vec![[1, 1], [5, 1], [10, 1]],
        }
    }

    pub fn make_warrior() -> Self {
        Self {
            name: "Warrior".to_string(),
            level: 1i32,
            modifiers: vec![[1, 1], [5, 1], [10, 1]],
        }
    }

    pub fn make_tank() -> Self {
        Self {
            name: "Tank".to_string(),
            level: 1i32,
            modifiers: vec![[1, 1], [5, 1], [10, 1]],
        }
    }

    /* CHECKS */
    fn is_passive(&self, num: usize) -> bool {
        if self.modifiers[num][1] == 1i32 {
            return true
        } else {
            return false
        }
    }

    fn is_level(&self, num: usize, unit_lvl: i32) -> bool {
        if self.modifiers[num][0] > unit_lvl {
            return false
        } else {
            return true
        }
    }

    /* RANGER MODS */
    pub fn ranger_passives(&self) {
        // output signal to change stat_b of unit

        for index in 0..(self.modifiers.len() - 1) {

        }
    }

    fn ranger1(&self) {
        
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_class_none() {
        let modless = Class::none();
        assert_eq!(modless.name, "None".to_string());
    }

    #[test]
    fn test_make_ranger() {
        let ranger = Class::make_ranger();
        assert_eq!(ranger.name, "Ranger".to_string());
        assert_eq!(ranger.level, 1i32);

        println!("{}", ranger.modifiers.len());

        assert_eq!(ranger.modifiers[0][0], 1i32);
        assert_eq!(ranger.modifiers[0][1], 1i32);
        assert_eq!(ranger.modifiers[1][0], 5i32);
        assert_eq!(ranger.modifiers[1][1], 1i32);
        assert_eq!(ranger.modifiers[2][0], 10i32);
        assert_eq!(ranger.modifiers[2][1], 1i32);
    }
}
