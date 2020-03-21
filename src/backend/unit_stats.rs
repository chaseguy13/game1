
/* stats defined as [hp, stamina, mana, attack, defense, magic, magic defense, speed, accuracy, evasion, crit]
0: hp
1: stamina
2: mana
3: attack
4: defense
5: magic
6: magic defense
7: speed
8: accuracy
9: evasion
10: crit
*/

pub struct UnitStats {
    pub base: [u8; 11],

    pub class_passive: [u8; 11],
    pub class_active: [u8; 11],
    pub item_passive: [u8; 11],
    pub item_active: [u8; 11],

    pub total: [u16; 11],
}

impl UnitStats {
    pub fn new(
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
            base: [hp, stam, mana, att, def, mag, mdef, spd, acc, eva, crit],
            class_passive: [0u8; 11],
            class_active: [0u8; 11],
            item_passive: [0u8; 11],
            item_active: [0u8; 11],

            total: [hp as u16, stam as u16, mana as u16,
            att as u16, def as u16, mag as u16, mdef as u16,
            spd as u16, acc as u16, eva as u16, crit as u16],
        }
    }

    pub fn base() -> Self {
        Self {
            base: [1u8; 11],

            class_passive: [0u8; 11],
            class_active: [0u8; 11],
            item_passive: [0u8; 11],
            item_active: [0u8; 11],

            total: [1u16; 11],
        }
    }

    pub fn chase_base() -> Self {
        Self {
            base: [10u8, 10u8, 10u8,
            3u8, 1u8, 4u8, 1u8,
            4u8, 7u8, 2u8, 0u8],

            class_passive: [0u8; 11],
            class_active: [0u8; 11],
            item_passive: [0u8; 11],
            item_active: [0u8; 11],

            total: [10u16, 10u16, 10u16,
            3u16, 1u16, 4u16, 1u16,
            4u16, 7u16, 2u16, 0u16],
        }
    }

    pub fn ryan_base() -> Self {
        Self {
            base: [10u8, 10u8, 10u8,
            2u8, 2u8, 3u8, 2u8,
            3u8, 8u8, 1u8, 0u8],

            class_passive: [0u8; 11],
            class_active: [0u8; 11],
            item_passive: [0u8; 11],
            item_active: [0u8; 11],

            total: [10u16, 10u16, 10u16,
            2u16, 2u16, 3u16, 2u16,
            3u16, 8u16, 1u16, 0u16],
        }
    }

    pub fn base_hp(&self) -> u8 {
        return self.base[0].into()
    }

    pub fn base_sta(&self) -> u8 {
        return self.base[1].into()
    }

    pub fn base_mana(&self) -> u8 {
        return self.base[2].into()
    }

    pub fn base_att(&self) -> u8 {
        return self.base[3].into()
    }

    pub fn base_def(&self) -> u8 {
        return self.base[4].into()
    }

    pub fn base_mag(&self) -> u8 {
        return self.base[5].into()
    }

    pub fn base_mdef(&self) -> u8 {
        return self.base[6].into()
    }

    pub fn base_spd(&self) -> u8 {
        return self.base[7].into()
    }

    pub fn base_acc(&self) -> u8 {
        return self.base[8].into()
    }

    pub fn base_eva(&self) -> u8 {
        return self.base[9].into()
    }

    pub fn base_crit(&self) -> u8 {
        return self.base[10].into()
    }

    fn total(&self, ind: usize) -> u16 {
        return (self.base[ind] +
        self.class_passive[ind] +
        self.class_active[ind] +
        self.item_passive[ind] +
        self.item_active[ind]) as u16
    }

    fn set_total(&mut self) {
        self.total = [self.total(0), self.total(1), self.total(2), self.total(3), self.total(4),
        self.total(5), self.total(6), self.total(7), self.total(8), self.total(9), self.total(10)]
    }

    pub fn hp(&self) -> u16 {
        return self.total[0]
    }

    pub fn sta(&self) -> u16 {
        return self.total[1]
    }

    pub fn mana(&self) -> u16 {
        return self.total[2]
    }

    pub fn att(&self) -> u16 {
        return self.total[3]
    }

    pub fn def(&self) -> u16 {
        return self.total[4]
    }

    pub fn mag(&self) -> u16 {
        return self.total[5]
    }

    pub fn mdef(&self) -> u16 {
        return self.total[6]
    }

    pub fn spd(&self) -> u16 {
        return self.total[7]
    }

    pub fn acc(&self) -> u16 {
        return self.total[8]
    }

    pub fn eva(&self) -> u16 {
        return self.total[9]
    }

    pub fn crit(&self) -> u16 {
        return self.total[10]
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let unitstat = UnitStats::base();

        assert_eq!(unitstat.base[0], 1u8);
        assert_eq!(unitstat.base[1], 1u8);
        assert_eq!(unitstat.base[2], 1u8);
        assert_eq!(unitstat.base[3], 1u8);
        assert_eq!(unitstat.base[4], 1u8);
        assert_eq!(unitstat.base[5], 1u8);
        assert_eq!(unitstat.base[6], 1u8);
        assert_eq!(unitstat.base[7], 1u8);
        assert_eq!(unitstat.base[8], 1u8);
        assert_eq!(unitstat.base[9], 1u8);
        assert_eq!(unitstat.base[10], 1u8);
    }

    #[test]
    fn test_unitstat_total() {
        let stats = UnitStats::chase_base();
        assert_eq!(stats.total(3), 3u16);
        assert_eq!(stats.total(5), 4u16);
    }
}
