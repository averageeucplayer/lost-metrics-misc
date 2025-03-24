use std::mem::transmute;

use strum_macros::{AsRefStr, EnumString};

#[derive(Default, Debug, AsRefStr, PartialEq, EnumString)]
#[repr(u32)]
pub enum Classes {
    #[default]
    Unknown = 0,
    #[strum(serialize = "Warrior (Male)")]
    WarriorMale = 101,
    Berserker = 102,
    Destroyer = 103,
    Gunlancer = 104,
    Paladin = 105,
    #[strum(serialize = "Warrior (Female)")]
    WarriorFemale = 111,
    Slayer = 112,
    Mage = 201,
    Arcanist = 202,
    Summoner = 203,
    Bard = 204,
    Sorceress = 205,
    #[strum(serialize = "Martial Artist (Female)")]
    MartialArtistFemale = 301,
    Wardancer = 302,
    Scrapper = 303,
    Soulfist = 304,
    Glaivier = 305,
    #[strum(serialize = "Martial Artist (Male)")]
    MartialArtistMale = 311,
    Striker = 312,
    Breaker = 313,
    Assassin = 401,
    Deathblade = 402,
    Shadowhunter = 403,
    Reaper = 404,
    Souleater = 405,
    #[strum(serialize = "Gunner (Male)")]
    GunnerMale = 501,
    Sharpshooter = 502,
    Deadeye = 503,
    Artillerist = 504,
    Machinist = 505,
    #[strum(serialize = "Gunner (Female)")]
    GunnerFemale = 511,
    Gunslinger = 512,
    Specialist = 601,
    Artist = 602,
    Aeromancer = 603,
    Wildsoul = 604,
}

impl From<u32> for Classes {
    fn from(value: u32) -> Self {
        unsafe { transmute(value) }
    }
}

pub fn get_class_from_id(class_id: &u32) -> String {
    let class: Classes = (*class_id).into();
    class.as_ref().to_string()
}

#[cfg(test)]
mod tests {
    use crate::get_class_from_id;

    #[test]
    fn should_return_class_name() {
        assert_eq!("Artist", get_class_from_id(&602));
    }
}