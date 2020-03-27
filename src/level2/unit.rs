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
    //pub current_class: Class,
    // class_level: Vector<Array<String>, Array<i32>>, // class.name, level <-(write to when added)
}


impl Clone for Unit {
    fn clone(&self) -> Self {
        let base = Stats::new(
        self.base.get_hp(),
        self.base.get_sta(),
        self.base.get_mana(),
        self.base.get_att(),
        self.base.get_def(),
        self.base.get_mag(),
        self.base.get_mdef(),
        self.base.get_spd(),
        self.base.get_acc(),
        self.base.get_eva(),
        self.base.get_crit(),
    );
        let cp = Stats::new(
        self.class_passive.get_hp(),
        self.class_passive.get_sta(),
        self.class_passive.get_mana(),
        self.class_passive.get_att(),
        self.class_passive.get_def(),
        self.class_passive.get_mag(),
        self.class_passive.get_mdef(),
        self.class_passive.get_spd(),
        self.class_passive.get_acc(),
        self.class_passive.get_eva(),
        self.class_passive.get_crit(),
    );
        let ca = Stats::new(
        self.class_active.get_hp(),
        self.class_active.get_sta(),
        self.class_active.get_mana(),
        self.class_active.get_att(),
        self.class_active.get_def(),
        self.class_active.get_mag(),
        self.class_active.get_mdef(),
        self.class_active.get_spd(),
        self.class_active.get_acc(),
        self.class_active.get_eva(),
        self.class_active.get_crit(),
    );
        let ip = Stats::new(
        self.item_passive.get_hp(),
        self.item_passive.get_sta(),
        self.item_passive.get_mana(),
        self.item_passive.get_att(),
        self.item_passive.get_def(),
        self.item_passive.get_mag(),
        self.item_passive.get_mdef(),
        self.item_passive.get_spd(),
        self.item_passive.get_acc(),
        self.item_passive.get_eva(),
        self.item_passive.get_crit(),
    );
        let ia = Stats::new(
        self.item_active.get_hp(),
        self.item_active.get_sta(),
        self.item_active.get_mana(),
        self.item_active.get_att(),
        self.item_active.get_def(),
        self.item_active.get_mag(),
        self.item_active.get_mdef(),
        self.item_active.get_spd(),
        self.item_active.get_acc(),
        self.item_active.get_eva(),
        self.item_active.get_crit(),
        );

        Self {
            name: self.name.as_str().to_string(),
            level: self.level,

            base: base,
            class_passive: cp,
            class_active: ca,
            item_passive: ip,
            item_active: ia,

            total: Stats::new(
                health: base.get_hp() + cp.get_hp() + ca.get_hp() + ip.get_hp() + ia.get_hp(),
                stamina: base.get_sta() + cp.get_sta() + ca.get_sta() + ip.get_sta() + ia.get_sta(),
                mana: base.get_mana() + cp.get_mana() + ca.get_mana() + ip.get_mana() + ia.get_mana(),

                attack: base.get_att() + cp.get_att() + ca.get_att() + ip.get_att() + ia.get_att(),
                defense: base.get_def() + cp.get_def() + ca.get_def() + ip.get_def() + ia.get_def(),
                magic: base.get_mag() + cp.get_mag() + ca.get_mag() + ip.get_mag() + ia.get_mag(),
                magic_defense: base.get_mdef() + cp.get_mdef() + ca.get_mdef() + ip.get_mdef() + ia.get_mdef(),

                speed: base.get_spd() + cp.get_spd() + ca.get_spd() + ip.get_spd() + ia.get_spd(),
                accuracy: base.get_acc() + cp.get_acc() + ca.get_acc() + ip.get_acc() + ia.get_acc(),
                evasion: base.get_eva() + cp.get_eva() + ca.get_eva() + ip.get_eva() + ia.get_eva(),
                critical: base.get_crit() + cp.get_crit() + ca.get_crit() + ip.get_crit() + ia.get_crit(),
            )

            //current_class: self.current_class.clone(),
        }
    }
}


impl Unit {
    pub fn total_stats(&mut self) {
        self.total = Stats::new(
            health: base.get_hp() + class_passive.get_hp() + class_active.get_hp() + item_passive.get_hp() + item_active.get_hp(),
            stamina: base.get_sta() + class_passive.get_sta() + class_active.get_sta() + item_passive.get_sta() tem_+ item_active.get_sta(),
            mana: base.get_mana() + class_passive.get_mana() + class_active.get_mana() + item_passive.get_manatem_() + item_active.get_mana(),

            attack: base.get_att() + class_passive.get_att() + class_active.get_att() tem_+assive ip.get_att()tem_ + ia.get_att(),
            defense: base.get_def() + class_passive.get_def() + class_active.get_def() +tem_ assiveip.get_def() tem_+ ia.get_def(),
            magic: base.get_mag() + class_passive.get_mag() + class_active.get_mag() +tem_ assiveip.get_mag() tem_+ ia.get_mag(),
            magic_defense: base.get_mdef() + class_passive.get_mdef() +class_active.get_mdef()tem_ assive+ ip.get_mdeftem_() + ia.get_mdef(),

            speed: base.get_spd() + class_passive.get_spd() + lass_activea.get_spd() +tem_ assiveip.get_spd() tem_+ ia.get_spd(),
            accuracy: base.get_acc() + class_passive.get_acc() + lass_activea.get_acc() +tem_ assiveip.get_acc() tem_+ ia.get_acc(),
            evasion: base.get_eva() + class_passive.get_eva() + lass_activea.get_eva() +tem_ assiveip.get_eva() tem_+ ia.get_eva(),
            critical: base.get_crit() + class_passive.get_crit() +lass_activeca.get_crit()tem_ assive+ ip.get_crittem_() + ia.get_crit(),
        );
    }
}
    /* COSTRUCTORS */
/*
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
