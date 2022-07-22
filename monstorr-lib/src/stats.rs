/*
 * Copyright © 2022 Neil M. Sheldon
 * 
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use serde::Deserialize;
use serde::Serialize;

use crate::utils::FloorDiv;
use crate::utils::DisplayWithThousands;

#[derive(PartialEq,Debug,Clone)]
#[derive(Serialize,Deserialize)]
/**
 These are used in a few creature commands and elsewhere to indicate an ability score.
 */
pub enum Ability {
    /// `Strength`
    Strength,
    /// `Dexterity`
    Dexterity,
    /// `Constitution`
    Constitution,
    /// `Intelligence`
    Intelligence,
    /// `Wisdom`
    Wisdom,
    /// `Charisma`
    Charisma
}

impl Ability {

    pub fn score_to_mod(score: u8) -> i8 {
        (score as i8-10).nms_div_floor(&2)
    }

    pub fn to_short_str(&self) -> &'static str {
        match self {
            Ability::Strength => "str",
            Ability::Dexterity => "dex",
            Ability::Constitution => "con",
            Ability::Intelligence => "int",
            Ability::Wisdom => "wis",
            Ability::Charisma => "cha"
        }
    }

    pub fn to_stat_block(score: u8) -> String {
        format!("{} ({:+})",score,Self::score_to_mod(score))
    }



}

impl std::fmt::Display for Ability {

    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(),std::fmt::Error> {
        match self {
            Ability::Strength => write!(f,"Strength"),
            Ability::Dexterity => write!(f,"Dexterity"),
            Ability::Constitution => write!(f,"Constitution"),
            Ability::Intelligence => write!(f,"Intelligence"),
            Ability::Wisdom => write!(f,"Wisdom"),
            Ability::Charisma => write!(f,"Charisma"),
        }
    }
}


#[derive(Debug)]
pub struct ParseAbilityError;


impl std::str::FromStr for Ability {
    type Err = ParseAbilityError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.trim().to_lowercase().as_str() {
            "strength" => Ability::Strength,
            "dexterity" => Ability::Dexterity,
            "constitution" => Ability::Constitution,
            "intelligence" => Ability::Intelligence,
            "wisdom" => Ability::Wisdom,
            "charisma" => Ability::Charisma,
            _ => Err(ParseAbilityError)?
        })
    }    
}

#[derive(Debug,PartialEq)]
#[derive(Serialize,Deserialize)]
pub enum Condition {
    Blinded,
    Charmed,
    Deafened,
    Exhaustion,
    Frightened,
    Grappled,
    Incapacitated,
    Invisible,
    Paralyzed,
    Petrified,
    Poisoned,
    Prone,
    Restrained,
    Stunned,
    Unconscious,
}


impl std::fmt::Display for Condition {

    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(),std::fmt::Error> {
        match self {
            Condition::Blinded => write!(f,"blinded"),
            Condition::Charmed => write!(f,"charmed"),
            Condition::Deafened => write!(f,"deafened"),
            Condition::Exhaustion => write!(f,"exhaustion"),
            Condition::Frightened => write!(f,"frightened"),
            Condition::Grappled => write!(f,"grappled"),
            Condition::Incapacitated => write!(f,"incapacitated"),
            Condition::Invisible => write!(f,"invisible"),
            Condition::Paralyzed => write!(f,"paralyzed"),
            Condition::Petrified => write!(f,"petrified"),
            Condition::Poisoned => write!(f,"poisoned"),
            Condition::Prone => write!(f,"prone"),
            Condition::Restrained => write!(f,"restrained"),
            Condition::Stunned => write!(f,"stunned"),
            Condition::Unconscious => write!(f,"unconscious"),
        }
    }
}


#[derive(Debug,PartialEq,Clone)]
#[derive(Serialize,Deserialize)]
/**
These values are used to represent standard damage types.
*/
pub enum Damage { 
    /// `Bludgeoning`
    Bludgeoning,
    /// `Piercing`
    Piercing,
    /// `Slashing`
    Slashing,
    /// `Cold`
    Cold,
    /// `Fire`
    Fire,
    /// `Thunder`
    Thunder,
    /// `Radiant`
    Radiant,
    /// `Force`
    Force,
    /// `Lightning`
    Lightning,
    /// `Poison`
    Poison,
    /// `Acid`
    Acid,
    /// `Necrotic`
    Necrotic,
    /// `Psychic`
    Psychic,
}

impl std::fmt::Display for Damage {

    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(),std::fmt::Error> {
        match self {
            Damage::Bludgeoning => write!(f,"bludgeoning"),
            Damage::Piercing => write!(f,"piercing"),
            Damage::Slashing => write!(f,"slashing"),
            Damage::Cold => write!(f,"cold"),
            Damage::Fire => write!(f,"fire"),
            Damage::Thunder => write!(f,"thunder"),
            Damage::Radiant => write!(f,"radiant"),
            Damage::Force => write!(f,"force"),
            Damage::Lightning => write!(f,"lightning"),
            Damage::Poison => write!(f,"poison"),
            Damage::Acid => write!(f,"acid"),
            Damage::Necrotic => write!(f,"necrotic"),
            Damage::Psychic => write!(f,"psychic"),
        }

    }

}



#[derive(Debug)]
pub struct ParseDamageError;


impl std::str::FromStr for Damage {
    type Err = ParseDamageError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.trim().to_lowercase().as_str() {
            "bludgeoning" => Damage::Bludgeoning,
            "piercing" => Damage::Piercing,
            "slashing" => Damage::Slashing,
            "cold" => Damage::Cold,
            "fire" => Damage::Fire,
            "thunder" => Damage::Thunder,
            "radiant" => Damage::Radiant,
            "lightning" => Damage::Lightning,
            "poison" => Damage::Poison,
            "acid" => Damage::Acid,
            "necrotic" => Damage::Necrotic,
            "psychic" => Damage::Psychic,
            _ => Err(ParseDamageError)?
        })
    }    
}

#[derive(PartialEq,Debug)]
#[derive(Serialize,Deserialize)]
pub enum DamageRestriction {
    Spell, // damage from spells
    NotAdamantine, // damage from weapons that aren't adamantine
    NotMagicalNotSilver, // damage from weapons that aren't magical and aren't silvered
    NotMagicalNotAdamantine, // damage from weapons that aren't magical and aren't adamantine
    NotMagical, // damage from weapons that aren't magical
}


#[derive(PartialEq,Debug,Hash,Eq,Clone)]
#[derive(Serialize,Deserialize)]
/**
These values are used to represent most of the standard languages found in the core rulebooks, as well as custom languages.
*/
pub enum Language { 
    /// `All`
    All,
    /// `Common`
    Common,
    /// `Goblin`
    Goblin,
    /// `DeepSpeech`
    DeepSpeech,
    /// `Draconic`
    Draconic,
    /// `Auran`
    Auran,
    /// `Sphinx`
    Sphinx,
    /// `Ignan`
    Ignan,
    /// `Abyssal`
    Abyssal,
    /// `Infernal`
    Infernal,
    /// `Elvish`
    Elvish,
    /// `Sylvan`
    Sylvan,
    /// `Undercommon`
    Undercommon,
    /// `Giant`
    Giant,
    /// `Gnomish`
    Gnomish,
    /// `Terran`
    Terran,
    /// `Aquan`
    Aquan,
    /// `Dwarvish`
    Dwarvish,
    /// `Orc`
    Orc,
    /// `GiantEagle`
    GiantEagle,
    /// `GiantElk`
    GiantElk,
    /// `BlinkDog`
    BlinkDog,
    /// `GiantOwl`
    GiantOwl,
    /// `Gnoll`
    Gnoll,
    /// `Celestial`
    Celestial,
    /// `Primordial`
    Primordial,
    /// `Otyugh`
    Otyugh,
    /// `Sahuagin`
    Sahuagin,
    /// `Druidic`
    Druidic,
    /// `WinterWolf`
    WinterWolf,
    /// `Worg`
    Worg,
    /** 
    `Telepathy(<integer>)`
    
    This language adds the telepathy modifier to the languages in your creature's stat block. The integer represents the range of the telepathy.
    */
    Telepathy(u8),
    /**
    `Language(<string>)`

    This adds a custom language to your creature.
    */
    Language(String)

}

impl std::fmt::Display for Language {

    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(),std::fmt::Error> {
        match self {
            Language::All => write!(f,"all"),
            Language::Common => write!(f,"Common"),
            Language::Goblin => write!(f,"Goblin"),
            Language::DeepSpeech => write!(f,"Deep Speech"),
            Language::Draconic => write!(f,"Draconic"),
            Language::Auran => write!(f,"Auran"),
            Language::Sphinx => write!(f,"Sphinx"),
            Language::Ignan => write!(f,"Ignan"),
            Language::Abyssal => write!(f,"Abyssal"),
            Language::Infernal => write!(f,"Infernal"),
            Language::Elvish => write!(f,"Elvish"),
            Language::Sylvan => write!(f,"Sylvan"),
            Language::Undercommon => write!(f,"Undercommon"),
            Language::Giant => write!(f,"Giant"),
            Language::Gnomish => write!(f,"Gnomish"),
            Language::Terran => write!(f,"Terran"),
            Language::Aquan => write!(f,"Aquan"),
            Language::Dwarvish => write!(f,"Dwarvish"),
            Language::Orc => write!(f,"Orc"),
            Language::GiantEagle => write!(f,"Giant Eagle"),
            Language::GiantElk => write!(f,"Giant Elk"),
            Language::BlinkDog => write!(f,"Blink Dog"),
            Language::GiantOwl => write!(f,"Giant Owl"),
            Language::Gnoll => write!(f,"Gnoll"),
            Language::Celestial => write!(f,"Celestial"),
            Language::Primordial => write!(f,"Primordial"),
            Language::Otyugh => write!(f,"Otyugh"),
            Language::Sahuagin => write!(f,"Sahuagin"),
            Language::Druidic => write!(f,"Druidic"),
            Language::WinterWolf => write!(f,"Winter Wolf"),
            Language::Worg => write!(f,"Worg"),
            Language::Telepathy(a) => write!(f,"telepathy {} ft.",a),
            Language::Language(a) => write!(f,"{}",a)
       
        }
    }
}

        


#[derive(Clone,PartialEq,Debug,Hash,Eq)]
#[derive(Serialize,Deserialize)]
/**
These are used to add skill proficiencies to a creature.
*/
pub enum Skill {
    /// `Athletics`
    Athletics,
    /// `Acrobatics`
    Acrobatics,
    /// `SleightOfHand`
    SleightOfHand,
    /// `Stealth`
    Stealth,
    /// `Arcana`
    Arcana,
    /// `History`
    History,
    /// `Investigation`
    Investigation,
    /// `Nature`
    Nature,
    /// `Religion`
    Religion,
    /// `AnimalHandling`
    AnimalHandling,
    /// `Insight`
    Insight,
    /// `Medicine`
    Medicine,
    /// `Perception`
    Perception,
    /// `Survival`
    Survival,
    /// `Deception`
    Deception,
    /// `Intimidation`
    Intimidation,
    /// `Performance`
    Performance,
    /// `Persuasion`
    Persuasion,
}


#[derive(Debug)]
pub struct ParseLanguageError;


impl std::str::FromStr for Language {
    type Err = ParseLanguageError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.trim().to_lowercase().as_str() {
            "common" => Language::Common,
            "goblin" => Language::Goblin,
            "deep speech" => Language::DeepSpeech,
            "draconic" => Language::Draconic,
            "auran" => Language::Auran,
            "sphinx" => Language::Sphinx,
            "ignan" => Language::Ignan,
            "abyssal" => Language::Abyssal,
            "infernal" => Language::Infernal,
            "elvish" => Language::Elvish,
            "sylvan" => Language::Sylvan,
            "undercommon" => Language::Undercommon,
            "giant" => Language::Giant,
            "gnomish" => Language::Gnomish,
            "terran" => Language::Terran,
            "aquan" => Language::Aquan,
            "dwarvish" => Language::Dwarvish,
            "orc" => Language::Orc,
            "giant eagle" => Language::GiantEagle,
            "giant elk" => Language::GiantElk,
            "blink dog" => Language::BlinkDog,
            "giant owl" => Language::GiantOwl,
            "gnoll" => Language::Gnoll,
            "celestial" => Language::Celestial,
            "primordial" => Language::Primordial,
            "otyugh" => Language::Otyugh,
            "sahuagin" => Language::Sahuagin,
            "druidic" => Language::Druidic,
            "winter wolf" => Language::WinterWolf,
            "worg" => Language::Worg,
            _ => Err(ParseLanguageError)?
        })
    }    
}


#[derive(PartialEq,Debug,Clone)]
#[derive(Serialize,Deserialize)]
/**
Represents the possible types of armor from the player's handbook, plus a couple of others.
*/
pub enum Armor { 

    // light armor
    /// `Padded`
    Padded,
    /// `Leather`
    Leather,
    /// `StuddedLeather`
    StuddedLeather,
    
    // medium armor
    /// `Hide`
    Hide,
    /// `ChainShirt`
    ChainShirt,
    /// `ScaleMail`
    ScaleMail,
    /// `Breastplate`
    Breastplate,
    /// `HalfPlate`
    HalfPlate,
    
    // heavy armor
    /// `RingMail`
    RingMail,
    /// `ChainMail`
    ChainMail,
    /// `Splint`
    Splint,
    /// `Plate`
    Plate,

    // Special armor types 
    /**
    `NaturalArmor(<integer>)`

    Used to give a creature natural armor, with the specified bonus to its armor class, in addition to the dexterity bonus.
    */
    Natural(i8), // natural bonus

    /**
    `Armor(<integer>,<string>)`

    Used to add custom armor to a character. The first argument is a specified bonus to its armor class, in addition to the dexterity bonus. The second argument is the description that will be used.
    */
    Armor(u8,String), // value, description

}

pub enum CreatureSize {
    Tiny,
    Small,
    Medium,
    Large,
    Huge,
    Gargantuan
}

impl std::fmt::Display for CreatureSize {

    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(),std::fmt::Error> {
        match self {
            CreatureSize::Tiny => write!(f,"Tiny"),
            CreatureSize::Small => write!(f,"Small"),
            CreatureSize::Medium => write!(f,"Medium"),
            CreatureSize::Large => write!(f,"Large"),
            CreatureSize::Huge => write!(f,"Huge"),
            CreatureSize::Gargantuan => write!(f,"Gargantuan")
        }

    }

}


pub enum CreatureType {
    Aberration,
    Beast,
    Celestial,
    Construct,
    Dragon,
    Elemental,
    Fey,
    Fiend,
    Giant,
    Humanoid,
    Monstrosity,
    Ooze,
    Plant,
    Undead,
    Custom(String)
}

impl std::fmt::Display for CreatureType {

    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(),std::fmt::Error> {
        match self {
            CreatureType::Aberration => write!(f,"aberration"),
            CreatureType::Beast => write!(f,"beast"),
            CreatureType::Celestial => write!(f,"celestial"),
            CreatureType::Construct => write!(f,"construct"),
            CreatureType::Dragon => write!(f,"dragon"),
            CreatureType::Elemental => write!(f,"elemental"),
            CreatureType::Fey => write!(f,"fey"),
            CreatureType::Fiend => write!(f,"fiend"),
            CreatureType::Giant => write!(f,"giant"),
            CreatureType::Humanoid => write!(f,"humanoid"),
            CreatureType::Monstrosity => write!(f,"monstrosity"),
            CreatureType::Ooze => write!(f,"ooze"),
            CreatureType::Plant => write!(f,"plant"),
            CreatureType::Undead => write!(f,"undead"),
            CreatureType::Custom(a) => write!(f,"{}",a)
        }

    }

}




pub enum Alignment {
    AnyAlignment,
    AnyNonGood,
    AnyNonEvil,
    AnyNonLawful,
    AnyNonChaotic,
    AnyGood,
    AnyEvil,
    AnyLawful,
    AnyChaotic,
    LawfulGood,
    NeutralGood,
    ChaoticGood,
    LawfulNeutral,
    Neutral,
    ChaoticNeutral,
    LawfulEvil,
    NeutralEvil,
    ChaoticEvil,
    Unaligned,
    Custom(String), // custom alignment, such as (50% good and 50% evil)

}

impl std::fmt::Display for Alignment {

    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(),std::fmt::Error> {
        match self {
            Alignment::AnyAlignment => write!(f,"any alignment"),
            Alignment::AnyNonGood => write!(f,"any non-good"),
            Alignment::AnyNonEvil => write!(f,"any non-evil"),
            Alignment::AnyNonLawful => write!(f,"any non-lawful"),
            Alignment::AnyNonChaotic => write!(f,"any non-chaotic"),
            Alignment::AnyGood => write!(f,"any good"),
            Alignment::AnyEvil => write!(f,"any evil"),
            Alignment::AnyLawful => write!(f,"any lawful"),
            Alignment::AnyChaotic => write!(f,"any chaotic"),
            Alignment::LawfulGood => write!(f,"lawful good"),
            Alignment::NeutralGood => write!(f,"neutral good"),
            Alignment::ChaoticGood => write!(f,"chaotic good"),
            Alignment::LawfulNeutral => write!(f,"lawful neutral"),
            Alignment::Neutral => write!(f,"neutral"),
            Alignment::ChaoticNeutral => write!(f,"chaotic neutral"),
            Alignment::LawfulEvil => write!(f,"lawful evil"),
            Alignment::NeutralEvil => write!(f,"neutral evil"),
            Alignment::ChaoticEvil => write!(f,"chaotic evil"),
            Alignment::Unaligned => write!(f,"unaligned"),
            Alignment::Custom(a) => write!(f,"{}",a)
        }

    }

}

#[derive(Debug)]
pub enum ParseChallengeRatingError {
    InvalidDenominator,
    InvalidNumerator,
    ChallengeRatingTooBig,
    CouldNotParseAsNumber,
}

impl std::fmt::Display for ParseChallengeRatingError {

    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(),std::fmt::Error> {
        match self {
            Self::InvalidDenominator => write!(f,"invalid denominator"),
            Self::InvalidNumerator => write!(f,"invalid numerator"),
            Self::ChallengeRatingTooBig => write!(f,"challenge ratings over 30 not supported"),
            Self::CouldNotParseAsNumber => write!(f,"could not parse as integer"),
        }
    }
}


#[derive(PartialEq)]
pub enum ChallengeRating {
    None, // for CR 0 with 0 XP, CR 0 with 10 XP uses Whole()
    Eighth,
    Quarter,
    Half,
    Whole(u8)

}

impl ChallengeRating {

    pub fn get_proficiency_bonus(&self) -> u8 {
        match self {
            ChallengeRating::Whole(a) => match a {
                0..=4 => 2,
                5..=8 => 3,
                9..=12 => 4,
                13..=16 => 5,
                17..=20 => 6,
                21..=24 => 7,
                25..=28 => 8,
                29..=30 => 9,
                _ => unimplemented!("Challenge ratings above 30")
            },
            _ => 2
        }
    }

    pub fn get_xp(&self) -> u32 {
        match self {
            Self::None => 0,
            Self::Eighth => 25,
            Self::Quarter => 50,
            Self::Half => 100,
            Self::Whole(num) => match num {
                0 => 10,
                1 => 200,
                2 => 450,
                3 => 700,
                4 => 1100,
                5 => 1800,
                6 => 2300,
                7 => 2900,
                8 => 3900,
                9 => 5000,
                10 => 5900,
                11 => 7200,
                12 => 8400,
                13 => 10000,
                14 => 11500,
                15 => 13000,
                16 => 15000,
                17 => 18000,
                18 => 20000,
                19 => 22000,
                20 => 25000,
                21 => 33000,
                22 => 41000,
                23 => 50000,
                24 => 62000,
                25 => 75000,
                26 => 90000,
                27 => 105000,
                28 => 120000,
                29 => 135000,
                30 => 155000,
                _ => unimplemented!("Challenge ratings above 30")
            }
        }

    }

    pub fn display_with_xp(&self) -> String {
        format!("{} ({} XP)",self,self.get_xp().display_with_thousands())
    }
}

impl std::cmp::PartialOrd for ChallengeRating {

    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(match self {
            ChallengeRating::None => match other {
                ChallengeRating::None => std::cmp::Ordering::Equal,
                _ => std::cmp::Ordering::Less
            },
            ChallengeRating::Eighth => match other {
                ChallengeRating::None => std::cmp::Ordering::Greater,
                ChallengeRating::Whole(0) => std::cmp::Ordering::Greater,
                ChallengeRating::Eighth => std::cmp::Ordering::Equal,
                _ => std::cmp::Ordering::Less
            },
            ChallengeRating::Quarter => match other {
                ChallengeRating::None | 
                ChallengeRating::Eighth => std::cmp::Ordering::Greater,
                ChallengeRating::Whole(0) => std::cmp::Ordering::Greater,
                ChallengeRating::Quarter => std::cmp::Ordering::Equal,
                _ => std::cmp::Ordering::Less
            },
            ChallengeRating::Half => match other {
                ChallengeRating::None | 
                ChallengeRating::Eighth |
                ChallengeRating::Quarter => std::cmp::Ordering::Greater,
                ChallengeRating::Whole(0) => std::cmp::Ordering::Greater,
                ChallengeRating::Half => std::cmp::Ordering::Equal,
                _ => std::cmp::Ordering::Less
            },
            ChallengeRating::Whole(0) =>  match other {
                ChallengeRating::None => std::cmp::Ordering::Greater,
                ChallengeRating::Eighth |
                ChallengeRating::Quarter |
                ChallengeRating::Half => std::cmp::Ordering::Less,
                ChallengeRating::Whole(0) => std::cmp::Ordering::Equal,
                _ => std::cmp::Ordering::Less
            },
            ChallengeRating::Whole(lhs) =>  match other {
                ChallengeRating::None | 
                ChallengeRating::Eighth |
                ChallengeRating::Quarter |
                ChallengeRating::Half => std::cmp::Ordering::Greater,
                ChallengeRating::Whole(rhs) => lhs.partial_cmp(rhs)?
            },
        })

    }

}

impl std::fmt::Display for ChallengeRating {

    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(),std::fmt::Error> {
        match self {
            ChallengeRating::None => write!(f,"0"),
            ChallengeRating::Eighth => write!(f,"1/8"),
            ChallengeRating::Quarter => write!(f,"1/4"),
            ChallengeRating::Half => write!(f,"1/2"),
            ChallengeRating::Whole(a) => write!(f,"{}",a)
        }
    }
}

impl std::str::FromStr for ChallengeRating {
    type Err = ParseChallengeRatingError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s.trim();
        if let Some((numerator,denominator)) = trimmed.split_once('/') {
            if numerator == "1" {
                match denominator {
                    "2" => Ok(ChallengeRating::Half),
                    "4" => Ok(ChallengeRating::Quarter),
                    "8" => Ok(ChallengeRating::Eighth),
                    _ => Err(ParseChallengeRatingError::InvalidDenominator)
                }
            } else {
                Err(ParseChallengeRatingError::InvalidNumerator)
            }

        } else if let Ok(value) = trimmed.parse::<usize>() {
            match value {
                0..=30 => Ok(ChallengeRating::Whole(value as u8)),
                _ => Err(ParseChallengeRatingError::ChallengeRatingTooBig)
            }
        } else {
            Err(ParseChallengeRatingError::CouldNotParseAsNumber)
        }
    }    
}

impl Default for ChallengeRating {

    fn default() -> Self {
        ChallengeRating::None
    }
}
