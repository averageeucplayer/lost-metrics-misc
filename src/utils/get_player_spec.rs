use hashbrown::HashMap;
use lost_metrics_core::models::*;

use super::misc::get_buff_names;

macro_rules! id {
    ($e:expr) => {
        &($e as u32)
    };
}

pub fn get_player_spec(
    class: Class,
    skills: &HashMap<u32, Skill>,
    buffed_by: &HashMap<u32, i64>,
    buffs: &HashMap<u32, StatusEffect>) -> &'static str {
    if skills.len() < 8 {
        return "Unknown";
    }

    match class {
        Class::Berserker => {
            skills
                .contains_key(id!(BerserkerSkills::BloodyRush))
                .then(|| "Berserker Technique")
                .unwrap_or_else(|| "Mayhem")
        }
        Class::Destroyer => {
            skills
                .contains_key(id!(DestroyerSkills::EarthWave))
                .then(|| "Gravity Training")
                .unwrap_or_else(|| "Rage Hammer")
        }
        Class::Gunlancer => {
            if skills.contains_key(id!(GunlancerSkills::SurgeCannon))
                && skills.contains_key(id!(GunlancerSkills::ChargedStinger)) {
                "Lone Knight"
            } else if skills.contains_key(id!(GunlancerSkills::GuardiansThundercrack)) {
                "Combat Readiness"
            } else {
                "Princess"
            }
        }
        Class::Paladin => {
            (skills.contains_key(id!(PaladinSkills::LightShock))
                || skills.contains_key(id!(PaladinSkills::SwordOfJustice))
                || skills.contains_key(id!(PaladinSkills::GodsDecree))
                || skills.contains_key(id!(PaladinSkills::HolyExplosion))
                && skills.contains_key(id!(PaladinSkills::HeavenlyBlessings))
                && skills.contains_key(id!(PaladinSkills::WrathOfGod)))
                .then(|| "Blessed Aura")
                .unwrap_or_else(|| "Judgement")
        }
        Class::Slayer => {
            skills
                .contains_key(id!(SlayerSkills::Bloodlust))
                .then(|| "Punisher")
                .unwrap_or_else(|| "Predator")
        }
        Class::Arcanist => {
            skills
                .contains_key(id!(ArcanistSkills::Emperor))
                .then(|| "Order of the Emperor")
                .unwrap_or_else(|| "Grace of the Empress")
        }
        Class::Summoner => {
            skills.iter()
                .any(|(_, skill)| skill.name.contains("Kelsion"))
                .then(|| "Communication Overflow")
                .unwrap_or_else(|| "Master Summoner")
        }
        Class::Bard => {
            ((skills.contains_key(id!(BardSkills::GuardianTune))
            || skills.contains_key(id!(BardSkills::RhapsodyOfLight))
            || skills.contains_key(id!(BardSkills::WindOfMusic)))
            && skills.contains_key(id!(BardSkills::HeavenlyTune)))
                .then(|| "Desperate Salvation")
                .unwrap_or_else(|| "True Courage")              
        }
        Class::Sorceress => {
            (skills.contains_key(id!(SorceressSkills::Doomsday))
                && skills.contains_key(id!(SorceressSkills::PunishingStrike))
                && skills.contains_key(id!(SorceressSkills::Explosion)))
                .then(|| "Igniter")
                .unwrap_or_else(|| "Reflux")
          
        }
        Class::Wardancer => {
            skills.contains_key(id!(WardancerSkills::EsotericSkillAzureDragonSupremeFist))
                .then(|| "Esoteric Skill Enhancement")
                .unwrap_or_else(|| "First Intention")
        }
        Class::Scrapper => {
            skills.contains_key(id!(ScrapperSkills::IronCannonBlow))
                .then(|| "Ultimate Skill: Taijutsu")
                .unwrap_or_else(|| "Shock Training")
        }
        Class::Soulfist => {
            skills.contains_key(id!(SoulfistSkills::Shadowbreaker))
                .then(|| "Energy Overflow")
                .unwrap_or_else(|| "Robust Spirit")
        }
        Class::Glaivier => {
            skills.contains_key(id!(GlaivierSkills::RedDragonsHorn))
                .then(|| "Pinnacle")
                .unwrap_or_else(|| "Control")
        }
        Class::Striker => {
            skills.contains_key(id!(StrikerSkills::EsotericSkillCallOfTheWindGod))
                .then(|| "Esoteric Flurry")
                .unwrap_or_else(|| "Deathblow")
        }
        Class::Breaker => {
            skills.contains_key(id!(BreakerSkills::AsuraDestructionBasicAttack))
                .then(|| "Asura's Path")
                .unwrap_or_else(|| "Brawl King Storm")
        }
        Class::Deathblade => {
            skills.contains_key(id!(DeathbladeSkills::Zero))
                .then(|| "Surge")
                .unwrap_or_else(|| "Remaining Energy")
        }
        Class::Shadowhunter => {
            skills.contains_key(id!(ShadowhunterSkills::BloodMassacre))
                .then(|| "Demonic Impulse")
                .unwrap_or_else(|| "Perfect Suppression")
        }
        Class::Reaper => {
            let buff_names = get_buff_names(buffed_by, buffs);
            buff_names.iter().any(|s| s.contains("Lunar Voice"))
                .then(|| "Lunar Voice")
                .unwrap_or_else(|| "Hunger")
        }
        Class::Souleater => {
            skills.contains_key(id!(SouleaterSkills::LethalSpinning))
                .then(|| "Night's Edge")
                .unwrap_or_else(|| "Full Moon Harvester")
        }
        Class::Sharpshooter => {
            let buff_names = get_buff_names(buffed_by, buffs);
            buff_names
                .iter()
                .any(|s| s.contains("Loyal Companion") || s.contains("Hawk Support"))
                .then(|| "Loyal Companion")
                .unwrap_or_else(|| "Death Strike")
        }
        Class::Deadeye => {
            skills.contains_key(id!(DeadeyeSkills::JudgmentDay)) 
                .then(|| "Enhanced Weapon")
                .unwrap_or_else(|| "Pistoleer")
        }
        Class::Artillerist => {
            skills.contains_key(id!(ArtilleristSkills::BarrageFocusFire)) 
                .then(|| "Barrage Enhancement")
                .unwrap_or_else(|| "Firepower Enhancement")
        }
        Class::Machinist => {
            let buff_names = get_buff_names(buffed_by, buffs);
            buff_names
                .iter()
                .any(|s| s.contains("Combat Mode") || s.contains("Evolutionary Legacy"))
                .then(|| "Evolutionary Legacy")
                .unwrap_or_else(|| "Arthetinean Skill")
        }
        Class::Gunslinger => {
            skills.contains_key(id!(GunslingerSkills::Sharpshooter)) 
                .then(|| "Peacemaker")
                .unwrap_or_else(|| "Time to Hunt")
        }
        Class::Artist => {
            ((skills.contains_key(id!(ArtistSkills::PaintDrawingOrchids))
            || skills.contains_key(id!(ArtistSkills::PaintStarryNight))
            || skills.contains_key(id!(ArtistSkills::PaintIllusionDoor)))
            && skills.contains_key(id!(ArtistSkills::PaintSunsketch))
            && skills.contains_key(id!(ArtistSkills::PaintSunWell))
            && !skills.contains_key(id!(ArtistSkills::PaintCattleDrive)))
                .then(|| "Full Bloom")
                .unwrap_or_else(|| "Recurrence")
        }
        Class::Aeromancer => {
            (skills.contains_key(id!(AeromancerSkills::WindGimlet))
                && skills.contains_key(id!(AeromancerSkills::PiercingWind)))
                .then(|| "Wind Fury")
                .unwrap_or_else(|| "Drizzle")
        }
        Class::Wildsoul => {
            (skills.contains_key(id!(WildsoulSkills::ForbiddenSorceryRippingBear))
                || skills.contains_key(id!(WildsoulSkills::ForbiddenSorceryFoxStarRainstorm)))
                .then(|| "Ferality")
                .unwrap_or_else(|| "Phantom Beast Awakening")
        }
        _ => "Unknown",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hashbrown::HashMap;
    use lost_metrics_core::models::{Class, Skill, StatusEffect};

    fn create_player_skills(skill_ids: Vec<u32>) -> HashMap<u32, Skill> {
        let mut skills = HashMap::new();
        for id in 0..=8u32 {
            let id = *skill_ids.get(id as usize).unwrap_or(&id);
            skills.insert(
                id,
                Skill {
                    id,
                    ..Default::default()
                },
            );
        }
        skills
    }

    fn create_buffed_by(buffs: Vec<(u32, &'static str)>) -> (HashMap<u32, i64>, HashMap<u32, StatusEffect>) {
        let mut buffed_by = HashMap::new();
        let mut buffs_map = HashMap::new();
        for (id, name) in buffs {
            buffed_by.insert(id, 1);
            buffs_map.insert(
                id,
                StatusEffect {
                    source: StatusEffectSource { 
                        name: name.to_string(),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        }
        (buffed_by, buffs_map)
    }

    #[test]
    fn should_return_berserker_technique_for_berserker() {
        let skills = create_player_skills(vec![BerserkerSkills::BloodyRush as u32]);
        let buffs: HashMap<u32, StatusEffect> = HashMap::new();
        let spec = get_player_spec(Class::Berserker, &skills, &HashMap::new(), &buffs);
        assert_eq!(spec, "Berserker Technique");
    }

    #[test]
    fn should_return_loyal_companion_for_sharpshooter() {
        let skills = create_player_skills(vec![]);
        let (buffed_by, buffs) = create_buffed_by(vec![(1000, "Loyal Companion")]);
        let spec = get_player_spec(Class::Sharpshooter, &skills, &buffed_by, &buffs);
        assert_eq!(spec, "Loyal Companion");
    }

    #[test]
    fn should_return_evolutionary_legacy_for_machinist() {
        let skills = create_player_skills(vec![]);
        let (buffed_by, buffs) = create_buffed_by(vec![(3000, "Evolutionary Legacy")]);
        let spec = get_player_spec(Class::Machinist, &skills, &buffed_by, &buffs);
        assert_eq!(spec, "Evolutionary Legacy");
    }

    #[test]
    fn should_return_full_bloom_for_artist() {
        let skills = create_player_skills(vec![
            ArtistSkills::PaintDrawingOrchids as u32,
            ArtistSkills::PaintStarryNight as u32,
            ArtistSkills::PaintIllusionDoor as u32,
            ArtistSkills::PaintSunsketch as u32,
            ArtistSkills::PaintSunWell as u32,
        ]);
        let buffs: HashMap<u32, StatusEffect> = HashMap::new();
        let spec = get_player_spec(Class::Artist, &skills, &HashMap::new(), &buffs);
        assert_eq!(spec, "Full Bloom");
    }
}