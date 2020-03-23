use std::vec::Vec;

pub use super::Unit;

pub struct Class {
    pub name: String,
    level: u8, // 0 - 50

    // needs restructuring for new stats type
    modifiers: Vec<(u8, bool)>,
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


impl Clone for Class {
    fn clone(&self) -> Self {
        let mut cloned_mod = vec![(0, false); self.modifiers.len()];

        for i in 0..(self.modifiers.len()) {
            cloned_mod[i] = self.modifiers[i];
        };

        Self {
        name: self.name.as_str().to_string(),
        level: self.level,
        modifiers: cloned_mod,
        }
    }
}


impl Class {
    /* CONSTRUCTORS */
    pub fn none() -> Self {
        Self {
            name: "None".to_string(),
            level: 1u8,
            modifiers: vec![(1, true)],
        }
    }

    pub fn make_ranger() -> Self {
        Self {
            name: "Ranger".to_string(),
            level: 1u8,
            modifiers: vec![(1, true), (5, true), (10, true)],
        }
    }

    pub fn make_assassin() -> Self {
        Self {
            name: "Assassin".to_string(),
            level: 1u8,
            modifiers: vec![(1, true), (5, true), (10, true)],
        }
    }

    pub fn make_warrior() -> Self {
        Self {
            name: "Warrior".to_string(),
            level: 1u8,
            modifiers: vec![(1, true), (5, true), (10, true)],
        }
    }

    pub fn make_tank() -> Self {
        Self {
            name: "Tank".to_string(),
            level: 1u8,
            modifiers: vec![(1, true), (5, true), (10, true)],
        }
    }

    /* CHECKS */
    fn is_passive(&self, num: usize) -> bool {
        if self.modifiers[num].1 == true {
            return true
        } else {
            return false
        }
    }

    fn is_level(&self, num: usize) -> bool {
        if self.modifiers[num].0 > self.level {
            return false
        } else {
            return true
        }
    }

    /* RANGER MODS */


    /* all passive mod functions must follow this structure:

    pub fn name(&self, stats: [u8; 11]) -> [f32; 11] {
        // will check class level and convert stat array by multiplying* index if level passes

        // also check for surpassing stats
    }

    all mod functions will then be compiled into a passive vector by a higher fn and used by unit
    */

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
        assert_eq!(ranger.level, 1u8);

        assert_eq!(ranger.modifiers[0].0, 1u8);
        assert_eq!(ranger.modifiers[0].1, true);
        assert_eq!(ranger.modifiers[1].0, 5u8);
        assert_eq!(ranger.modifiers[1].1, true);
        assert_eq!(ranger.modifiers[2].0, 10u8);
        assert_eq!(ranger.modifiers[2].1, true);
    }

    #[test]
    fn test_class_clone() {
        let ranger = Class::make_ranger();
        let ranger_clone = ranger.clone();

        assert_eq!(ranger.modifiers[0], ranger_clone.modifiers[0]);
        assert_eq!(ranger.modifiers[1], ranger_clone.modifiers[1]);
        assert_eq!(ranger.modifiers[2], ranger_clone.modifiers[2]);
    }
}
