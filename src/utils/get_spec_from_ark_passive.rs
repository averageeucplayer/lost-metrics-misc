use lost_metrics_core::models::ArkPassiveNode;
use num_traits::FromPrimitive;
use strum_macros::{AsRefStr, EnumString};

#[derive(Default, Debug, AsRefStr, PartialEq, EnumString, FromPrimitive)]
#[repr(u32)]
pub enum ArkPassiveNodeIdentifier {
    #[default]
    Unknown = 0,
    #[strum(serialize = "Berserker Technique")]
    BerserkerTechnique = 2160000,
    Mayhem = 2160010,
    #[strum(serialize = "Lone Knight")]
    LoneKnight = 2170000,
    #[strum(serialize = "Combat Readiness")]
    CombatReadiness = 2170010,
    #[strum(serialize = "Rage Hammer")]
    RageHammer = 2180000,
    #[strum(serialize = "Gravity Training")]
    GravityTraining = 2180010,
    Judgement = 2360000,
    #[strum(serialize = "Blessed Aura")]
    BlessedAura = 2360010,
    Punisher = 2450000,
    Predator = 2450010,
    #[strum(serialize = "Ultimate Skill: Taijutsu")]
    UltimateSkillTaijutsu = 2230000,
    #[strum(serialize = "Shock Training")]
    ShockTraining = 2230100,
    #[strum(serialize = "First Intention")]
    FirstIntention = 2220000,
    #[strum(serialize = "Esoteric Skill Enhancement")]
    EsotericSkillEnhancement = 2220100,
    #[strum(serialize = "Energy Overflow")]
    EnergyOverflow = 2240000,
    #[strum(serialize = "Robust Spirit")]
    RobustSpirit = 2240100,
    Control = 2340000,
    Pinnacle = 2340100,
    #[strum(serialize = "Brawl King Storm")]
    BrawlKingStorm = 2470000,
    #[strum(serialize = "Asura's Path")]
    AsurasPath = 2470100,
    #[strum(serialize = "Esoteric Flurry")]
    EsotericFlurry = 2390000,
    Deathblow = 2390010,
    #[strum(serialize = "Barrage Enhancement")]
    BarrageEnhancement = 2300000,
    #[strum(serialize = "Firepower Enhancement")]
    FirepowerEnhancement = 2300100,
    #[strum(serialize = "Enhanced Weapon")]
    EnhancedWeapon = 2290000,
    Pistoleer = 2290100,
    #[strum(serialize = "Death Strike")]
    DeathStrike = 2280000,
    #[strum(serialize = "Loyal Companion")]
    LoyalCompanion = 2280100,
    #[strum(serialize = "Evolutionary Legacy")]
    EvolutionaryLegacy = 2350000,
    #[strum(serialize = "Arthetinean Skill")]
    ArthetineanSkill = 2350100,
    Peacemaker = 2380000,
    #[strum(serialize = "Time to Hunt")]
    TimeToHunt = 2380100,
    Igniter = 2370000,
    Reflux = 2370100,
    #[strum(serialize = "Grace of the Empress")]
    GraceOfTheEmpress = 2190000,
    #[strum(serialize = "Order of the Emperor")]
    OrderOfTheEmperor = 2190100,
    #[strum(serialize = "Communication Overflow")]
    CommunicationOverflow = 2200000,
    #[strum(serialize = "Master Summoner")]
    MasterSummoner = 2200100,
    #[strum(serialize = "Desperate Salvation")]
    DesperateSalvation = 2210000,
    #[strum(serialize = "True Courage")]
    TrueCourage = 2210100,
    #[strum(serialize = "Demonic Impulse")]
    DemonicImpulse = 2270000,
    #[strum(serialize = "Perfect Suppression")]
    PerfectSuppression = 2270600,
    Surge = 2250000,
    #[strum(serialize = "Remaining Energy")]
    RemainingEnergy = 2250600,
    #[strum(serialize = "Lunar Voice")]
    LunarVoice = 2260000,
    Hunger = 2260600,
    #[strum(serialize = "Full Moon Harveste")]
    FullMoonHarvester = 2460000,
    #[strum(serialize = "Night's Edge")]
    NightsEdge = 2460600,
    #[strum(serialize = "Wind Fury")]
    WindFury = 2320000,
    Drizzle = 2320600,
    #[strum(serialize = "Full Bloom")]
    FullBloom = 2310000,
    Recurrence = 2310600,
    Ferality = 2330000,
    #[strum(serialize = "Phantom Beast Awakening")]
    PhantomBeastAwakening = 2330100,
}

pub fn get_spec_from_ark_passive(node: &ArkPassiveNode) -> String {
    ArkPassiveNodeIdentifier::from_u32(node.id)
        .unwrap_or_default()
        .as_ref()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_unknown() {
        let node = ArkPassiveNode {
            id: 3,
            lv: 1
        };
        let actual = get_spec_from_ark_passive(&node);
        assert_eq!(actual, "Unknown")
    }

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