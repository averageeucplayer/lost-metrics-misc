use std::cmp::Ordering;
use std::io::Write;
use flate2::write::GzEncoder;
use flate2::Compression;
use hashbrown::HashMap;

use lost_metrics_core::models::*;
use lost_metrics_data::*;
use serde::Serialize;

use super::is_class_engraving::is_class_engraving;


pub fn get_buff_names(player: &EncounterEntity, buffs: &HashMap<u32, StatusEffect>) -> Vec<String> {
    let mut names = Vec::new();
    for (id, _) in player.damage_stats.buffed_by.iter() {
        if let Some(buff) = buffs.get(id) {
            names.push(buff.source.name.clone());
        }
    }

    names
}

pub fn is_hat_buff(buff_id: &u32) -> bool {
    matches!(buff_id, 362600 | 212305 | 319503)
}

pub fn damage_gem_value_to_level(value: u32, tier: u8) -> u8 {
    if tier == 4 {
        match value {
            4400 => 10,
            4000 => 9,
            3600 => 8,
            3200 => 7,
            2800 => 6,
            2400 => 5,
            2000 => 4,
            1600 => 3,
            1200 => 2,
            800 => 1,
            _ => 0,
        }
    } else {
        match value {
            4000 => 10,
            3000 => 9,
            2400 => 8,
            2100 => 7,
            1800 => 6,
            1500 => 5,
            1200 => 4,
            900 => 3,
            600 => 2,
            300 => 1,
            _ => 0,
        }
    }
}

pub fn cooldown_gem_value_to_level(value: u32, tier: u8) -> u8 {
    if tier == 4 {
        match value {
            2400 => 10,
            2200 => 9,
            2000 => 8,
            1800 => 7,
            1600 => 6,
            1400 => 5,
            1200 => 4,
            1000 => 3,
            800 => 2,
            600 => 1,
            _ => 0,
        }
    } else {
        match value {
            2000 => 10,
            1800 => 9,
            1600 => 8,
            1400 => 7,
            1200 => 6,
            1000 => 5,
            800 => 4,
            600 => 3,
            400 => 2,
            200 => 1,
            _ => 0,
        }
    }
}

pub fn support_damage_gem_value_to_level(value: u32) -> u8 {
    match value {
        1000 => 10,
        900 => 9,
        800 => 8,
        700 => 7,
        600 => 6,
        500 => 5,
        400 => 4,
        300 => 3,
        200 => 2,
        100 => 1,
        _ => 0,
    }
}


pub fn get_engravings(
    class_id: u32,
    engravings: &Option<Vec<u32>>,
) -> (Vec<String>, Option<Vec<String>>) {
    let engravings = match engravings {
        Some(engravings) => engravings,
        None => return (vec![], None),
    };

    let mut class_engravings: Vec<String> = Vec::new();
    let mut other_engravings: Vec<String> = Vec::new();

    for engraving_id in engravings.iter() {
        if let Some(engraving_data) = ENGRAVING_DATA.get(engraving_id) {
            let player_engraving = engraving_data.name.clone();
            if is_class_engraving(class_id, engraving_data.id) {
                class_engravings.push(player_engraving.clone().unwrap_or("Unknown".to_string()));
            } else {
                other_engravings.push(player_engraving.unwrap_or("Unknown".to_string()));
            }
        }
    }

    other_engravings.sort_unstable();
    let sorted_engravings: Vec<String> = class_engravings
        .iter()
        .cloned()
        .chain(other_engravings)
        .collect();

    if sorted_engravings.is_empty() {
        (class_engravings, None)
    } else {
        (class_engravings, Some(sorted_engravings))
    }
}

pub fn is_support_class_id(class_id: u32) -> bool {
    class_id == 105 || class_id == 204 || class_id == 602
}

pub fn is_battle_item(skill_effect_id: &u32, _item_type: &str) -> bool {
    if let Some(item) = SKILL_EFFECT_DATA.get(skill_effect_id) {
        if let Some(category) = item.item_type.as_ref() {
            return category == "useup";
        }
    }
    false
}

pub fn generate_intervals(start: i64, end: i64) -> Vec<i64> {
    if start >= end {
        return Vec::new();
    }

    (0..end - start).step_by(1_000).collect()
}

pub fn sum_in_range(vec: &[(i64, i64)], start: i64, end: i64) -> i64 {
    let start_idx = binary_search_left(vec, start);
    let end_idx = binary_search_left(vec, end + 1);

    vec[start_idx..end_idx]
        .iter()
        .map(|&(_, second)| second)
        .sum()
}

pub fn binary_search_left(vec: &[(i64, i64)], target: i64) -> usize {
    let mut left = 0;
    let mut right = vec.len();

    while left < right {
        let mid = left + (right - left) / 2;
        match vec[mid].0.cmp(&target) {
            Ordering::Less => left = mid + 1,
            _ => right = mid,
        }
    }

    left
}

pub fn set_summon_source_skill(skill: Option<&SkillData>, status_effect: &mut StatusEffect) {
    if let Some(skill) = skill {
        if let Some(summon_skills) = skill.summon_source_skills.as_ref() {
            let summon_source_skill = summon_skills.first().unwrap_or(&0);
            if *summon_source_skill > 0 {
                if let Some(summon_skill) = SKILL_DATA.get(summon_source_skill) {
                    status_effect.source.skill = Some(summon_skill.clone());
                }
            }
        } else {
            status_effect.source.skill = Some(skill.clone());
        }
    }
}

pub fn check_tripod_index_change(before: Option<TripodIndex>, after: Option<TripodIndex>) -> bool {
    if before.is_none() && after.is_none() {
        return false;
    }

    if before.is_none() || after.is_none() {
        return true;
    }

    let before = before.unwrap();
    let after = after.unwrap();

    before != after
}

pub fn check_tripod_level_change(before: Option<TripodLevel>, after: Option<TripodLevel>) -> bool {
    if before.is_none() && after.is_none() {
        return false;
    }

    if before.is_none() || after.is_none() {
        return true;
    }

    let before = before.unwrap();
    let after = after.unwrap();

    before != after
}

pub fn compress_json<T>(value: &T) -> Vec<u8>
    where
        T: ?Sized + Serialize,
{
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    let bytes = serde_json::to_vec(value).expect("unable to serialize json");
    encoder.write_all(&bytes).expect("unable to write json to buffer");
    encoder.finish().expect("unable to compress json")
}

pub fn update_current_boss_name(boss_name: &str) -> String {
    match boss_name {
        "Chaos Lightning Dragon Jade" => "Argeos",
        "Vicious Argeos" | "Ruthless Lakadroff" | "Untrue Crimson Yoho" | "Despicable Skolakia" => {
            "Behemoth, the Storm Commander"
        }
        _ => boss_name,
    }
    .to_string()
}

pub fn get_npc_entity_type_name_grade(object_id: u64, type_id: u32, max_hp: i64) -> (EntityType, String, String) {
    if let Some(esther) = get_esther_from_npc_id(type_id) {
        return (EntityType::Esther, esther.name, "none".to_string());
    }

    if let Some((_, npc_info)) = NPC_DATA.get_key_value(&type_id) {
        let npc_name = npc_info.name.clone().unwrap_or_default();
        if npc_info.is_boss()
            && max_hp > 10_000
            && npc_info.has_valid_name()
        {
            (EntityType::Boss, npc_name.clone(), format!("{}", npc_info.grade))
        } else {
            (EntityType::Npc, npc_name.clone(), format!("{}", npc_info.grade))
        }
    } else {
        (EntityType::Npc, format!("{:x}", object_id), "none".to_string())
    }
}

pub fn get_esther_from_npc_id(npc_id: u32) -> Option<Esther> {
    ESTHER_DATA
        .iter()
        .find(|esther| esther.npc_ids.contains(&npc_id))
        .cloned()
}

#[cfg(test)]
mod tests {
    
    #[test]
    fn test() {
    }
}