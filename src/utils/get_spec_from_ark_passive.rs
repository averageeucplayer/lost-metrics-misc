use std::mem::transmute;

use lost_metrics_core::models::ArkPassiveNode;
use strum_macros::{AsRefStr, EnumString};

#[derive(Default, Debug, AsRefStr, PartialEq, EnumString)]
#[repr(u32)]
pub enum ArkPassiveNodeIdentifier {
    #[default]
    Unknown = 0,
    #[strum(serialize = "Berserker Technique")]
    BerserkerTechnique = 2160000,
    Mayhem = 2160010,
    LoneKnight = 2170000,
    CombatReadiness = 2170010,
    RageHammer = 2180000,
    GravityTraining = 2180010,
    Judgement = 2360000,
    BlessedAura = 2360010,
    Punisher = 2450000,
    Predator = 2450010,
    #[strum(serialize = "Ultimate Skill: Taijutsu")]
    UltimateSkillTaijutsu = 2230000,
    ShockTraining = 2230100,
    FirstIntention = 2220000,
    EsotericSkillEnhancement = 2220100,
    EnergyOverflow = 2240000,
    RobustSpirit = 2240100,
    Control = 2340000,
    Pinnacle = 2340100,
    BrawlKingStorm = 2470000,
    AsurasPath = 2470100,
    EsotericFlurry = 2390000,
    Deathblow = 2390010,
    BarrageEnhancement = 2300000,
    FirepowerEnhancement = 2300100,
    EnhancedWeapon = 2290000,
    Pistoleer = 2290100,
    DeathStrike = 2280000,
    LoyalCompanion = 2280100,
    EvolutionaryLegacy = 2350000,
    ArthetineanSkill = 2350100,
    Peacemaker = 2380000,
    TimeToHunt = 2380100,
    Igniter = 2370000,
    Reflux = 2370100,
    #[strum(serialize = "Grace of the Empress")]
    GraceOfTheEmpress = 2190000,
    #[strum(serialize = "Order of the Emperor")]
    OrderOfTheEmperor = 2190100,
    #[strum(serialize = "Communication Overflow")]
    CommunicationOverflow = 2200000,
    MasterSummoner = 2200100,
    DesperateSalvation = 2210000,
    TrueCourage = 2210100,
    DemonicImpulse = 2270000,
    PerfectSuppression = 2270600,
    Surge = 2250000,
    RemainingEnergy = 2250600,
    LunarVoice = 2260000,
    Hunger = 2260600,
    FullMoonHarvester = 2460000,
    NightsEdge = 2460600,
    WindFury = 2320000,
    Drizzle = 2320600,
    FullBloom = 2310000,
    Recurrence = 2310600,
    Ferality = 2330000,
    #[strum(serialize = "Phantom Beast Awakening")]
    PhantomBeastAwakening = 2330100,
}


pub fn get_spec_from_ark_passive(node: &ArkPassiveNode) -> String {
    match node.id {
        2160000 => "Berserker Technique",
        2160010 => "Mayhem",
        2170000 => "Lone Knight",
        2170010 => "Combat Readiness",
        2180000 => "Rage Hammer",
        2180010 => "Gravity Training",
        2360000 => "Judgement",
        2360010 => "Blessed Aura",
        2450000 => "Punisher",
        2450010 => "Predator",
        2230000 => "Ultimate Skill: Taijutsu",
        2230100 => "Shock Training",
        2220000 => "First Intention",
        2220100 => "Esoteric Skill Enhancement",
        2240000 => "Energy Overflow",
        2240100 => "Robust Spirit",
        2340000 => "Control",
        2340100 => "Pinnacle",
        2470000 => "Brawl King Storm",
        2470100 => "Asura's Path",
        2390000 => "Esoteric Flurry",
        2390010 => "Deathblow",
        2300000 => "Barrage Enhancement",
        2300100 => "Firepower Enhancement",
        2290000 => "Enhanced Weapon",
        2290100 => "Pistoleer",
        2280000 => "Death Strike",
        2280100 => "Loyal Companion",
        2350000 => "Evolutionary Legacy",
        2350100 => "Arthetinean Skill",
        2380000 => "Peacemaker",
        2380100 => "Time to Hunt",
        2370000 => "Igniter",
        2370100 => "Reflux",
        2190000 => "Grace of the Empress",
        2190100 => "Order of the Emperor",
        2200000 => "Communication Overflow",
        2200100 => "Master Summoner",
        2210000 => "Desperate Salvation",
        2210100 => "True Courage",
        2270000 => "Demonic Impulse",
        2270600 => "Perfect Suppression",
        2250000 => "Surge",
        2250600 => "Remaining Energy",
        2260000 => "Lunar Voice",
        2260600 => "Hunger",
        2460000 => "Full Moon Harvester",
        2460600 => "Night's Edge",
        2320000 => "Wind Fury",
        2320600 => "Drizzle",
        2310000 => "Full Bloom",
        2310600 => "Recurrence",
        2330000 => "Ferality",
        2330100 => "Phantom Beast Awakening",
        _ => "Unknown",
    }
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_get_spec() {
        let node = ArkPassiveNode {
            id: ArkPassiveNodeIdentifier::CommunicationOverflow as u32,
            lv: 1
        };
        let actual = get_spec_from_ark_passive(&node);
        assert_eq!(actual, "Communication Overflow")
    }
}