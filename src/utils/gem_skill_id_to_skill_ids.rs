pub fn gem_skill_id_to_skill_ids(skill_id: u32) -> Vec<u32> {
    match skill_id {
        13000 | 13001 => vec![18011, 18030], // destroyer hypergravity skills
        23000 => vec![
            20311, 20310, 20070, 20071, 20080, 20081, 20170, 20181, 20280, 20281,
        ], // summoner elemental damage
        41000 => vec![25038, 25035, 25036, 25037, 25400, 25401, 25402], // db surge skill
        42000 | 42001 => vec![
            27800, 27030, 27810, 27820, 27830, 27840, 27850, 27860, 27940, 27960,
        ], // sh transformation skills
        51001 => vec![28159, 28160, 28161, 28162, 28170], // sharpshooter bird skill
        53000 | 53001 => vec![30240, 30250, 30260, 30270, 30290], // arty barrage skills
        54000 | 54001 => vec![
            35720, 35750, 35760, 35761, 35770, 35771, 35780, 35781, 35790, 35800,
        ], // machinist transformation skills
        62000 => vec![32040, 32041],         // aeromancer sun shower
        24000 => vec![21140, 21141, 21142, 21143, 21130, 21131, 21132, 21133], // bard serenade skills
        47000 => vec![47950], // bk breaker identity
        60000 => vec![
            31050, 31051, 31110, 31120, 31121, 31130, 31131, 31140, 31141,
        ], // artist moonfall
        19030 => vec![19290, 19030, 19300], // arcana evokes
        63000 | 63001 => vec![33200, 33201], // wildsoul swish bear
        63002 | 63003 => vec![33230, 33231], // wildsoul boulder bear
        63004 | 63005 => vec![33330, 33331], // wildsoul fox leap
        63006 | 63007 => vec![33320, 33321], // wildsoul fox flame
        63008 | 63009 => vec![33400, 33410], // wildsoul identity skills
        _ => vec![skill_id],
    }
}

#[cfg(test)]
mod tests {
    
    #[test]
    fn test() {
    }
}