use rand::distributions::uniform::{UniformFloat};

use super::Class;
use super::UnitStats;

pub struct Unit {
    pub name: String,
    level: u8, // 1+

    /* STATS */
    stats: UnitStats,

    /* CLASS DATA */
    pub current_class: Class,
    // class_level: Vector<Array<String>, Array<i32>>, // class.name, level <-(write to when added)
}


impl Clone for Unit {
    fn clone(&self) -> Self {
        let stats_copy = UnitStats::new(
        self.stats.base_hp(),
        self.stats.base_sta(),
        self.stats.base_mana(),
        self.stats.base_att(),
        self.stats.base_def(),
        self.stats.base_mag(),
        self.stats.base_mdef(),
        self.stats.base_spd(),
        self.stats.base_acc(),
        self.stats.base_eva(),
        self.stats.base_crit(),
    );

        Self {
            name: self.name.as_str().to_string(),
            level: self.level,

            stats: stats_copy,

            //health: self.health,
            //attack: self.attack,
            //defense: self.defense,
            //speed: self.speed,
            //accuracy: self.accuracy,
            //evasion: self.evasion,
            //crit: self.crit,

            current_class: self.current_class.clone(),
        }
    }
}


impl Unit {

    /* COSTRUCTORS */

    pub fn new(
        name: String,
        level: u8,

        hp: u8,
        stam: u8,
        mana: u8,
        att: u8,
        def: u8,
        mag: u8,
        mdef: u8,
        spd: u8,
        acc: u8,
        eva: u8,
        crit: u8,
    ) -> Self {
        Self {
            name,
            level,
            stats: UnitStats::new (hp, stam, mana, att, def, mag, mdef, spd, acc, eva, crit),
            current_class: Class::none(),
        }
    }

    pub fn new_with_stats(name: String, stats: UnitStats, class: Class) -> Self {
        Self {
            name,
            level: 1u8,
            stats,
            current_class: class,
        }
    }

    pub fn premade_chase() -> Self {
        Self {
            name: "Chase".to_string(),
            level: 1u8,
            stats: UnitStats::chase_base(),
            current_class: Class::none(),
        }
    }

    pub fn premade_ryan() -> Self {
        Self {
            name: "Ryan".to_string(),
            level: 1u8,
            stats: UnitStats::ryan_base(),
            current_class: Class::none(),
        }
    }

    /* VISBILITY */

    pub fn name(&self) -> String {
        self.name.as_str().into()
    }

    pub fn level(&self) -> u16 {
        self.level.into()
    }

    pub fn hp(&self) -> u16 {
        self.stats.hp().into()
    }

    pub fn sta(&self) -> u16 {
        self.stats.sta().into()
    }

    pub fn mana(&self) -> u16 {
        self.stats.mana().into()
    }

    pub fn att(&self) -> u16 {
        self.stats.att().into()
    }

    pub fn def(&self) -> u16 {
        self.stats.def().into()
    }

    pub fn mag(&self) -> u16 {
        self.stats.mag().into()
    }

    pub fn mdef(&self) -> u16 {
        self.stats.mdef().into()
    }

    pub fn spd(&self) -> u16 {
        self.stats.spd().into()
    }

    pub fn acc(&self) -> u16 {
        self.stats.acc().into()
    }

    pub fn eva(&self) -> u16 {
        self.stats.eva().into()
    }

    pub fn crit(&self) -> u16 {
        self.stats.crit().into()
    }

    /* COMBAT */

    pub fn take_damage(&mut self, dmg: u16) {
        self.stats.total[0] = self.stats.total[0] - dmg;
    }

    // hit check (needs redefining, more robust)
    pub fn hit(&self, attacker: &Unit) -> bool {
        if attacker.stats.acc() >= self.stats.eva() {
            return true
        } else {
            return false
        };
    }

    pub fn defend(&mut self, attacker: &Unit) {
        let dmg = if (attacker.stats.att() - self.stats.def()) > 0 {
            attacker.stats.att() - self.stats.def()
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

    pub fn compile_passive_mods(&mut self) {
        // changes to stats
    }

    pub fn compile_active_mods(&mut self) {
        // changes to stats
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unit_new() {
        let name = "Chase".to_string();
        let chase = Unit::new_with_stats(name, UnitStats::chase_base(), Class::none());

        assert_eq!("Chase".to_string(), chase.name);
        assert_eq!(chase.hp(), 10u16);
        assert_eq!(chase.sta(), 10u16);
        assert_eq!(chase.mana(), 10u16);
        assert_eq!(chase.att(), 3u16);
        assert_eq!(chase.def(), 1u16);
        assert_eq!(chase.mag(), 4u16);
        assert_eq!(chase.mdef(), 1u16);
        assert_eq!(chase.spd(), 4u16);
        assert_eq!(chase.acc(), 7u16);
        assert_eq!(chase.eva(), 2u16);
        assert_eq!(chase.crit(), 0u16);
    }

    #[test]
    fn test_take_damage() {
        let mut chase = Unit::new_with_stats("Chase".to_string(), UnitStats::chase_base(), Class::none());
        chase.take_damage(3u16);

        assert_eq!(chase.stats.total[0], 7u16);
    }

    #[test]
    fn test_defend() {
        let mut chase = Unit::new_with_stats("Chase".to_string(), UnitStats::chase_base(), Class::none());
        let ryan = Unit::new_with_stats("Ryan".to_string(), UnitStats::ryan_base(), Class::none());
        let original_hp = chase.stats.total[0];
        chase.defend(&ryan);

        assert_eq!(chase.stats.total[0], original_hp - ryan.stats.total[3] + chase.stats.total[4]);
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
