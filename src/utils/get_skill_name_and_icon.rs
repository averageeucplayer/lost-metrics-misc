use lost_metrics_data::{SKILL_DATA, SKILL_EFFECT_DATA};
use moka::sync::Cache;

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
    
    #[test]
    fn test() {
    }
}