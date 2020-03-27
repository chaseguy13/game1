
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


pub trait Unsigned {}

impl Unsigned for u8 {}
impl Unsigned for u16 {}

pub struct Stats<T: Unsigned> {
    health: u16,
    stamina: T,
    mana: T,

    attack: T,
    defense: T,
    magic: T,
    magic_defense: T,

    speed: T,
    accuracy: T,
    evasion: T,
    critical: T,
}

/*
- base_chase
- base_ryan

getters
setters
*/


impl Stats<u8> {
    pub fn base() -> Self {
        Self {
            health: 10u16,
            stamina: 10u8,
            mana: 10u8,
            attack: 1u8,
            defense: 1u8,
            magic: 1u8,
            magic_defense: 1u8,
            speed: 1u8,
            accuracy: 1u8,
            evasion: 1u8,
            critical: 1u8,
        }
    }

    pub fn base_chase() -> Self {
        Self {
            health: 10u16,
            stamina: 10u8,
            mana: 10u8,
            attack: 3u8,
            defense: 3u8,
            magic: 3u8,
            magic_defense: 3u8,
            speed: 3u8,
            accuracy: 3u8,
            evasion: 3u8,
            critical: 3u8,
        }
    }

    pub fn base_ryan() -> Self {
        Self {
            health: 10u16,
            stamina: 10u8,
            mana: 10u8,
            attack: 2u8,
            defense: 4u8,
            magic: 2u8,
            magic_defense: 4u8,
            speed: 2u8,
            accuracy: 4u8,
            evasion: 3u8,
            critical: 3u8,
        }
    }
}

impl<T: Unsigned> Stats<T> where T: Copy, {

    pub fn new(
        health: u16,
        stamina: T,
        mana: T,
        attack: T,
        defense: T,
        magic: T,
        magic_defense: T,
        speed: T,
        accuracy: T,
        evasion: T,
        critical: T,
    ) -> Self {
        Self {
            health,
            stamina,
            mana,
            attack,
            defense,
            magic,
            magic_defense,
            speed,
            accuracy,
            evasion,
            critical,
        }
    }

    /* GETTERS */

    pub fn get_hp(&self) -> u16 {
        return self.health.into()
    }

    pub fn get_sta(&self) -> T {
        return self.stamina.into()
    }

    pub fn get_mana(&self) -> T {
        return self.mana.into()
    }

    pub fn get_att(&self) -> T {
        return self.attack.into()
    }

    pub fn get_def(&self) -> T {
        return self.defense.into()
    }

    pub fn get_mag(&self) -> T {
        return self.magic.into()
    }

    pub fn get_mdef(&self) -> T {
        return self.magic_defense.into()
    }

    pub fn get_speed(&self) -> T {
        return self.speed.into()
    }

    pub fn get_acc(&self) -> T {
        return self.accuracy.into()
    }

    pub fn get_eva(&self) -> T {
        return self.evasion.into()
    }

    pub fn get_cit(&self) -> T {
        return self.critical.into()
    }

    /* SETTERS */

    pub fn set_hp(&mut self, val: u16) {
        self.health = val;
    }

    pub fn set_sta(&mut self, val: T) {
        self.stamina = val;
    }

    pub fn set_mana(&mut self, val: T) {
        self.mana = val;
    }

    pub fn set_att(&mut self, val: T) {
        self.attack = val;
    }

    pub fn set_def(&mut self, val: T) {
        self.defense = val;
    }

    pub fn set_mag(&mut self, val: T) {
        self.magic = val;
    }

    pub fn set_mdef(&mut self, val: T) {
        self.magic_defense = val;
    }

    pub fn set_spd(&mut self, val: T) {
        self.speed = val;
    }

    pub fn set_acc(&mut self, val: T) {
        self.accuracy = val;
    }

    pub fn set_eva(&mut self, val: T) {
        self.evasion = val;
    }

    pub fn set_crit(&mut self, val: T) {
        self.critical = val;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_getter() {
        let stats8 = Stats::base();
        let hp = stats8.get_hp();
        assert_eq!(hp, 10u16);
    }

    #[test]
    fn test_setter() {
        let mut stats8 = Stats::base();
        let mut hp = stats8.get_hp();
        assert_eq!(hp, 10u16);

        stats8.set_hp(8u16);
        hp = stats8.get_hp();
        assert_eq!(hp, 8u16);
    }
}
