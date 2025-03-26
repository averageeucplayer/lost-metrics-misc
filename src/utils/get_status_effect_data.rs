use lost_metrics_core::models::{StatusEffect, StatusEffectSource, StatusEffectTarget};
use lost_metrics_data::{SKILL_BUFF_DATA, SKILL_DATA, SKILL_EFFECT_DATA};

use super::{get_status_effect_buff_type_flags, set_summon_source_skill};


pub fn get_status_effect_data(buff_id: u32, source_skill: Option<u32>) -> Option<StatusEffect> {
    let buff = SKILL_BUFF_DATA.get(&buff_id);
    if buff.is_none() || buff.unwrap().icon_show_type.clone().unwrap_or_default() == "none" {
        return None;
    }

    let buff = buff.unwrap();
    let buff_category = if buff.buff_category.clone().unwrap_or_default() == "ability"
        && [501, 502, 503, 504, 505].contains(&buff.unique_group)
    {
        "dropsofether".to_string()
    } else {
        buff.buff_category.clone().unwrap_or_default()
    };
    let mut status_effect = StatusEffect {
        target: {
            if buff.target == "none" {
                StatusEffectTarget::OTHER
            } else if buff.target == "self" {
                StatusEffectTarget::SELF
            } else {
                StatusEffectTarget::PARTY
            }
        },
        category: buff.category.clone(),
        buff_category: buff_category.clone(),
        buff_type: get_status_effect_buff_type_flags(buff),
        unique_group: buff.unique_group,
        source: StatusEffectSource {
            name: buff.name.clone()?,
            desc: buff.desc.clone()?,
            icon: buff.icon.clone()?,
            ..Default::default()
        },
    };

    if buff_category == "classskill"
        || buff_category == "arkpassive"
        || buff_category == "identity"
        || (buff_category == "ability" && buff.unique_group != 0)
    {
        if let Some(buff_source_skills) = buff.source_skills.as_ref() {
            if let Some(source_skill) = source_skill {
                let skill = SKILL_DATA.get(&source_skill);
                set_summon_source_skill(skill, &mut status_effect);
            } else {
                let source_skill = buff_source_skills.first().unwrap_or(&0);
                let skill = SKILL_DATA.get(source_skill);
                set_summon_source_skill(skill, &mut status_effect);
            }
        } else if let Some(buff_source_skill) = SKILL_DATA.get(&(buff_id / 10)) {
            status_effect.source.skill = Some(buff_source_skill.clone());
        } else if let Some(buff_source_skill) = SKILL_DATA.get(&((buff_id / 100) * 10)) {
            status_effect.source.skill = Some(buff_source_skill.clone());
        } else {
            let skill_id = buff.unique_group / 10;
            let buff_source_skill = SKILL_DATA.get(&skill_id);
            status_effect.source.skill = buff_source_skill.cloned();
        }
    } else if buff_category == "set" && buff.set_name.is_some() {
        status_effect.source.set_name.clone_from(&buff.set_name);
    } else if buff_category == "battleitem" {
        if let Some(buff_source_item) = SKILL_EFFECT_DATA.get(&buff_id) {
            if let Some(item_name) = buff_source_item.item_name.as_ref() {
                status_effect.source.name.clone_from(item_name);
            }
            if let Some(item_desc) = buff_source_item.item_desc.as_ref() {
                status_effect.source.desc.clone_from(item_desc);
            }
            if let Some(icon) = buff_source_item.icon.as_ref() {
                status_effect.source.icon.clone_from(icon);
            }
        }
    }

    Some(status_effect)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_drops_of_ether_buff() {
        let status_effect = get_status_effect_data(702, None).unwrap();
        assert_eq!(status_effect.buff_category, "dropsofether");
    }

    #[test]
    fn should_return_self_buff() {
        let status_effect = get_status_effect_data(391604, None).unwrap();
        assert_eq!(status_effect.target, StatusEffectTarget::SELF);
    }

    #[test]
    fn should_return_party_buff() {
        let status_effect = get_status_effect_data(500153, None).unwrap();
        assert_eq!(status_effect.target, StatusEffectTarget::PARTY);
    }
}