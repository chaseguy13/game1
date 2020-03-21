use rand::distributions::uniform::{UniformFloat};

use super::Class;

pub struct Unit {
    pub name: String,
    // level: i32, // 1+

    /* BASIC STATS (change to array [base, class_p, class_a, item_p, item_a])*/
    health: i32, // 0 - 10000
    attack: i32, // 0 - 255
    defense: i32, // 0 - 255
    speed: i32, // 0 - 255
    accuracy: i32, // 0 - 100
    evasion: i32, // 0 - 100
    crit: i32, // 0 - 100

    /* CLASS DATA */
    pub current_class: Class,
    // class_level: Vector<Array<String>, Array<i32>>, // class.name, level <-(write to when added)
}


impl Clone for Unit {
    fn clone(&self) -> Self {
        Self {
            name: self.name.as_str().to_string(),
            health: self.health,
            attack: self.attack,
            defense: self.defense,
            speed: self.speed,
            accuracy: self.accuracy,
            evasion: self.evasion,
            crit: self.crit,
            current_class: self.current_class.clone(),
        }
    }
}


impl Unit {

    /* COSTRUCTORS */

    pub fn new(
        name: String,
        health: i32,
        attack: i32,
        defense: i32,
        speed: i32,
        accuracy: i32,
        evasion: i32,
        crit: i32,
    ) -> Self {
        Self {
            name,
            health,
            attack,
            defense,
            speed,
            accuracy,
            evasion,
            crit,
            current_class: Class::none(),
        }
    }

    pub fn premade_chase() -> Self {
        Self {
            name: "Chase".to_string(),
            health: 10i32,
            attack: 3i32,
            defense: 1i32,
            speed: 4i32,
            accuracy: 7i32,
            evasion: 2i32,
            crit: 0i32,
            current_class: Class::none(),
        }
    }

    pub fn premade_ryan() -> Self {
        Self {
            name: "Ryan".to_string(),
            health: 15i32,
            attack: 2i32,
            defense: 2i32,
            speed: 3i32,
            accuracy: 8i32,
            evasion: 1i32,
            crit: 0i32,
            current_class: Class::none(),
        }
    }

    /* VISBILITY */

    pub fn name(&self) -> String {
        self.name.as_str().into()
    }

    pub fn hp(&self) -> i32 {
        self.health.into()
    }

    pub fn _hp(&mut self, hp: i32) {
        self.health = hp;
    }

    pub fn att(&self) -> i32 {
        self.attack.into()
    }

    pub fn _att(&mut self, att: i32) {
        self.attack = att;
    }

    pub fn def(&self) -> i32 {
        self.defense.into()
    }

    pub fn _def(&mut self, def: i32) {
        self.defense = def;
    }

    pub fn spd(&self) -> i32 {
        self.speed.into()
    }

    pub fn _spd(&mut self, spd: i32) {
        self.speed = spd;
    }

    pub fn acc(&self) -> i32 {
        self.accuracy.into()
    }

    pub fn _acc(&mut self, acc: i32) {
        self.accuracy = acc;
    }

    pub fn eva(&self) -> i32 {
        self.evasion.into()
    }

    pub fn _eva(&mut self, eva: i32) {
        self.evasion = eva;
    }

    pub fn cri(&self) -> i32 {
        self.crit.into()
    }

    pub fn _cri(&mut self, cri: i32) {
        self.crit = cri;
    }

    /* COMBAT */

    pub fn take_damage(&mut self, dmg: i32) {
        self.health = self.health - dmg;
    }

    // hit check
    pub fn hit(&self, attacker: &Unit) -> bool {
        if attacker.accuracy >= self.evasion {
            return true
        } else {
            return false
        };
    }

    pub fn defend(&mut self, attacker: &Unit) {
        let dmg = if (attacker.attack - self.defense) > 0 {
            attacker.attack - self.defense
        } else {
            0
        };

        self.take_damage(dmg);
    }

    /* CLASS FUNCTIONS */

    // eventually allow adding but not changing current class
    pub fn add_class(&mut self, class: Class) {
        self.current_class = class;
    }

    // pub fn switch_class(&mut self, class: Class)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unit_new() {
        let name = "Chase".to_string();
        let (health, attack, defense, speed, accuracy, evasion, crit) = (1i32, 1i32, 1i32, 1i32, 5i32, 5i32, 0i32);
        let unit_chase = Unit::new(name, health, attack, defense, speed, accuracy, evasion, crit);

        assert_eq!("Chase".to_string(), unit_chase.name);
        assert_eq!(1i32, unit_chase.health);
        assert_eq!(1i32, unit_chase.attack);
        assert_eq!(1i32, unit_chase.defense);
        assert_eq!(1i32, unit_chase.speed);
        assert_eq!(5i32, unit_chase.accuracy);
        assert_eq!(5i32, unit_chase.evasion);
    }

    #[test]
    fn test_take_damage() {
        let name = "Chase".to_string();
        let (health, attack, defense, speed, accuracy, evasion, crit) = (5i32, 1i32, 1i32, 1i32, 5i32, 5i32, 0i32);
        let mut unit_chase = Unit::new(name, health, attack, defense, speed, accuracy, evasion, crit);
        unit_chase.take_damage(3i32);

        assert_eq!(unit_chase.health, 2i32);
    }

    #[test]
    fn test_defend() {
        let name = "Chase".to_string();
        let (health, attack, defense, speed, accuracy, evasion, crit) = (10i32, 3i32, 2i32, 1i32, 5i32, 5i32, 0i32);
        let mut unit_chase = Unit::new(name, health, attack, defense, speed, accuracy, evasion, crit);
        let unit_ryan = Unit::new("Name".to_string(), health, attack, defense, speed, accuracy, evasion, crit);
        unit_chase.defend(&unit_ryan);

        assert_eq!(unit_chase.health, health - attack + defense);
    }

    #[test]
    fn test_hit() {
        let chase = Unit::premade_chase();
        let ryan = Unit::premade_ryan();
        let land = chase.hit(&ryan);
        assert_eq!(land, true)
    }

    #[test]
    fn test_unit_add_class() {
        let mut chase = Unit::premade_chase();
        assert_eq!(chase.current_class.name, "None".to_string());

        chase.add_class(Class::make_ranger());
        assert_eq!(chase.current_class.name, "Ranger".to_string());
    }
}
