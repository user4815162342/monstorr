/*
 * Copyright © 2022 Neil M. Sheldon
 * 
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */


    use std::path::PathBuf;

    use pretty_assertions::assert_eq;

    use crate::creature_commands::*;
    use crate::stats::*;
    use crate::features::*;
    use crate::attacks::*;
    use crate::dice::*;
    use crate::actions::*;
    use crate::stat_block::*;
    use crate::structured_text::*;


    fn goblin() -> CreatureCreator {
    
        CreatureCreator(vec![
            CreatureCommand::Monstorr(1.0,None),
            CreatureCommand::Source("D&D 5E System Reference Document".to_owned()),
            CreatureCommand::Name("Goblin".to_owned()),
            CreatureCommand::Small,
            CreatureCommand::Humanoid,
            CreatureCommand::Subtype("goblinoid".to_owned()),
            CreatureCommand::NeutralEvil,
            CreatureCommand::Armor(Armor::Leather),
            CreatureCommand::Shield,
            CreatureCommand::HitDiceCount(2),
            CreatureCommand::Walk(30),
            CreatureCommand::Str(8),
            CreatureCommand::Dex(14),
            //CreatureCommand::Con(10),
            //CreatureCommand::Int(10),
            CreatureCommand::Wis(8),
            CreatureCommand::Cha(8),
            CreatureCommand::Expertise(vec![Skill::Stealth]),
            CreatureCommand::Darkvision(60),
            CreatureCommand::Languages(vec![Language::Common, Language::Goblin]),
            CreatureCommand::OverrideQuarterChallenge,
            CreatureCommand::Feature(Feature::NimbleEscape,None),
            CreatureCommand::Weapon(Weapon::Scimitar(0),None),
            CreatureCommand::Weapon(Weapon::Shortbow(0),None),


        ])
    }

    const GOBLIN: &str = "([
    Monstorr(1),
    Source(\"D&D 5E System Reference Document\"),
    Name(\"Goblin\"),
    Small,
    Humanoid,
    Subtype(\"goblinoid\"),
    NeutralEvil,
    Armor(Leather),
    Shield,
    HitDiceCount(2),
    Walk(30),
    Str(8),
    Dex(14),
    Wis(8),
    Cha(8),
    Expertise([
        Stealth,
    ]),
    Darkvision(60),
    Languages([
        Common,
        Goblin,
    ]),
    OverrideQuarterChallenge,
    Feature(NimbleEscape),
    Weapon(Scimitar(0)),
    Weapon(Shortbow(0)),
])";

    fn goblin_stat_block() -> CreatureStatBlock {
        CreatureStatBlock {
            name: "Goblin".to_owned(),
            size: "Small".to_owned(),
            type_: "humanoid".to_owned(),
            subtype: Some("goblinoid".to_owned()),
            group: None,
            alignment: "neutral evil".to_owned(),
            armor: "15 (leather armor, shield)".to_owned(),
            hit_points: "7 (2d6)".to_owned(),
            speed: "30 ft.".to_owned(),
            strength: "8 (-1)".to_owned(),
            dexterity: "14 (+2)".to_owned(),
            constitution: "10 (+0)".to_owned(),
            intelligence: "10 (+0)".to_owned(),
            wisdom: "8 (-1)".to_owned(),
            charisma: "8 (-1)".to_owned(),
            saving_throws: None,
            skills: Some("Stealth +6".to_owned()),
            damage_vulnerabilities: None, 
            damage_resistances: None,
            damage_immunities: None,
            condition_immunities: None,
            senses: "darkvision 60 ft., passive Perception 9".to_owned(),
            languages: Some("Common, Goblin".to_owned()),
            challenge_rating: "1/4 (50 XP)".to_owned(), 
            special_abilities: vec![
                StatBlockFeature {
                    text: vec![
                        TextBlock::Paragraph {
                            heading: Some(vec![TextSpan::Normal("Nimble Escape.".to_owned())]),
                            body: vec![
                                TextSpan::Normal("The goblin can take the Disengage or Hide action as a bonus action on each of its turns.".to_owned())
                            ]
                        }
                    ]
                }
            ],
            actions: vec![
                StatBlockFeature {
                    text: vec![
                        TextBlock::Paragraph {
                            heading: Some(vec![TextSpan::Normal("Scimitar.".to_owned())]),
                            body: vec![
                                TextSpan::Italic("Melee Weapon Attack:".to_owned()),
                                TextSpan::Normal(" +4 to hit, reach 5 ft., one target. ".to_owned()),
                                TextSpan::Italic("Hit:".to_owned()),
                                TextSpan::Normal(" 5 (1d6 + 2) slashing damage.".to_owned())
                            ]
                        }
                    ]
                },
                StatBlockFeature {
                    text: vec![
                        TextBlock::Paragraph {
                            heading: Some(vec![TextSpan::Normal("Shortbow.".to_owned())]),
                            body: vec![
                                TextSpan::Italic("Ranged Weapon Attack:".to_owned()),
                                TextSpan::Normal(" +4 to hit, range 80/320 ft., one target. ".to_owned()),
                                TextSpan::Italic("Hit:".to_owned()),
                                TextSpan::Normal(" 5 (1d6 + 2) piercing damage.".to_owned())
                            ]
                        }
                    ]
                }            
            ],
            reactions: vec![],
            legendary_actions: None,
            lair_actions: None,
            regional_effects: None,
            source: Some("D&D 5E System Reference Document".to_owned())        
        }
    }


    fn bugbear() -> CreatureCreator {

        CreatureCreator(vec![
            CreatureCommand::Monstorr(1.0,None),
            CreatureCommand::Source("D&D 5E System Reference Document".to_owned()),
            CreatureCommand::Name("Bugbear".to_owned()),
            CreatureCommand::Medium,
            CreatureCommand::Humanoid,
            CreatureCommand::Subtype("goblinoid".to_owned()),
            CreatureCommand::ChaoticEvil,
            CreatureCommand::Armor(Armor::Hide),
            CreatureCommand::Shield,
            CreatureCommand::HitDiceCount(5), 
            CreatureCommand::Walk(30),
            CreatureCommand::Str(15),
            CreatureCommand::Dex(14),
            CreatureCommand::Con(13),
            CreatureCommand::Int(8),
            CreatureCommand::Wis(11),
            CreatureCommand::Cha(9),
            CreatureCommand::Expertise(vec![Skill::Stealth]),
            CreatureCommand::Skills(vec![Skill::Survival]),
            CreatureCommand::Darkvision(60),
            CreatureCommand::Languages(vec![Language::Common,Language::Goblin]),
            CreatureCommand::OverrideChallenge(1),
            CreatureCommand::Weapon(Weapon::Morningstar(0),None),
            CreatureCommand::Weapon(Weapon::Javelin(0),None),
            CreatureCommand::Feature(Feature::Brute(vec!["Morningstar".to_owned(),"Javelin".to_owned()]),None),
            CreatureCommand::Feature(Feature::SurpriseAttack(Dice::new(2,&Die::D6).into()),None),

        ])

    }

    const BUGBEAR: &str = "([
    Monstorr(1),
    Source(\"D&D 5E System Reference Document\"),
    Name(\"Bugbear\"),
    Medium,
    Humanoid,
    Subtype(\"goblinoid\"),
    ChaoticEvil,
    Armor(Hide),
    Shield,
    HitDiceCount(5),
    Walk(30),
    Str(15),
    Dex(14),
    Con(13),
    Int(8),
    Wis(11),
    Cha(9),
    Expertise([
        Stealth,
    ]),
    Skills([
        Survival,
    ]),
    Darkvision(60),
    Languages([
        Common,
        Goblin,
    ]),
    OverrideChallenge(1),
    Weapon(Morningstar(0)),
    Weapon(Javelin(0)),
    Feature(Brute([
        \"Morningstar\",
        \"Javelin\",
    ])),
    Feature(SurpriseAttack(\"2d6\")),
])";

    fn bugbear_stat_block() -> CreatureStatBlock {
        CreatureStatBlock {
            name: "Bugbear".to_owned(),
            size: "Medium".to_owned(),
            type_: "humanoid".to_owned(),
            subtype: Some("goblinoid".to_owned()),
            group: None,
            alignment: "chaotic evil".to_owned(),
            armor: "16 (hide armor, shield)".to_owned(),
            hit_points: "27 (5d8 + 5)".to_owned(),
            speed: "30 ft.".to_owned(),
            strength: "15 (+2)".to_owned(),
            dexterity: "14 (+2)".to_owned(),
            constitution: "13 (+1)".to_owned(),
            intelligence: "8 (-1)".to_owned(),
            wisdom: "11 (+0)".to_owned(),
            charisma: "9 (-1)".to_owned(),
            saving_throws: None,
            skills: Some("Stealth +6, Survival +2".to_owned()),
            damage_vulnerabilities: None,
            damage_resistances: None,
            damage_immunities: None,
            condition_immunities: None,
            senses: "darkvision 60 ft., passive Perception 10".to_owned(),
            languages: Some("Common, Goblin".to_owned()),
            challenge_rating: "1 (200 XP)".to_owned(),
            special_abilities: vec![
                StatBlockFeature {
                    text: vec![
                        TextBlock::Paragraph {
                            heading: Some(vec![TextSpan::Normal("Brute.".to_owned())]),
                            body: vec![
                                TextSpan::Normal("A melee weapon deals one extra die of its damage when the bugbear hits with it (included in the attack).".to_owned())
                            ]
                        }
                    ]
                },
                StatBlockFeature {
                    text: vec![
                        TextBlock::Paragraph {
                            heading: Some(vec![TextSpan::Normal("Surprise Attack.".to_owned())]),
                            body: vec![
                                TextSpan::Normal("If the bugbear surprises a creature and hits it with an attack during the first round of combat, the target takes an extra 7 (2d6) damage from the attack.".to_owned())
                            ]
                        }
                    ]
                }
            ],
            actions: vec![
                StatBlockFeature {
                    text: vec![
                        TextBlock::Paragraph {
                            heading: Some(vec![TextSpan::Normal("Morningstar.".to_owned())]),
                            body: vec![
                                TextSpan::Italic("Melee Weapon Attack:".to_owned()),
                                TextSpan::Normal(" +4 to hit, reach 5 ft., one target. ".to_owned()),
                                TextSpan::Italic("Hit:".to_owned()),
                                TextSpan::Normal(" 11 (2d8 + 2) piercing damage.".to_owned())
                            ]
                        }
                    ]
                },
                StatBlockFeature {
                    text: vec![
                        TextBlock::Paragraph {
                            heading: Some(vec![TextSpan::Normal("Javelin.".to_owned())]),
                            body: vec![
                                TextSpan::Italic("Melee or Ranged Weapon Attack:".to_owned()),
                                TextSpan::Normal(" +4 to hit, reach 5 ft. or range 30/120 ft., one target. ".to_owned()),
                                TextSpan::Italic("Hit:".to_owned()),
                                TextSpan::Normal(" 9 (2d6 + 2) piercing damage in melee or 5 (1d6 + 2) piercing damage at range.".to_owned())
                            ]
                        }
                    ]
                }
            ],
            reactions: Vec::new(),
            legendary_actions: None,
            lair_actions: None,
            regional_effects: None,
            source: Some("D&D 5E System Reference Document".to_owned())            
        }
    }

    fn dragon() -> CreatureCreator {
        CreatureCreator(vec![
            CreatureCommand::Monstorr(1.0,None),
            CreatureCommand::Source("D&D 5E System Reference Document".to_owned()),
            CreatureCommand::Name("Adult Gold Dragon".to_owned()),
            CreatureCommand::SubjectName("the dragon".to_owned()),
            CreatureCommand::Huge,
            CreatureCommand::Dragon,
            CreatureCommand::LawfulGood,
            CreatureCommand::Armor(Armor::Natural(7)),
            CreatureCommand::HitDiceCount(19),
            CreatureCommand::Walk(40),
            CreatureCommand::Fly(80),
            CreatureCommand::Swim(40),
            CreatureCommand::Str(27),
            CreatureCommand::Dex(14),
            CreatureCommand::Con(25),
            CreatureCommand::Int(16),
            CreatureCommand::Wis(15),
            CreatureCommand::Cha(24),
            CreatureCommand::Saves(vec![Ability::Dexterity,Ability::Constitution,Ability::Wisdom,Ability::Charisma]),
            CreatureCommand::Skills(vec![Skill::Insight,Skill::Persuasion,Skill::Stealth]),
            CreatureCommand::Expertise(vec![Skill::Perception]),
            CreatureCommand::Immunity(Damage::Fire),
            CreatureCommand::Blindsight(60),
            CreatureCommand::Darkvision(120),
            CreatureCommand::Languages(vec![Language::Common,Language::Draconic]),
            CreatureCommand::OverrideChallenge(17),
            CreatureCommand::Feature(Feature::Amphibious,None),
            CreatureCommand::Feature(Feature::LegendaryResistance,Some(UsageLimit::PerDay(3))),
            CreatureCommand::Multiattack("The dragon can use its Frightful Presence. It then makes three attacks: one with its bite and two with its claws.".to_owned(),
                                          Multiattack::And(vec![
                                             Multiattack::Attack("Frightful Presence".to_owned()),
                                             Multiattack::Attack("Bite".to_owned()), 
                                             Multiattack::Count(2,vec![Multiattack::Attack("Claw".to_owned())])
                                         ])),
            CreatureCommand::Action(Action::Attack("Bite".to_owned(),Attack {
                type_: Some(AttackType::Weapon),
                bonus: AttackBonus::Default,
                magic: None,
                reach: Some(10),
                range: None,
                long_range: None,
                target: "one target".to_owned()
            },AttackEffect::Damage(Dice::new(2,&Die::D10).into(),AttackBonus::Default,Damage::Piercing),None),None),
            CreatureCommand::Action(Action::Attack("Claw".to_owned(),Attack {
                type_: Some(AttackType::Weapon),
                bonus: AttackBonus::Default,
                magic: None,
                reach: Some(5),
                range: None,
                long_range: None,
                target: "one target".to_owned()
            },AttackEffect::Damage(Dice::new(2,&Die::D6).into(),AttackBonus::Default,Damage::Slashing),None),None),
            CreatureCommand::Action(Action::Attack("Tail".to_owned(),Attack {
                type_: Some(AttackType::Weapon),
                bonus: AttackBonus::Default,
                magic: None,
                reach: Some(15),
                range: None,
                long_range: None,
                target: "one target".to_owned()
            },AttackEffect::Damage(Dice::new(2,&Die::D8).into(),AttackBonus::Default,Damage::Bludgeoning),None),None),
            CreatureCommand::Action(Action::FrightfulPresence(120,21,false),None),
            CreatureCommand::Action(Action::BreathWeapon("Breath Weapons".to_owned(),
                "${Subj} uses one of the following breath weapons.${sub( }Fire Breath.${ )}${Subj} exhales fire in a 60-foot cone. Each creature in that area must make a DC 21 Dexterity saving throw, taking 66 (12d10) fire damage on a failed save, or half as much damage on a successful one.${sub( }Weakening Breath.${ )}${Subj} exhales gas in a 60-foot cone. Each creature in that area must succeed on a DC 21 Strength saving throw or have disadvantage on Strength-based attack rolls, Strength checks, and Strength saving throws for 1 minute. A creature can repeat the saving throw at the end of each of its turns, ending the effect on itself on a success.".to_owned(),
                Some(AttackEffect::AreaSaveHalf(21,Ability::Dexterity,Dice::new(12,&Die::D10).into(),AttackBonus::Fixed(0),Damage::Fire)),
                None
            ),Some(UsageLimit::Recharge(5))),
            CreatureCommand::Action(Action::ChangeShape("a humanoid or beast that has a challenge rating no higher than its own".to_owned()),None), 
            CreatureCommand::LegendaryActions(3,vec![
                LegendaryAction::LegendaryAction(1,Action::Action("Detect".to_owned(),"The dragon makes a Wisdom (Perception) check.".to_owned(),None,None)),
                LegendaryAction::UseAction(1,"Tail Attack".to_owned(),"${Subj} makes a tail attack.".to_owned(),"Tail".to_owned()), 
                LegendaryAction::LegendaryAction(2,Action::Action("Wing Attack".to_owned(), 
                                   "The dragon beats its wings. Each creature within 10 feet of the dragon must succeed on a DC 22 Dexterity saving throw or take ${ 2d6 + 8 } bludgeoning damage and be knocked prone. The dragon can then fly up to half its flying speed.".to_owned(),
                                   Some(AttackEffect::AreaSaveAll(22,Ability::Dexterity,Dice::new(2,&Die::D6).into(),AttackBonus::Fixed(8),Damage::Bludgeoning)),
                                   None
                                ))
            ])
        ])

    }


    const DRAGON: &str = "([
    Monstorr(1),
    Source(\"D&D 5E System Reference Document\"),
    Name(\"Adult Gold Dragon\"),
    SubjectName(\"the dragon\"),
    Huge,
    Dragon,
    LawfulGood,
    Armor(Natural(7)),
    HitDiceCount(19),
    Walk(40),
    Fly(80),
    Swim(40),
    Str(27),
    Dex(14),
    Con(25),
    Int(16),
    Wis(15),
    Cha(24),
    Saves([
        Dexterity,
        Constitution,
        Wisdom,
        Charisma,
    ]),
    Skills([
        Insight,
        Persuasion,
        Stealth,
    ]),
    Expertise([
        Perception,
    ]),
    Immunity(Fire),
    Blindsight(60),
    Darkvision(120),
    Languages([
        Common,
        Draconic,
    ]),
    OverrideChallenge(17),
    Feature(Amphibious),
    Feature(LegendaryResistance, Some(PerDay(3))),
    Multiattack(\"The dragon can use its Frightful Presence. It then makes three attacks: one with its bite and two with its claws.\", And([
        Attack(\"Frightful Presence\"),
        Attack(\"Bite\"),
        Count(2, [
            Attack(\"Claw\"),
        ]),
    ])),
    Action(Attack(\"Bite\", (
        type: Some(Weapon),
        reach: Some(10),
        target: \"one target\",
    ), Damage(\"2d10\", Default, Piercing))),
    Action(Attack(\"Claw\", (
        type: Some(Weapon),
        reach: Some(5),
        target: \"one target\",
    ), Damage(\"2d6\", Default, Slashing))),
    Action(Attack(\"Tail\", (
        type: Some(Weapon),
        reach: Some(15),
        target: \"one target\",
    ), Damage(\"2d8\", Default, Bludgeoning))),
    Action(FrightfulPresence(120, 21, false)),
    Action(BreathWeapon(\"Breath Weapons\", \"${Subj} uses one of the following breath weapons.${sub( }Fire Breath.${ )}${Subj} exhales fire in a 60-foot cone. Each creature in that area must make a DC 21 Dexterity saving throw, taking 66 (12d10) fire damage on a failed save, or half as much damage on a successful one.${sub( }Weakening Breath.${ )}${Subj} exhales gas in a 60-foot cone. Each creature in that area must succeed on a DC 21 Strength saving throw or have disadvantage on Strength-based attack rolls, Strength checks, and Strength saving throws for 1 minute. A creature can repeat the saving throw at the end of each of its turns, ending the effect on itself on a success.\", Some(AreaSaveHalf(21, Dexterity, \"12d10\", Fixed(0), Fire))), Some(Recharge(5))),
    Action(ChangeShape(\"a humanoid or beast that has a challenge rating no higher than its own\")),
    LegendaryActions(3, [
        LegendaryAction(1, Action(\"Detect\", \"The dragon makes a Wisdom (Perception) check.\", None)),
        UseAction(1, \"Tail Attack\", \"${Subj} makes a tail attack.\", \"Tail\"),
        LegendaryAction(2, Action(\"Wing Attack\", \"The dragon beats its wings. Each creature within 10 feet of the dragon must succeed on a DC 22 Dexterity saving throw or take ${ 2d6 + 8 } bludgeoning damage and be knocked prone. The dragon can then fly up to half its flying speed.\", Some(AreaSaveAll(22, Dexterity, \"2d6\", Fixed(8), Bludgeoning)))),
    ]),
])";

    fn dragon_stat_block() -> CreatureStatBlock {
        CreatureStatBlock {
            name: "Adult Gold Dragon".to_owned(),
            size: "Huge".to_owned(),
            type_: "dragon".to_owned(),
            subtype: None,
            group: None,
            alignment: "lawful good".to_owned(),
            armor: "19 (natural armor)".to_owned(),
            hit_points: "256 (19d12 + 133)".to_owned(),
            speed: "40 ft., fly 80 ft., swim 40 ft.".to_owned(),
            strength: "27 (+8)".to_owned(),
            dexterity: "14 (+2)".to_owned(),
            constitution: "25 (+7)".to_owned(),
            intelligence: "16 (+3)".to_owned(),
            wisdom: "15 (+2)".to_owned(),
            charisma: "24 (+7)".to_owned(),
            saving_throws: Some("Dex +8, Con +13, Wis +8, Cha +13".to_owned()),
            skills: Some("Insight +8, Perception +14, Persuasion +13, Stealth +8".to_owned()),
            damage_vulnerabilities: None,
            damage_resistances: None,
            damage_immunities: Some("fire".to_owned()),
            condition_immunities: None,
            senses: "blindsight 60 ft., darkvision 120 ft., passive Perception 24".to_owned(),
            languages: Some("Common, Draconic".to_owned()),
            challenge_rating: "17 (18,000 XP)".to_owned(),
            special_abilities: vec![
                StatBlockFeature {
                    text: vec![
                        TextBlock::Paragraph {
                            heading: Some(vec![TextSpan::Normal("Amphibious.".to_owned())]),
                            body: vec![
                                TextSpan::Normal("The dragon can breathe air and water.".to_owned())
                            ]
                        }
                    ]
                },
                StatBlockFeature {
                    text: vec![
                        TextBlock::Paragraph {
                            heading: Some(vec![TextSpan::Normal("Legendary Resistance (3/Day).".to_owned())]),
                            body: vec![
                                TextSpan::Normal("If the dragon fails a saving throw, it can choose to succeed instead.".to_owned())
                            ]
                        }
                    ]
                }
            ],
            actions: vec![
                StatBlockFeature {
                    text: vec![
                        TextBlock::Paragraph {
                            heading: Some(vec![TextSpan::Normal("Multiattack.".to_owned())]),
                            body: vec![
                                TextSpan::Normal("The dragon can use its Frightful Presence. It then makes three attacks: one with its bite and two with its claws.".to_owned())
                            ]
                        }
                    ]
                },
                StatBlockFeature {
                    text: vec![
                        TextBlock::Paragraph {
                            heading: Some(vec![TextSpan::Normal("Bite.".to_owned())]),
                            body: vec![
                                TextSpan::Italic("Melee Weapon Attack:".to_owned()),
                                TextSpan::Normal(" +14 to hit, reach 10 ft., one target. ".to_owned()),
                                TextSpan::Italic("Hit:".to_owned()),
                                TextSpan::Normal(" 19 (2d10 + 8) piercing damage.".to_owned())
                            ]
                        }
                    ]
                },
                StatBlockFeature {
                    text: vec![
                        TextBlock::Paragraph {
                            heading: Some(vec![TextSpan::Normal("Claw.".to_owned())]),
                            body: vec![
                                TextSpan::Italic("Melee Weapon Attack:".to_owned()),
                                TextSpan::Normal(" +14 to hit, reach 5 ft., one target. ".to_owned()),
                                TextSpan::Italic("Hit:".to_owned()),
                                TextSpan::Normal(" 15 (2d6 + 8) slashing damage.".to_owned())
                            ]
                        }
                    ]
                },
                StatBlockFeature {
                    text: vec![
                        TextBlock::Paragraph {
                            heading: Some(vec![TextSpan::Normal("Tail.".to_owned())]),
                            body: vec![
                                TextSpan::Italic("Melee Weapon Attack:".to_owned()),
                                TextSpan::Normal(" +14 to hit, reach 15 ft., one target. ".to_owned()),
                                TextSpan::Italic("Hit:".to_owned()),
                                TextSpan::Normal(" 17 (2d8 + 8) bludgeoning damage.".to_owned())
                            ]
                        }
                    ]
                },
                StatBlockFeature {
                    text: vec![
                        TextBlock::Paragraph {
                            heading: Some(vec![TextSpan::Normal("Frightful Presence.".to_owned())]),
                            body: vec![
                                TextSpan::Normal("Each creature of the dragon's choice that is within 120 feet of the dragon and aware of it must succeed on a DC 21 Wisdom saving throw or become frightened for 1 minute. A creature can repeat the saving throw at the end of each of its turns, ending the effect on itself on a success. If a creature's saving throw is successful or the effect ends for it, the creature is immune to the dragon's Frightful Presence for the next 24 hours.".to_owned())
                            ]
                        }
                    ]
                },
                StatBlockFeature {
                    text: vec![
                        TextBlock::Paragraph {
                            heading: Some(vec![TextSpan::Normal("Breath Weapons (Recharge 5-6).".to_owned())]),
                            body: vec![
                                TextSpan::Normal("The dragon uses one of the following breath weapons.".to_owned())
                            ]
                        },
                        TextBlock::SubParagraph{
                            heading: Some(vec![
                                TextSpan::Normal("Fire Breath.".to_owned())
                            ]),
                            body: vec![
                                TextSpan::Normal("The dragon exhales fire in a 60-foot cone. Each creature in that area must make a DC 21 Dexterity saving throw, taking 66 (12d10) fire damage on a failed save, or half as much damage on a successful one.".to_owned())
                            ]
                        },
                        TextBlock::SubParagraph{
                            heading: Some(vec![
                                TextSpan::Normal("Weakening Breath.".to_owned())
                            ]),
                            body: vec![
                                TextSpan::Normal("The dragon exhales gas in a 60-foot cone. Each creature in that area must succeed on a DC 21 Strength saving throw or have disadvantage on Strength-based attack rolls, Strength checks, and Strength saving throws for 1 minute. A creature can repeat the saving throw at the end of each of its turns, ending the effect on itself on a success.".to_owned())
                            ]
                        }
                    ]
                },
                StatBlockFeature {
                    text: vec![
                        TextBlock::Paragraph {
                            heading: Some(vec![TextSpan::Normal("Change Shape.".to_owned())]),
                            body: vec![
                                TextSpan::Normal("The dragon magically polymorphs into a humanoid or beast that has a challenge rating no higher than its own, or back into its true form. It reverts to its true form if it dies. Any equipment it is wearing or carrying is absorbed or borne by the new form (the dragon's choice). In a new form, the dragon retains its alignment, hit points, Hit Dice, ability to speak, proficiencies, Legendary Resistance, lair actions, and Intelligence, Wisdom, and Charisma scores, as well as this action. Its statistics and capabilities are otherwise replaced by those of the new form, except any class features or legendary actions of that form.".to_owned())
                            ]
                        }
                    ]
                }
            ],
            reactions: Vec::new(),
            legendary_actions: Some(StatBlockLegendary {
                description: vec![
                    TextBlock::Paragraph {
                        heading: None,
                        body: vec![
                            TextSpan::Normal("The dragon can take 3 legendary actions, choosing from the options below. Only one legendary action option can be used at a time and only at the end of another creature's turn. The dragon regains spent legendary actions at the start of its turn.".to_owned())
                        ]
                    }
                ],
                actions: vec![
                    StatBlockFeature {
                        text: vec![
                            TextBlock::SubParagraph {
                                heading: Some(vec![TextSpan::Normal("Detect.".to_owned())]),
                                body: vec![
                                    TextSpan::Normal("The dragon makes a Wisdom (Perception) check.".to_owned())
                                ]
                            }
                        ]
                    },
                    StatBlockFeature {
                        text: vec![
                            TextBlock::SubParagraph {
                                heading: Some(vec![TextSpan::Normal("Tail Attack.".to_owned())]),
                                body: vec![
                                    TextSpan::Normal("The dragon makes a tail attack.".to_owned())
                                ]
                            }
                        ]
                    },
                    StatBlockFeature {
                        text: vec![
                            TextBlock::SubParagraph {
                                heading: Some(vec![TextSpan::Normal("Wing Attack (Costs 2 Actions).".to_owned())]),
                                body: vec![
                                    TextSpan::Normal("The dragon beats its wings. Each creature within 10 feet of the dragon must succeed on a DC 22 Dexterity saving throw or take 15 (2d6 + 8) bludgeoning damage and be knocked prone. The dragon can then fly up to half its flying speed.".to_owned())
                                ]
                            }
                        ]
                    }
                ]
            }),
            lair_actions: None,
            regional_effects: None,
            source: Some("D&D 5E System Reference Document".to_owned())            
        }
    }

    fn efreeti() -> CreatureCreator {
        CreatureCreator(vec![
            CreatureCommand::Monstorr(1.0,None),
            CreatureCommand::Source("D&D 5E System Reference Document".to_owned()),
            CreatureCommand::Name("Efreeti".to_owned()),
            CreatureCommand::Large,
            CreatureCommand::Elemental,
            CreatureCommand::LawfulEvil,
            CreatureCommand::Armor(Armor::Natural(6)),
            CreatureCommand::HitDiceCount(16),
            CreatureCommand::Walk(40),
            CreatureCommand::Fly(60),
            CreatureCommand::Str(22),
            CreatureCommand::Dex(12),
            CreatureCommand::Con(24),
            CreatureCommand::Int(16),
            CreatureCommand::Wis(15),
            CreatureCommand::Cha(16),
            CreatureCommand::Saves(vec![Ability::Intelligence,Ability::Wisdom,Ability::Charisma]),
            CreatureCommand::Immunity(Damage::Fire),
            CreatureCommand::Darkvision(120),
            CreatureCommand::Languages(vec![Language::Ignan]),
            CreatureCommand::OverrideChallenge(11),
            CreatureCommand::Feature(Feature::Feature("Elemental Demise".to_owned(),"If ${subj} dies, its body disintegrates in a flash of fire and puff of smoke, leaving behind only equipment ${subj} was wearing or carrying.".to_owned()),None),
            CreatureCommand::InnateSpellcasting(vec![
                InnateSpellcastingCommand::Ability(Ability::Charisma),
                InnateSpellcastingCommand::SaveDC(15),
                InnateSpellcastingCommand::Attack(7),
                InnateSpellcastingCommand::AtWill(vec!["detect magic".to_owned()]),
                InnateSpellcastingCommand::PerDay(3,vec!["enlarge/reduce".to_owned(),"tongues".to_owned()]),
                InnateSpellcastingCommand::PerDay(1,vec!["conjure elemental".to_owned(),"gaseous form".to_owned(),"invisibility".to_owned(),"major image".to_owned(),"plane shift".to_owned(),"wall of fire".to_owned()]),
                InnateSpellcastingCommand::SpellRestriction("conjure elemental".to_owned(),"fire elemental only".to_owned()),
            ]),
            CreatureCommand::Multiattack("${Subj} makes two scimitar attacks or uses its Hurl Flame twice.".to_owned(),Multiattack::Or(vec![
                Multiattack::Count(2,vec![Multiattack::Attack("Scimitar".to_owned())]),
                Multiattack::Count(2,vec![Multiattack::Attack("Hurl Flame".to_owned())])
            ])),
            CreatureCommand::Weapon(Weapon::Scimitar(0),Some(CompoundAttackEffect::Plus(
                AttackEffect::Damage(Dice::new(2,&Die::D6).into(),AttackBonus::Fixed(0),Damage::Fire)
            ))),
            CreatureCommand::Action(Action::Attack("Hurl Flame".to_owned(),Attack { 
                type_: Some(AttackType::Spell),
                bonus: AttackBonus::Charisma,
                magic: None,
                reach: None,
                range: Some(120),
                long_range: None,
                target: "one target".to_owned()
            },AttackEffect::Damage(Dice::new(5,&Die::D6).into(),AttackBonus::Fixed(0),Damage::Fire),None),None)

        ])
    
    }

    const EFREETI: &str = "([
    Monstorr(1),
    Source(\"D&D 5E System Reference Document\"),
    Name(\"Efreeti\"),
    Large,
    Elemental,
    LawfulEvil,
    Armor(Natural(6)),
    HitDiceCount(16),
    Walk(40),
    Fly(60),
    Str(22),
    Dex(12),
    Con(24),
    Int(16),
    Wis(15),
    Cha(16),
    Saves([
        Intelligence,
        Wisdom,
        Charisma,
    ]),
    Immunity(Fire),
    Darkvision(120),
    Languages([
        Ignan,
    ]),
    OverrideChallenge(11),
    Feature(Feature(\"Elemental Demise\", \"If ${subj} dies, its body disintegrates in a flash of fire and puff of smoke, leaving behind only equipment ${subj} was wearing or carrying.\")),
    InnateSpellcasting([
        Ability(Charisma),
        SaveDC(15),
        Attack(7),
        AtWill([
            \"detect magic\",
        ]),
        PerDay(3, [
            \"enlarge/reduce\",
            \"tongues\",
        ]),
        PerDay(1, [
            \"conjure elemental\",
            \"gaseous form\",
            \"invisibility\",
            \"major image\",
            \"plane shift\",
            \"wall of fire\",
        ]),
        SpellRestriction(\"conjure elemental\", \"fire elemental only\"),
    ]),
    Multiattack(\"${Subj} makes two scimitar attacks or uses its Hurl Flame twice.\", Or([
        Count(2, [
            Attack(\"Scimitar\"),
        ]),
        Count(2, [
            Attack(\"Hurl Flame\"),
        ]),
    ])),
    Weapon(Scimitar(0), Some(Plus(Damage(\"2d6\", Fixed(0), Fire)))),
    Action(Attack(\"Hurl Flame\", (
        type: Some(Spell),
        bonus: Charisma,
        range: Some(120),
        target: \"one target\",
    ), Damage(\"5d6\", Fixed(0), Fire))),
])";

    fn efreeti_stat_block() -> CreatureStatBlock {
        CreatureStatBlock {
            name: "Efreeti".to_owned(),
            size: "Large".to_owned(),
            type_: "elemental".to_owned(),
            subtype: None,
            group: None,
            alignment: "lawful evil".to_owned(),
            armor: "17 (natural armor)".to_owned(),
            hit_points: "200 (16d10 + 112)".to_owned(),
            speed: "40 ft., fly 60 ft.".to_owned(),
            strength: "22 (+6)".to_owned(),
            dexterity: "12 (+1)".to_owned(),
            constitution: "24 (+7)".to_owned(),
            intelligence: "16 (+3)".to_owned(),
            wisdom: "15 (+2)".to_owned(),
            charisma: "16 (+3)".to_owned(),
            saving_throws: Some("Int +7, Wis +6, Cha +7".to_owned()),
            skills: None,
            damage_vulnerabilities: None,
            damage_resistances: None,
            damage_immunities: Some("fire".to_owned()),
            condition_immunities: None,
            senses: "darkvision 120 ft., passive Perception 12".to_owned(),
            languages: Some("Ignan".to_owned()),
            challenge_rating: "11 (7,200 XP)".to_owned(),
            special_abilities: vec![
                StatBlockFeature {
                    text: vec![
                        TextBlock::Paragraph {
                            heading: Some(vec![TextSpan::Normal("Elemental Demise.".to_owned())]),
                            body: vec![
                                TextSpan::Normal("If the efreeti dies, its body disintegrates in a flash of fire and puff of smoke, leaving behind only equipment the efreeti was wearing or carrying.".to_owned())
                            ]
                        }
                    ]
                },
                StatBlockFeature {
                    text: vec![
                        TextBlock::Paragraph {
                            heading: Some(vec![TextSpan::Normal("Innate Spellcasting.".to_owned())]),
                            body: vec![
                                TextSpan::Normal("The efreeti's innate spellcasting ability is Charisma (spell save DC 15, +7 to hit with spell attacks). It can innately cast the following spells, requiring no material components:".to_owned())
                            ]
                        },
                        TextBlock::SubParagraph{
                            heading: None,
                            body: vec![
                                TextSpan::Normal("At will: ".to_owned()),
                                TextSpan::Italic("detect magic".to_owned())
                            ]
                        },
                        TextBlock::SubParagraph {
                            heading: None,
                            body: vec![
                                TextSpan::Normal("1/day each: ".to_owned()),
                                TextSpan::Italic("conjure elemental".to_owned()),
                                TextSpan::Normal(" (fire elemental only), ".to_owned()),
                                TextSpan::Italic("gaseous form".to_owned()),
                                TextSpan::Normal(", ".to_owned()),
                                TextSpan::Italic("invisibility".to_owned()),
                                TextSpan::Normal(", ".to_owned()),
                                TextSpan::Italic("major image".to_owned()),
                                TextSpan::Normal(", ".to_owned()),
                                TextSpan::Italic("plane shift".to_owned()),
                                TextSpan::Normal(", ".to_owned()),
                                TextSpan::Italic("wall of fire".to_owned())
                            ]
                        },
                        TextBlock::SubParagraph {
                            heading: None,
                            body: vec![
                                TextSpan::Normal("3/day each: ".to_owned()),
                                TextSpan::Italic("enlarge/reduce".to_owned()),
                                TextSpan::Normal(", ".to_owned()),
                                TextSpan::Italic("tongues".to_owned())
                            ]
                        }
                    ]
                }
            ],
            actions: vec![
                StatBlockFeature {
                    text: vec![
                        TextBlock::Paragraph {
                            heading: Some(vec![TextSpan::Normal("Multiattack.".to_owned())]),
                            body: vec![
                                TextSpan::Normal("The efreeti makes two scimitar attacks or uses its Hurl Flame twice.".to_owned())
                            ]
                        }
                    ]
                },
                StatBlockFeature {
                    text: vec![
                        TextBlock::Paragraph {
                            heading: Some(vec![TextSpan::Normal("Scimitar.".to_owned())]),
                            body: vec![
                                TextSpan::Italic("Melee Weapon Attack:".to_owned()),
                                TextSpan::Normal(" +10 to hit, reach 5 ft., one target. ".to_owned()),
                                TextSpan::Italic("Hit:".to_owned()),
                                TextSpan::Normal(" 13 (2d6 + 6) slashing damage plus 7 (2d6) fire damage.".to_owned())
                            ]
                        }
                    ]
                },
                StatBlockFeature {
                    text: vec![
                        TextBlock::Paragraph {
                            heading: Some(vec![TextSpan::Normal("Hurl Flame.".to_owned())]),
                            body: vec![
                                TextSpan::Italic("Ranged Spell Attack:".to_owned()),
                                TextSpan::Normal(" +7 to hit, range 120 ft., one target. ".to_owned()),
                                TextSpan::Italic("Hit:".to_owned()),
                                TextSpan::Normal(" 17 (5d6) fire damage.".to_owned())
                            ]
                        }
                    ]
                }
            ],
            reactions: Vec::new(),
            legendary_actions: None,
            lair_actions: None,
            regional_effects: None,
            source: Some("D&D 5E System Reference Document".to_owned()),
 
        }
    }

    #[test]
    fn create_goblin() {
        goblin().create_creature(&PathBuf::from(env!("CARGO_MANIFEST_DIR"))).expect("Creature should have been created.");

    }

    #[test]
    fn create_bugbear() {
        bugbear().create_creature(&PathBuf::from(env!("CARGO_MANIFEST_DIR"))).expect("Creature should have been created.");

    }

    #[test]
    fn create_dragon() {
        dragon().create_creature(&PathBuf::from(env!("CARGO_MANIFEST_DIR"))).expect("Creature should have been created.");

    }

    #[test]
    fn create_efreeti() {
        efreeti().create_creature(&PathBuf::from(env!("CARGO_MANIFEST_DIR"))).expect("Creature should have been created.");

    }

    #[test]
    fn stat_block_goblin() {
        assert_eq!(goblin().create_creature(&PathBuf::from(env!("CARGO_MANIFEST_DIR"))).unwrap().try_into_stat_block().expect("Stat block should have been created"),goblin_stat_block());

    }

    #[test]
    fn stat_block_bugbear() {
        assert_eq!(bugbear().create_creature(&PathBuf::from(env!("CARGO_MANIFEST_DIR"))).unwrap().try_into_stat_block().expect("Stat block should have been created"),bugbear_stat_block());

    }

    #[test]
    fn stat_block_dragon() {
        assert_eq!(dragon().create_creature(&PathBuf::from(env!("CARGO_MANIFEST_DIR"))).unwrap().try_into_stat_block().expect("Stat block should have been created"),dragon_stat_block());

    }

    #[test]
    fn stat_block_efreeti() {
        assert_eq!(efreeti().create_creature(&PathBuf::from(env!("CARGO_MANIFEST_DIR"))).unwrap().try_into_stat_block().expect("Stat block should have been created"),efreeti_stat_block());

    }

    #[test]
    fn serialize_goblin() {
        assert_eq!(goblin().save_to_string(),Ok(GOBLIN.to_owned()));

    }

    #[test]
    fn serialize_bugbear() {
        assert_eq!(bugbear().save_to_string(),Ok(BUGBEAR.to_owned()));

    }

    #[test]
    fn serialize_dragon() {
        assert_eq!(dragon().save_to_string(),Ok(DRAGON.to_owned()));

    }

    #[test]
    fn serialize_efreeti() {
        assert_eq!(efreeti().save_to_string(),Ok(EFREETI.to_owned()));

    }

    #[test]
    fn deserialize_goblin() {
        assert_eq!(CreatureCreator::load_from_str(GOBLIN),Ok(goblin()))

    }

    #[test]
    fn deserialize_bugbear() {
        assert_eq!(CreatureCreator::load_from_str(BUGBEAR),Ok(bugbear()))

    }

    #[test]
    fn deserialize_dragon() {
        assert_eq!(CreatureCreator::load_from_str(DRAGON),Ok(dragon()))

    }
    
    #[test]
    fn deserialize_efreeti() {
        assert_eq!(CreatureCreator::load_from_str(EFREETI),Ok(efreeti()))

    }

