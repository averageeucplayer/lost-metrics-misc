use lost_metrics_core::models::{SkillBuffData, StatusEffectBuffTypeFlags};
use lost_metrics_data::{COMBAT_EFFECT_DATA, STAT_TYPE_MAP};


pub fn get_status_effect_buff_type_flags(buff: &SkillBuffData) -> u32 {
    let dmg_buffs = [
        "weaken_defense",
        "weaken_resistance",
        "skill_damage_amplify",
        "beattacked_damage_amplify",
        "skill_damage_amplify_attack",
        "directional_attack_amplify",
        "instant_stat_amplify",
        "attack_power_amplify",
        "instant_stat_amplify_by_contents",
        "evolution_type_damage",
    ];

    let mut buff_type = StatusEffectBuffTypeFlags::NONE;
    if dmg_buffs.contains(&buff.buff_type.as_str()) {
        buff_type |= StatusEffectBuffTypeFlags::DMG;
    } else if ["move_speed_down", "all_speed_down"].contains(&buff.buff_type.as_str()) {
        buff_type |= StatusEffectBuffTypeFlags::MOVESPEED;
    } else if buff.buff_type == "reset_cooldown" {
        buff_type |= StatusEffectBuffTypeFlags::COOLDOWN;
    } else if ["change_ai_point", "ai_point_amplify"].contains(&buff.buff_type.as_str()) {
        buff_type |= StatusEffectBuffTypeFlags::STAGGER;
    } else if buff.buff_type == "increase_identity_gauge" {
        buff_type |= StatusEffectBuffTypeFlags::RESOURCE;
    }

    for option in buff.passive_options.iter() {
        let key_stat_str = option.key_stat.as_str();
        let option_type = option.option_type.as_str();
        if option_type == "stat" {
            let stat = STAT_TYPE_MAP.get(key_stat_str);
            if stat.is_none() {
                continue;
            }
            let stat = stat.unwrap().to_owned();
            if ["mastery", "mastery_x", "paralyzation_point_rate"].contains(&key_stat_str) {
                buff_type |= StatusEffectBuffTypeFlags::STAGGER;
            } else if ["rapidity", "rapidity_x", "cooldown_reduction"].contains(&key_stat_str) {
                buff_type |= StatusEffectBuffTypeFlags::COOLDOWN;
            } else if [
                "max_mp",
                "max_mp_x",
                "max_mp_x_x",
                "normal_mp_recovery",
                "combat_mp_recovery",
                "normal_mp_recovery_rate",
                "combat_mp_recovery_rate",
                "resource_recovery_rate",
            ]
            .contains(&key_stat_str)
            {
                buff_type |= StatusEffectBuffTypeFlags::RESOURCE;
            } else if [
                "con",
                "con_x",
                "max_hp",
                "max_hp_x",
                "max_hp_x_x",
                "normal_hp_recovery",
                "combat_hp_recovery",
                "normal_hp_recovery_rate",
                "combat_hp_recovery_rate",
                "self_recovery_rate",
                "drain_hp_dam_rate",
                "vitality",
            ]
            .contains(&key_stat_str)
            {
                buff_type |= StatusEffectBuffTypeFlags::HP;
            } else if STAT_TYPE_MAP["def"] <= stat && stat <= STAT_TYPE_MAP["magical_inc_rate"]
                || ["endurance", "endurance_x"].contains(&option.key_stat.as_str())
            {
                if buff.category == "buff" && option.value >= 0
                    || buff.category == "debuff" && option.value <= 0
                {
                    buff_type |= StatusEffectBuffTypeFlags::DMG;
                } else {
                    buff_type |= StatusEffectBuffTypeFlags::DEFENSE;
                }
            } else if STAT_TYPE_MAP["move_speed"] <= stat
                && stat <= STAT_TYPE_MAP["vehicle_move_speed_rate"]
            {
                buff_type |= StatusEffectBuffTypeFlags::MOVESPEED;
            }
            if [
                "attack_speed",
                "attack_speed_rate",
                "rapidity",
                "rapidity_x",
            ]
            .contains(&key_stat_str)
            {
                buff_type |= StatusEffectBuffTypeFlags::ATKSPEED;
            } else if ["critical_hit_rate", "criticalhit", "criticalhit_x"].contains(&key_stat_str)
            {
                buff_type |= StatusEffectBuffTypeFlags::CRIT;
            } else if STAT_TYPE_MAP["attack_power_sub_rate_1"] <= stat
                && stat <= STAT_TYPE_MAP["skill_damage_sub_rate_2"]
                || STAT_TYPE_MAP["fire_dam_rate"] <= stat
                    && stat <= STAT_TYPE_MAP["elements_dam_rate"]
                || [
                    "str",
                    "agi",
                    "int",
                    "str_x",
                    "agi_x",
                    "int_x",
                    "char_attack_dam",
                    "attack_power_rate",
                    "skill_damage_rate",
                    "attack_power_rate_x",
                    "skill_damage_rate_x",
                    "hit_rate",
                    "dodge_rate",
                    "critical_dam_rate",
                    "awakening_dam_rate",
                    "attack_power_addend",
                    "weapon_dam",
                ]
                .contains(&key_stat_str)
            {
                if buff.category == "buff" && option.value >= 0
                    || buff.category == "debuff" && option.value <= 0
                {
                    buff_type |= StatusEffectBuffTypeFlags::DMG;
                } else {
                    buff_type |= StatusEffectBuffTypeFlags::DEFENSE;
                }
            }
        } else if option_type == "skill_critical_ratio" {
            buff_type |= StatusEffectBuffTypeFlags::CRIT;
        } else if [
            "skill_damage",
            "class_option",
            "skill_group_damage",
            "skill_critical_damage",
            "skill_penetration",
        ]
        .contains(&option_type)
        {
            if buff.category == "buff" && option.value >= 0
                || buff.category == "debuff" && option.value <= 0
            {
                buff_type |= StatusEffectBuffTypeFlags::DMG;
            } else {
                buff_type |= StatusEffectBuffTypeFlags::DEFENSE;
            }
        } else if ["skill_cooldown_reduction", "skill_group_cooldown_reduction"]
            .contains(&option_type)
        {
            buff_type |= StatusEffectBuffTypeFlags::COOLDOWN;
        } else if ["skill_mana_reduction", "mana_reduction"].contains(&option_type) {
            buff_type |= StatusEffectBuffTypeFlags::RESOURCE;
        } else if option_type == "combat_effect" {
            if let Some(combat_effect) = COMBAT_EFFECT_DATA.get(&(option.key_index as u32)) {
                for effect in combat_effect.effects.iter() {
                    for action in effect.actions.iter() {
                        if [
                            "modify_damage",
                            "modify_final_damage",
                            "modify_critical_multiplier",
                            "modify_penetration",
                            "modify_penetration_when_critical",
                            "modify_penetration_addend",
                            "modify_penetration_addend_when_critical",
                            "modify_damage_shield_multiplier",
                        ]
                        .contains(&action.action_type.as_str())
                        {
                            buff_type |= StatusEffectBuffTypeFlags::DMG;
                        } else if action.action_type == "modify_critical_ratio" {
                            buff_type |= StatusEffectBuffTypeFlags::CRIT;
                        }
                    }
                }
            }
        }
    }

    buff_type.bits()
}

#[cfg(test)]
mod tests {
    use lost_metrics_data::SKILL_BUFF_DATA;

    use super::*;

    #[test]
    fn should_have_dmg_bits() {
        let buff_id = 2000362;
        let buff = SKILL_BUFF_DATA.get(&buff_id).unwrap();
        let bits = get_status_effect_buff_type_flags(buff);
        assert_eq!(bits, 1);
    }
}