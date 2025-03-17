use hashbrown::HashMap;
use lost_metrics_core::models::{EncounterEntity, StatusEffect};

use super::misc::get_buff_names;

pub fn get_player_spec(player: &EncounterEntity, buffs: &HashMap<u32, StatusEffect>) -> String {
    if player.skills.len() < 8 {
        return "Unknown".to_string();
    }

    match player.class.as_str() {
        "Berserker" => {
            if player.skills.contains_key(&16140) {
                "Berserker Technique".to_string()
            } else {
                "Mayhem".to_string()
            }
        }
        "Destroyer" => {
            if player.skills.contains_key(&18090) {
                "Gravity Training".to_string()
            } else {
                "Rage Hammer".to_string()
            }
        }
        "Gunlancer" => {
            if player.skills.contains_key(&17200) && player.skills.contains_key(&17210) {
                "Lone Knight".to_string()
            } else if player.skills.contains_key(&17140) {
                "Combat Readiness".to_string()
            } else {
                "Princess".to_string()
            }
        }
        "Paladin" => {
            if (player.skills.contains_key(&36050)
                || player.skills.contains_key(&36080)
                || player.skills.contains_key(&36150)
                || player.skills.contains_key(&36100))
                && player.skills.contains_key(&36200)
                && player.skills.contains_key(&36170)
            {
                "Blessed Aura".to_string()
            } else {
                "Judgement".to_string()
            }
        }
        "Slayer" => {
            if player.skills.contains_key(&45004) {
                "Punisher".to_string()
            } else {
                "Predator".to_string()
            }
        }
        "Arcanist" => {
            if player.skills.contains_key(&19282) {
                "Order of the Emperor".to_string()
            } else {
                "Grace of the Empress".to_string()
            }
        }
        "Summoner" => {
            if player
                .skills
                .iter()
                .any(|(_, skill)| skill.name.contains("Kelsion"))
            {
                "Communication Overflow".to_string()
            } else {
                "Master Summoner".to_string()
            }
        }
        "Bard" => {
            if player.skills.contains_key(&21250) && player.skills.contains_key(&21080) {
                "Desperate Salvation".to_string()
            } else {
                "True Courage".to_string()
            }
        }
        "Sorceress" => {
            if player.skills.contains_key(&37350)
                && player.skills.contains_key(&37270)
                && player.skills.contains_key(&37330)
            {
                "Igniter".to_string()
            } else {
                "Reflux".to_string()
            }
        }
        "Wardancer" => {
            if player.skills.contains_key(&22340) {
                "Esoteric Skill Enhancement".to_string()
            } else {
                "First Intention".to_string()
            }
        }
        "Scrapper" => {
            if player.skills.contains_key(&23230) {
                "Ultimate Skill: Taijutsu".to_string()
            } else {
                "Shock Training".to_string()
            }
        }
        "Soulfist" => {
            if player.skills.contains_key(&24200) {
                "Energy Overflow".to_string()
            } else {
                "Robust Spirit".to_string()
            }
        }
        "Glaivier" => {
            if player.skills.contains_key(&34590) {
                "Pinnacle".to_string()
            } else {
                "Control".to_string()
            }
        }
        "Striker" => {
            if player.skills.contains_key(&39110) {
                "Esoteric Flurry".to_string()
            } else {
                "Deathblow".to_string()
            }
        }
        "Breaker" => {
            if player.skills.contains_key(&47020) {
                "Asura's Path".to_string()
            } else {
                "Brawl King Storm".to_string()
            }
        }
        "Deathblade" => {
            if player.skills.contains_key(&25038) {
                "Surge".to_string()
            } else {
                "Remaining Energy".to_string()
            }
        }
        "Shadowhunter" => {
            if player.skills.contains_key(&27860) {
                "Demonic Impulse".to_string()
            } else {
                "Perfect Suppression".to_string()
            }
        }
        "Reaper" => {
            let buff_names = get_buff_names(player, buffs);
            if buff_names.iter().any(|s| s.contains("Lunar Voice")) {
                "Lunar Voice".to_string()
            } else {
                "Hunger".to_string()
            }
        }
        "Souleater" => {
            if player.skills.contains_key(&46250) {
                "Night's Edge".to_string()
            } else {
                "Full Moon Harvester".to_string()
            }
        }
        "Sharpshooter" => {
            let buff_names = get_buff_names(player, buffs);
            if buff_names
                .iter()
                .any(|s| s.contains("Loyal Companion") || s.contains("Hawk Support"))
            {
                "Loyal Companion".to_string()
            } else {
                "Death Strike".to_string()
            }
        }
        "Deadeye" => {
            if player.skills.contains_key(&29300) {
                "Enhanced Weapon".to_string()
            } else {
                "Pistoleer".to_string()
            }
        }
        "Artillerist" => {
            if player.skills.contains_key(&30260) {
                "Barrage Enhancement".to_string()
            } else {
                "Firepower Enhancement".to_string()
            }
        }
        "Machinist" => {
            let buff_names = get_buff_names(player, buffs);
            if buff_names
                .iter()
                .any(|s| s.contains("Combat Mode") || s.contains("Evolutionary Legacy"))
            {
                "Evolutionary Legacy".to_string()
            } else {
                "Arthetinean Skill".to_string()
            }
        }
        "Gunslinger" => {
            if player.skills.contains_key(&38110) {
                "Peacemaker".to_string()
            } else {
                "Time to Hunt".to_string()
            }
        }
        "Artist" => {
            if player.skills.contains_key(&31400)
                && player.skills.contains_key(&31410)
                && player.skills.contains_key(&31420)
            {
                "Full Bloom".to_string()
            } else {
                "Recurrence".to_string()
            }
        }
        "Aeromancer" => {
            if player.skills.contains_key(&32250) && player.skills.contains_key(&32260) {
                "Wind Fury".to_string()
            } else {
                "Drizzle".to_string()
            }
        }
        "Wildsoul" => {
            if player.skills.contains_key(&33400) || player.skills.contains_key(&33410) {
                "Ferality".to_string()
            } else {
                "Phantom Beast Awakening".to_string()
            }
        }
        _ => "Unknown".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use lost_metrics_core::models::Skill;

    use crate::constants::WildsoulSkills;

    use super::*;

    fn creater_player_stats(class: String) -> EncounterEntity {

        let mut stats = EncounterEntity {
            class,
            skills: hashbrown::HashMap::new(),
            ..Default::default()
        };

        for id in 1..=7 {
            let skill = Skill {
                id,
                ..Default::default()    
            };
            stats.skills.insert(skill.id, skill);
        }

        stats
    }

    #[test]
    fn should_return_ferality_for_wildsoul() {
       
        let mut player = creater_player_stats("Wildsoul".into());
        let skill = Skill {
            id: WildsoulSkills::ForbiddenSorceryRippingBear as u32,
            ..Default::default()    
        };
        player.skills.insert(skill.id, skill);

        let buffs: HashMap<u32, StatusEffect> = HashMap::new();

        let spec = get_player_spec(&player, &buffs);

        assert_eq!(spec, "Ferality");
    }
}