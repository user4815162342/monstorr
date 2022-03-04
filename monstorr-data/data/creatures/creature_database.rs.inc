// This file was automatically generated, do not edit. Use `build-data.sh` to regenerate
pub const STORED_CREATURES: [(CreatureSummary<&'static str>, &'static str); 4] = [
        (
            CreatureSummary {
                name: "Efreeti",
                slug: "efreeti",
                type_: "elemental",
                subtype: None,
                size: "Large",
                alignment: "lawful evil",
                challenge_rating: "11",
             },
             include_str!("efreeti.creature")
        ),
        (
            CreatureSummary {
                name: "Adult Gold Dragon",
                slug: "adult-gold-dragon",
                type_: "dragon",
                subtype: None,
                size: "Huge",
                alignment: "lawful good",
                challenge_rating: "17",
             },
             include_str!("adult_gold_dragon.creature")
        ),
        (
            CreatureSummary {
                name: "Goblin",
                slug: "goblin",
                type_: "humanoid",
                subtype: Some("goblinoid"),
                size: "Small",
                alignment: "neutral evil",
                challenge_rating: "1/4",
             },
             include_str!("goblin.creature")
        ),
        (
            CreatureSummary {
                name: "Bugbear",
                slug: "bugbear",
                type_: "humanoid",
                subtype: Some("goblinoid"),
                size: "Medium",
                alignment: "chaotic evil",
                challenge_rating: "1",
             },
             include_str!("bugbear.creature")
        ),
    ];