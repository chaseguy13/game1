use rand::distributions::uniform::{UniformFloat};

use super::Class;

pub struct Unit {
    name: String,
    // level: i32, // 1+

    /* BASIC STATS (change to array [base, class_p, class_a, item_p, item_a])*/
    health: i32, // 0 - 10000
    attack: i32, // 0 - 255
    defense: i32, // 0 - 255
    speed: i32, // 0 - 255
    accuracy: i32, // 0 - 100
    evasion: i32, // 0 - 100
    crit: i32, // 0 - 100

    /*
    //
    health_p: i32,
    attack_p: i32,
    defense_p: i32,
    speed_p: i32,
    accuracy_p: i32,
    evasion_p: i32,
    crit_p: i32,
    */

    /* CLASS DATA */
    current_class: Class,
    // class_level: Vector<Array<String>, Array<i32>>, // class.name, level <-(write to when added)
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

    pub fn name(self) -> String {
        self.name.into()
    }

    pub fn hp(self) -> i32 {
        self.health.into()
    }

    pub fn att(self) -> i32 {
        self.attack.into()
    }

    pub fn def(self) -> i32 {
        self.defense.into()
    }

    pub fn spd(self) -> i32 {
        self.speed.into()
    }

    pub fn acc(self) -> i32 {
        self.accuracy.into()
    }

    pub fn eva(self) -> i32 {
        self.evasion.into()
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
}
