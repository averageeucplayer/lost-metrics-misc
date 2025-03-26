
pub fn boss_to_raid_map(boss: &str, max_hp: i64) -> String {
    match boss {
        "Killineza the Dark Worshipper" => "Thaemine G1",
        "Valinak, Knight of Darkness" | "Valinak, Taboo Usurper" | "Valinak, Herald of the End" => {
            "Thaemine G2"
        }
        "Thaemine the Lightqueller" | "Dark Greatsword" => "Thaemine G3",
        "Darkness Legion Commander Thaemine"
        | "Thaemine Prokel"
        | "Thaemine, Conqueror of Stars" => "Thaemine G4",
        "Red Doom Narkiel" | "Agris" => "Echidna G1",
        "Echidna"
        | "Covetous Master Echidna"
        | "Desire in Full Bloom, Echidna"
        | "Alcaone, the Twisted Venom" => "Echidna G2",
        "Behemoth, the Storm Commander"
        | "Despicable Skolakia"
        | "Untrue Crimson Yoho"
        | "Ruthless Lakadroff"
        | "Vicious Argeos" => "Behemoth G1",
        "Behemoth, Cruel Storm Slayer" => "Behemoth G2",
        "Akkan, Lord of Death" | "Abyss Monarch Aegir" => "Aegir G1",
        "Aegir, the Oppressor" | "Pulsating Giant's Heart" => "Aegir G2",
        "Narok the Butcher" => "Act 2: Brelshaza G1",
        "Phantom Manifester Brelshaza" => "Act 2: Brelshaza G2",
        "Phantom Legion Commander Brelshaza" => {
            if max_hp > 100_000_000_000 {
                "Act 2: Brelshaza G2"
            } else {
                "Brelshaza G6"
            }
        }
        _ => boss,
    }
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_handle_old_brelshaza() {
        assert_eq!("Brelshaza G6", boss_to_raid_map("Phantom Legion Commander Brelshaza", 100_000_000_00));
    }

    #[test]
    fn should_return_raid_name() {
        assert_eq!("Act 2: Brelshaza G2", boss_to_raid_map("Phantom Manifester Brelshaza", 100_000_000_001));
    }
}