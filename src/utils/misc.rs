use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::io::Write;
use flate2::write::GzEncoder;
use flate2::Compression;
use hashbrown::HashMap;

use lost_metrics_core::models::*;
use lost_metrics_data::*;
use serde::Serialize;

use super::{calculate_average_dps, gem_skill_id_to_skill_ids, get_player_spec, get_spec_from_ark_passive};
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

pub const WINDOW_MS: i64 = 5_000;
pub const WINDOW_S: i64 = 5;

pub fn create_identity_logs_for_local_player(
    identity_log: &IdentityLog,
    class: &str,
    fight_start: i64) -> String {
  
    let mut total_identity_gain = 0;
    let data = identity_log;
    let duration_seconds = (data[data.len() - 1].0 - data[0].0) / 1000;
    let max = match class {
        "Summoner" => 7_000.0,
        "Souleater" => 3_000.0,
        _ => 10_000.0,
    };

    let stats: String = match class {
        "Arcanist" => {
            let mut cards: HashMap<u32, u32> = HashMap::new();
            let mut log: Vec<(i32, (f32, u32, u32))> = Vec::new();
            for i in 1..data.len() {
                let (t1, prev) = data[i - 1];
                let (t2, curr) = data[i];

                // don't count clown cards draws as card draws
                if curr.1 != 0 && curr.1 != prev.1 && prev.1 != 19284 {
                    cards.entry(curr.1).and_modify(|e| *e += 1).or_insert(1);
                }
                if curr.2 != 0 && curr.2 != prev.2 && prev.2 != 19284 {
                    cards.entry(curr.2).and_modify(|e| *e += 1).or_insert(1);
                }

                if t2 > t1 && curr.0 > prev.0 {
                    total_identity_gain += curr.0 - prev.0;
                }

                let relative_time = ((t2 - fight_start) as f32 / 1000.0) as i32;
                // calculate percentage, round to 2 decimal places
                let percentage = if curr.0 >= max as u32 {
                    100.0
                } else {
                    (((curr.0 as f32 / max) * 100.0) * 100.0).round() / 100.0
                };
                log.push((relative_time, (percentage, curr.1, curr.2)));
            }

            let avg_per_s = (total_identity_gain as f64 / duration_seconds as f64)
                / max as f64
                * 100.0;
            let identity_stats = IdentityArcanist {
                average: avg_per_s,
                card_draws: cards,
                log,
            };

            serde_json::to_string(&identity_stats).unwrap()
        }
        "Artist" | "Bard" => {
            let mut log: Vec<(i32, (f32, u32))> = Vec::new();

            for i in 1..data.len() {
                let (t1, i1) = data[i - 1];
                let (t2, i2) = data[i];

                if t2 <= t1 {
                    continue;
                }

                if i2.0 > i1.0 {
                    total_identity_gain += i2.0 - i1.0;
                }

                let relative_time = ((t2 - fight_start) as f32 / 1000.0) as i32;
                // since bard and artist have 3 bubbles, i.1 is the number of bubbles
                // we scale percentage to 3 bubbles
                // current bubble + max * number of bubbles
                let percentage: f32 =
                    ((((i2.0 as f32 + max * i2.1 as f32) / max) * 100.0) * 100.0)
                        .round()
                        / 100.0;
                log.push((relative_time, (percentage, i2.1)));
            }

            let avg_per_s = (total_identity_gain as f64 / duration_seconds as f64)
                / max as f64
                * 100.0;
            let identity_stats = IdentityArtistBard {
                average: avg_per_s,
                log,
            };
            serde_json::to_string(&identity_stats).unwrap()
        }
        _ => {
            let mut log: Vec<(i32, f32)> = Vec::new();
            for i in 1..data.len() {
                let (t1, i1) = data[i - 1];
                let (t2, i2) = data[i];

                if t2 <= t1 {
                    continue;
                }

                if i2.0 > i1.0 {
                    total_identity_gain += i2.0 - i1.0;
                }

                let relative_time = ((t2 - fight_start) as f32 / 1000.0) as i32;
                let percentage =
                    (((i2.0 as f32 / max) * 100.0) * 100.0).round() / 100.0;
                log.push((relative_time, percentage));
            }

            let avg_per_s = (total_identity_gain as f64 / duration_seconds as f64)
                / max as f64
                * 100.0;
            let identity_stats = IdentityGeneric {
                average: avg_per_s,
                log,
            };

            serde_json::to_string(&identity_stats).unwrap()
        }
    };

    stats
}

pub fn calculate_dps_rolling_10s_avg(intervals: &[i64], damage_log: &[(i64, i64)], fight_start: i64) -> Vec<i64> {
    let mut dps_rolling_10s_avg = vec![];

    for interval in intervals {
        let start = fight_start + interval - WINDOW_MS;
        let end = fight_start + interval + WINDOW_MS;

        let damage = sum_in_range(damage_log, start, end);
        dps_rolling_10s_avg.push(damage / (WINDOW_S * 2));
    }

    dps_rolling_10s_avg
}

pub fn create_stagger_stats(
    stagger_log: Vec<(i32, f32)>,
    encounter: &Encounter,
    prev_stagger: i32,
    stagger_intervals: &mut Vec<(i32, i32)>) -> Option<StaggerStats> {
    let mut stagger_stats: Option<StaggerStats> = None;

    if stagger_log.is_empty() {
        return None;
    }

    if prev_stagger > 0 && prev_stagger != encounter.encounter_damage_stats.max_stagger {
        // never finished staggering the boss, calculate average from whatever stagger has been done
        let stagger_start_s = ((encounter.encounter_damage_stats.stagger_start
            - encounter.fight_start)
            / 1000) as i32;
        let stagger_duration = stagger_log.last().unwrap().0 - stagger_start_s;
        if stagger_duration > 0 {
            stagger_intervals.push((stagger_duration, prev_stagger));
        }
    }

    let (total_stagger_time, total_stagger_dealt) = stagger_intervals.iter().fold(
        (0, 0),
        |(total_time, total_stagger), (time, stagger)| {
            (total_time + time, total_stagger + stagger)
        },
    );

    if total_stagger_time > 0 {
        let stagger = StaggerStats {
            average: (total_stagger_dealt as f64 / total_stagger_time as f64)
                / encounter.encounter_damage_stats.max_stagger as f64
                * 100.0,
            staggers_per_min: (total_stagger_dealt as f64 / (total_stagger_time as f64 / 60.0))
                / encounter.encounter_damage_stats.max_stagger as f64,
            log: stagger_log,
        };
        stagger_stats = Some(stagger);
    }

    stagger_stats
}

pub fn update_skill_cast_log(
    entity_id: u64,
    skills: &mut HashMap<u32, Skill>,
    skill_cast_log: &HashMap<u64, HashMap<u32, BTreeMap<i64, SkillCast>>>,) {
    for (_, skill_cast_log) in skill_cast_log.iter().filter(|&(s, _)| *s == entity_id) {
        for (skill, log) in skill_cast_log {
            skills.entry(*skill).and_modify(|e| {
                let average_cast = e.total_damage as f64 / e.casts as f64;
                let filter = average_cast * 0.05;
                let mut adj_hits = 0;
                let mut adj_crits = 0;
                for cast in log.values() {
                    for hit in cast.hits.iter() {
                        if hit.damage as f64 > filter {
                            adj_hits += 1;
                            if hit.crit {
                                adj_crits += 1;
                            }
                        }
                    }
                }

                if adj_hits > 0 {
                    e.adjusted_crit = Some(adj_crits as f64 / adj_hits as f64);
                }

                e.max_damage_cast = log
                    .values()
                    .map(|cast| cast.hits.iter().map(|hit| hit.damage).sum::<i64>())
                    .max()
                    .unwrap_or_default();
                e.skill_cast_log = log
                    .iter()
                    .map(|(_, skill_casts)| skill_casts.clone())
                    .collect();
            });
        }
    }
}

pub fn update_player_stats(
    entity: &mut EncounterEntity,
    player_info: Option<&HashMap<String, PlayerStats>>,
    damage_log: &HashMap<String, Vec<(i64, i64)>>,
    encounter_damage_stats: &EncounterDamageStats,
    fight_start: i64,
    last_combat_packet: i64,
    intervals: &[i64],
) {
    if let Some(damage_log) = damage_log.get(&entity.name) {
        if !&intervals.is_empty() {
            entity.damage_stats.dps_rolling_10s_avg = calculate_dps_rolling_10s_avg(intervals, &damage_log, fight_start);
        }

        let fight_start_sec = fight_start / 1000;
        let fight_end_sec = last_combat_packet / 1000;
        entity.damage_stats.dps_average =
            calculate_average_dps(damage_log, fight_start_sec, fight_end_sec);
    }

    let spec = get_player_spec(entity, &encounter_damage_stats.buffs);

    entity.spec = Some(spec.clone());
    let player_stats = player_info
        .and_then(|stats| stats.get(&entity.name));

    if let Some(info) = player_stats
    {
        for gem in info.gems.iter().flatten() {
            for skill_id in gem_skill_id_to_skill_ids(gem.skill_id) {
                if let Some(skill) = entity.skills.get_mut(&skill_id) {
                    match gem.gem_type {
                        5 | 34 => {
                            // damage gem
                            skill.gem_damage =
                                Some(damage_gem_value_to_level(gem.value, gem.tier));
                            skill.gem_tier_dmg = Some(gem.tier);
                        }
                        27 | 35 => {
                            // cooldown gem
                            skill.gem_cooldown =
                                Some(cooldown_gem_value_to_level(gem.value, gem.tier));
                            skill.gem_tier = Some(gem.tier);
                        }
                        64 | 65 => {
                            // support identity gem??
                            skill.gem_damage =
                                Some(support_damage_gem_value_to_level(gem.value));
                            skill.gem_tier_dmg = Some(gem.tier);
                        }
                        _ => {}
                    }
                }
            }
        }

        entity.ark_passive_active = Some(info.ark_passive_enabled);

        let (class, other) = get_engravings(entity.class_id, &info.engravings);
        entity.engraving_data = other;
        if info.ark_passive_enabled {
            if spec == "Unknown" {
                // not reliable enough to be used on its own
                if let Some(tree) = info.ark_passive_data.as_ref() {
                    if let Some(enlightenment) = tree.enlightenment.as_ref() {
                        for node in enlightenment.iter() {
                            let spec = get_spec_from_ark_passive(node);
                            if spec != "Unknown" {
                                entity.spec = Some(spec);
                                break;
                            }
                        }
                    }
                }
            }
            entity.ark_passive_data = info.ark_passive_data.clone();
        } else if class.len() == 1 {
            entity.spec = Some(class[0].clone());
        }
    }
}
