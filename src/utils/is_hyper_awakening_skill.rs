
pub fn is_hyper_awakening_skill(skill_id: u32) -> bool {
    matches!(
        skill_id,
        16720 | 16730 // berserker
            | 18240 | 18250 // destroyer
            | 17250 | 17260 // gunlancer
            | 36230 | 36240 // paladin
            | 45820 | 45830 // slayer
            | 19360 | 19370 // arcanist
            | 20370 | 20350 // summoner
            | 21320 | 21330 // bard
            | 37380 | 37390 // sorceress
            | 22360 | 22370 // wardancer
            | 23400 | 23410 // scrapper
            | 24300 | 24310 // soulfist
            | 34620 | 34630 // glaivier
            | 39340 | 39350 // striker
            | 47300 | 47310 // breaker
            | 25410 | 25420 // deathblade
            | 28260 | 28270 // sharpshooter
            | 27910 | 27920 // shadowhunter
            | 26940 | 26950 // reaper
            | 46620 | 46630 // souleater
            | 29360 | 29370 // deadeye
            | 30320 | 30330 // artillerist
            | 35810 | 35820 // machinist
            | 38320 | 38330 // gunslinger
            | 31920 | 31930 // artist
            | 32290 | 32300 // aeromancer
            | 33520 | 33530 // wildsoul
    )
}

#[cfg(test)]
mod tests {
    
    #[test]
    fn test() {
    }
}