

pub fn is_class_engraving(class_id: u32, engraving_id: u32) -> bool {
    match engraving_id {
        125 | 188 => class_id == 102, // mayhem, berserker's technique
        196 | 197 => class_id == 103, // rage hammer, gravity training
        224 | 225 => class_id == 104, // combat readiness, lone knight
        282 | 283 => class_id == 105, // judgement, blessed aura
        309 | 320 => class_id == 112, // predator, punisher
        200 | 201 => class_id == 202, // empress's grace, order of the emperor
        198 | 199 => class_id == 203, // master summoner, communication overflow
        194 | 195 => class_id == 204, // true courage, desperate salvation
        293 | 294 => class_id == 205, // igniter, reflux
        189 | 127 => class_id == 302, // first intention, esoteric skill enhancement
        190 | 191 => class_id == 303, // ultimate skill: taijutsu, shock training
        256 | 257 => class_id == 304, // energy overflow, robust spirit
        276 | 277 => class_id == 305, // pinnacle, control
        291 | 292 => class_id == 312, // deathblow, esoteric flurry
        314 | 315 => class_id == 313, // brawl king storm, asura's path
        278 | 279 => class_id == 402, // remaining energy, surge
        280 | 281 => class_id == 403, // perfect suppression, demonic impulse
        286 | 287 => class_id == 404, // hunger, lunar voice
        311 | 312 => class_id == 405, // full moon harvester, night's edge
        258 | 259 => class_id == 502, // loyal companion, death strike
        192 | 129 => class_id == 503, // pistoleer, enhanced weapon
        130 | 193 => class_id == 504, // firepower enhancement, barrage enhancement
        284 | 285 => class_id == 505, // arthetinean skill, evolutionary legacy
        289 | 290 => class_id == 512, // peacemaker, time to hunt
        305 | 306 => class_id == 602, // recurrence, full bloom
        307 | 308 => class_id == 603, // wind fury, drizzle
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    
    #[test]
    fn test() {
    }
}