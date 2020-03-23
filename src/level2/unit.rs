use rand::distributions::uniform::{UniformFloat};

use super::Class;
use super::Stats;

pub struct Unit {
    pub name: String,
    level: u8, // 1+

    /* STATS */
    base: Stats<u8>,

    class_passive: Stats<u8>,
    class_active: Stats<u8>,
    item_passive: Stats<u8>,
    item_active: Stats<u8>,

    total: Stats<u16>,


    /* CLASS DATA */
    pub current_class: Class,
    // class_level: Vector<Array<String>, Array<i32>>, // class.name, level <-(write to when added)
}

/*
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

    pub fn compile_passive_mods(&mut self, v: Vec<fn(&self, stat: u8) -> (u8, f32)>) {
        // idea: take in unknown number of functions as vector from class which spit stat index and multiplier

        // 1. unwrap functions from vector (solve them in order using given arguments)
    }

    pub fn compile_active_mods(&mut self) {
        // changes to stats
    }

}
*/

#[cfg(test)]
mod tests {
    use super::*;
}
