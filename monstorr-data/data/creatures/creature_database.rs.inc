// This file was automatically generated, do not edit. Use `build-data.sh` to regenerate
pub const STORED_CREATURES: [(CreatureSummary<&'static str>, &'static str); 13] = [
        (
            CreatureSummary {
                name: "Awakened Tree",
                slug: "awakened-tree",
                type_: "plant",
                subtype: None,
                size: "Huge",
                alignment: "unaligned",
                challenge_rating: "2",
            },
            include_str!("awakened_tree.creature")
        ),(
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
        (
            CreatureSummary {
                name: "Commoner",
                slug: "commoner",
                type_: "humanoid",
                subtype: Some("any race"),
                size: "Medium",
                alignment: "any alignment",
                challenge_rating: "0",
            },
            include_str!("commoner.creature")
        ),(
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
                name: "Giant Frog",
                slug: "giant-frog",
                type_: "beast",
                subtype: None,
                size: "Medium",
                alignment: "unaligned",
                challenge_rating: "1/4",
            },
            include_str!("giant_frog.creature")
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
                name: "Imp",
                slug: "imp",
                type_: "fiend",
                subtype: Some("devil"),
                size: "Tiny",
                alignment: "lawful evil",
                challenge_rating: "1",
            },
            include_str!("imp.creature")
        ),
        (
            CreatureSummary {
                name: "Dust Mephit",
                slug: "dust-mephit",
                type_: "elemental",
                subtype: None,
                size: "Small",
                alignment: "neutral evil",
                challenge_rating: "1/2",
            },
            include_str!("dust_mephit.creature")
        ),
        (
            CreatureSummary {
                name: "Ice Mephit",
                slug: "ice-mephit",
                type_: "elemental",
                subtype: None,
                size: "Small",
                alignment: "neutral evil",
                challenge_rating: "1/2",
            },
            include_str!("ice_mephit.creature")
        ),
        (
            CreatureSummary {
                name: "Magma Mephit",
                slug: "magma-mephit",
                type_: "elemental",
                subtype: None,
                size: "Small",
                alignment: "neutral evil",
                challenge_rating: "1/2",
            },
            include_str!("magma_mephit.creature")
        ),
        (
            CreatureSummary {
                name: "Steam Mephit",
                slug: "steam-mephit",
                type_: "elemental",
                subtype: None,
                size: "Small",
                alignment: "neutral evil",
                challenge_rating: "1/4",
            },
            include_str!("steam_mephit.creature")
        ),
        (
            CreatureSummary {
                name: "Sprite",
                slug: "sprite",
                type_: "fey",
                subtype: None,
                size: "Tiny",
                alignment: "neutral good",
                challenge_rating: "1/4",
            },
            include_str!("sprite.creature")
        )
    ];

