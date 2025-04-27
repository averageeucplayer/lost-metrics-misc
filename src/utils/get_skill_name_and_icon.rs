use lost_metrics_data::{SKILL_DATA, SKILL_EFFECT_DATA};
use moka::sync::Cache;

pub fn get_basic_skill_name_and_icon(
    skill_id: &u32,
    skill_name: String,
    skill_timestamp: &Cache<(u64, u32), i64>,
    entity_id: u64,
) -> (String, String, Option<Vec<u32>>) {
    if let Some(skill) = SKILL_DATA.get(skill_id) {
        if let Some(summon_source_skill) = skill.summon_source_skills.as_ref() {
            for source in summon_source_skill {
                if skill_timestamp.get(&(entity_id, *source)).is_some()
                {
                    if let Some(skill) = SKILL_DATA.get(source) {
                        return (
                            skill.name.clone().unwrap_or_default() + " (Summon)",
                            skill.icon.clone().unwrap_or_default(),
                            Some(summon_source_skill.clone()),
                        );
                    }
                }
            }
            if let Some(skill) = SKILL_DATA.get(summon_source_skill.iter().min().unwrap_or(&0))
            {
                (
                    skill.name.clone().unwrap_or_default() + " (Summon)",
                    skill.icon.clone().unwrap_or_default(),
                    Some(summon_source_skill.clone()),
                )
            } else {
                (skill_name, "".to_string(), None)
            }
        } else if let Some(source_skill) = skill.source_skills.as_ref() {
            if let Some(skill) = SKILL_DATA.get(source_skill.iter().min().unwrap_or(&0)) {
                (
                    skill.name.clone().unwrap_or_default(),
                    skill.icon.clone().unwrap_or_default(),
                    None,
                )
            } else {
                (skill_name, "".to_string(), None)
            }
        } else {
            (
                skill.name.clone().unwrap_or_default(),
                skill.icon.clone().unwrap_or_default(),
                None,
            )
        }
    } else if let Some(skill) = SKILL_DATA.get(&(skill_id - (skill_id % 10))) {
        (
            skill.name.clone().unwrap_or_default(),
            skill.icon.clone().unwrap_or_default(),
            None,
        )
    } else {
        (skill_name, "".to_string(), None)
    }
}

pub fn get_skill_name_and_icon(
    skill_id: &u32,
    skill_effect_id: &u32,
    skill_name: String,
    skill_timestamp: &Cache<(u64, u32), i64>,
    entity_id: u64,
) -> (String, String, Option<Vec<u32>>) {
    if (*skill_id == 0) && (*skill_effect_id == 0) {
        ("Bleed".to_string(), "buff_168.png".to_string(), None)
    } else if (*skill_effect_id != 0) && (*skill_effect_id == *skill_id) {
        return if let Some(effect) = SKILL_EFFECT_DATA.get(skill_effect_id) {
            if let Some(item_name) = effect.item_name.as_ref() {
                return (
                    item_name.clone(),
                    effect.icon.as_ref().cloned().unwrap_or_default(),
                    None,
                );
            }
            if let Some(source_skill) = effect.source_skills.as_ref() {
                if let Some(skill) = SKILL_DATA.get(source_skill.iter().min().unwrap_or(&0)) {
                    return (
                        skill.name.clone().unwrap_or_default(),
                        skill.icon.clone().unwrap_or_default(),
                        None,
                    );
                }
            } else if let Some(skill) = SKILL_DATA.get(&(skill_effect_id / 10)) {
                return (
                    skill.name.clone().unwrap_or_default(),
                    skill.icon.clone().unwrap_or_default(),
                    None,
                );
            }
            (effect.comment.clone(), "".to_string(), None)
        } else {
            (skill_name, "".to_string(), None)
        };
    } else {
        return if let Some(skill) = SKILL_DATA.get(skill_id) {
            if let Some(summon_source_skill) = skill.summon_source_skills.as_ref() {
                for source in summon_source_skill {
                    if skill_timestamp.get(&(entity_id, *source)).is_some()
                    {
                        if let Some(skill) = SKILL_DATA.get(source) {
                            return (
                                skill.name.clone().unwrap_or_default() + " (Summon)",
                                skill.icon.clone().unwrap_or_default(),
                                Some(summon_source_skill.clone()),
                            );
                        }
                    }
                }
                if let Some(skill) = SKILL_DATA.get(summon_source_skill.iter().min().unwrap_or(&0))
                {
                    (
                        skill.name.clone().unwrap_or_default() + " (Summon)",
                        skill.icon.clone().unwrap_or_default(),
                        Some(summon_source_skill.clone()),
                    )
                } else {
                    (skill_name, "".to_string(), None)
                }
            } else if let Some(source_skill) = skill.source_skills.as_ref() {
                if let Some(skill) = SKILL_DATA.get(source_skill.iter().min().unwrap_or(&0)) {
                    (
                        skill.name.clone().unwrap_or_default(),
                        skill.icon.clone().unwrap_or_default(),
                        None,
                    )
                } else {
                    (skill_name, "".to_string(), None)
                }
            } else {
                (
                    skill.name.clone().unwrap_or_default(),
                    skill.icon.clone().unwrap_or_default(),
                    None,
                )
            }
        } else if let Some(skill) = SKILL_DATA.get(&(skill_id - (skill_id % 10))) {
            (
                skill.name.clone().unwrap_or_default(),
                skill.icon.clone().unwrap_or_default(),
                None,
            )
        } else {
            (skill_name, "".to_string(), None)
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_bleed_skill() {
        let skill_info = get_skill_name_and_icon(&0, &0, "".into(), &Cache::new(0), 0);

        assert_eq!(skill_info.0, "Bleed");
    }

    #[test]
    fn should_return_consumable_skill() {
        let skill_info = get_skill_name_and_icon(&9099, &9099, "".into(), &Cache::new(0), 0);

        assert_eq!(skill_info.0, "Peach Wine");
    }

    #[test]
    fn should_return_main_skill() {
        let skill_info = get_skill_name_and_icon(&170300, &170300, "".into(), &Cache::new(0), 0);

        assert_eq!(skill_info.0, "Sharp Gunlance");
    }

    #[test]
    fn should_return_related_div10_skill() {

        // for (id, sed) in SKILL_EFFECT_DATA.iter() {
        //     let custom_id = id / 10;
            
        //     if SKILL_DATA.get(&custom_id).filter(|sk| sk.name.is_some()).is_some() {
        //         println!("{id} - {custom_id}");
        //     }
        // }

        let skill_info = get_skill_name_and_icon(&471802, &471802, "".into(), &Cache::new(0), 0);

        assert_eq!(skill_info.0, "Punishing Wave");
    }

    #[test]
    fn should_return_cached_summon_source_skill() {
        let cache: Cache<(u64, u32), i64> = Cache::new(0);
        cache.insert((0, 20090), 1);
        let skill_info = get_skill_name_and_icon(&20095, &0, "".into(), &cache, 0);

        assert_eq!(skill_info.0, "Elcid (Summon)");
    }

    #[test]
    fn should_return_summon_source_skill() {
        let cache: Cache<(u64, u32), i64> = Cache::new(0);
        let skill_info = get_skill_name_and_icon(&20291, &0, "".into(), &cache, 0);

        assert_eq!(skill_info.0, "Kelsion (Summon)");
    }

    #[test]
    fn should_return_source_skill() {
        let cache: Cache<(u64, u32), i64> = Cache::new(0);
        let skill_info = get_skill_name_and_icon(&24280, &0, "".into(), &cache, 0);

        assert_eq!(skill_info.0, "Weapon Attack");
    }
    
    #[test]
    fn should_return_skill() {
        let cache: Cache<(u64, u32), i64> = Cache::new(0);
        let skill_info = get_skill_name_and_icon(&17140, &0, "".into(), &cache, 0);

        assert_eq!(skill_info.0, "Guardian's Thundercrack");
    }
     
    #[test]
    fn should_return_related_mod10_skill() {
        
        // for skill_id in 1..100000 {
        //     let custom_id = skill_id - (skill_id % 10);
        //     if let Some(skill) = SKILL_DATA.get(&custom_id).filter(|sk| sk.name.is_some()) {
        //         println!("{:?} {skill_id} - {custom_id}", skill.name);
        //     }
        // }

        let cache: Cache<(u64, u32), i64> = Cache::new(0);
        let skill_info = get_skill_name_and_icon(&47109, &0, "".into(), &cache, 0);

        assert_eq!(skill_info.0, "Brawl King's Advance");
    }
   
}