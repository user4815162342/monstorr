/*
 * Copyright © 2022 Neil M. Sheldon
 * 
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

/*!

This crate implements the syntax for creature files.

# Rusty-Object Notation

If you just want to see the syntax, and aren't interested in rust programming details, you can skip this section.

Creature files follow a serialization format convention called RON, or Rusty-Object Notation. This format is related to how programs written print debug information about data structures. The main reason this format is used is because I don't have to write a special parser, as that code is already available through combination with the `serde` crate. RON has two features that make it great for this usage:

* Structures are tagged outside the enclosing data, so there's no need for a 'type' property on every object, which would make writing these creatures frustratingly wordy.
* RON automatically deduces the namespace of objects based on their context. This removes the need to specify the type of an enum variant when it's in use. This essentially turns the creature command data structure into context-sensitive functions.

# Basic Syntax

RON files are encoded in utf-8. The RON notation supports several basic values. There are a few more than what is described here, but those are not used in creature commands. Some of these are called differently in the official spec.

*Numbers.* Numbers are written as you'd expect. They can be signed, and include decimals and scientific exponent parts ('e+2'). Most of the types here require simple integers, but floating point values are legal in the appropriate context. The context determines the type required, and will often limit the range of the numbers due to the data type the numbers are stored in. In the documentation, these values shown as `<integer>` or `<number>` if the number is not an integer.

*Strings.* Strings are written using double-quotes. Conventional c-style escape sequences are allowed for certain control characters, and characters derived from unicode hex values. In the documentation, these values are shown as `<string>`

*Booleans.* Occasionally, you may find boolean (on/off) values, which are written simply as 'true' or 'false'. In the documentation, these values are shown as `<boolean>`

*List.* A list value is written between brackets, with the items separated by commas (`[ value, ...]`). The type of values allowed in the list depends on the context. In the documentation, these values are shown as `[<type>...]`.

*Map.* A mapping of keys to values. This is represented as comma-separated pairs inside brackets (`{ value: value, ...}`). In general the keys are strings, but they can be any type as long as they are all the same. Values must all be the same type as well. This is rarely used, but it is used at least once. In the documentation, these values are shown as `<map(<key type>, <value type>)>`.

*Tuple Structs.* A tuple struct is represented by list of plain values in parentheses, with the values separated by commas (`( value, ..)`). The empty tuple (`()`) is called the unit struct. This is different from a list, which has a variable length of values all of the same type. A tuple has a specific length and can require different value types for each member. Sometimes, the later items in the tuple can be skipped if they are optionals. In the documentation, these values are shown as `(<type>,...)`. If a value is not required, the documentation will show all the possible signatures.

*Mapped Structs.* A named struct is similar to a tuple struct, but each value is associated with a named property, which is followed by a colon (`( property: value, ..)`). The required fields in this struct, and their value types, depend on the context they are used. In the documentation, these values are shown as `( <property>: <type>,...)`. If a property is not required, it will be shown with a question mark after.

*Tagged Variants.* Some values are differentiated by a tag because more than one value is allowed in the context. Each of these is represented by a simple identifier, usually capitalized on the first character. A tagged variant can have associated data, which is represented by following that tag either by a tuple struct or a mapped struct. In the documentation, these values are shown by the identifier.

*Optionals.* A special case of tagged variants is the optionals. These are represented with the simple tag `None`, representing a value that doesn't exist, and a tagged tuple `Some(..)` representing the actual value. In many cases, if the value is None, it can also be skipped. For example, if a mapped struct has a property `foo` which should hold an optional string, it doesn't have to be specified if that value would be None. In the documentation, these values are shown as `optional(<type>)`.

The last part, the tagged variants, allow us to create what vaguely looks like a function in other languages.

# Dice Expressions

Some of the arguments require dice expressions. As RON has no concept of dice, these are always passed sa strings, which must match a specific pattern. There are three types of these:

*Die String.* This is a reference to a single die type, in the format "d6" or "d8". In the documentation, these values are shown with `<die-string>`.

*Dice String.* This is a reference to a die and the number of times it is rolled, in the format "1d4" or "2d10". In the documentation, these values are shown with `<dice-string>`.

*Dice Expression.* This is a reference to a full dice expression, including bonuses, additional terms, etc, in formats like "3d6 + 10" or "2d4 - 1d10 + 2". They can get slightly more complex than that, as each dice term could be accompanied by a multiplication factor, but you are unlikely to need that. In the documentation, these values are shown with `<dice-expression-string>`.

# Interpolation

Some of the structures allow for strings containing interpolation variables, which allow you to generate this text based on the creature values. This is not part of the RON syntax, but something programmed specifically for Monstorr. For more information on this interpolation syntax, see [`crate::interpolation`]. For a list of the variables available in the interpolation, see [`crate::creature::Creature`].

# Creature Document Structure

For a description fo the basic creature document structure, see [`CreatureCreator`], which is the structure deserialized from a creature file.

*/

use std::collections::HashMap;
use std::path::PathBuf;
use std::fs;

use serde::Deserialize;
use serde::Serialize;

use crate::stats::Ability;
use crate::spellcasting::Spellcasting;
use crate::errors::CreatureError;
use crate::spellcasting::SpellcastingStyle;
use crate::spellcasting::InnateSpellcasting;
use crate::dice::Die;
use crate::stats::Armor;
use crate::stats::Skill;
use crate::stats::Condition;
use crate::stats::Damage;
use crate::stats::Language;
use crate::stats::ChallengeRating;
use crate::attacks::Multiattack;
use crate::attacks::Weapon;
use crate::attacks::CompoundAttackEffect;
use crate::attacks::Attack;
use crate::attacks::AttackBonus;
use crate::attacks::AttackEffect;
use crate::actions::Action;
use crate::actions::UsageLimit;
use crate::reactions::Reaction;
use crate::features::Feature;
use crate::actions::LegendaryAction;
use crate::creature::Creature;
use crate::interpolation::interpolate_str_for_deserialization;
use crate::creature::CreatureLegendaryAction;




#[derive(PartialEq,Debug,Clone)]
#[derive(Serialize,Deserialize)]
/**
These commands add information to a creature's spellcasting feature.
*/
pub enum SpellcastingCommand {
    
    /**
    `Level(<integer>)`

    Specifies the level of spellcaster the creature emulates. In addition to changing the description, this is also used for calculating spell slots available.
    */
    Level(u8),
    
    /**
    `Class(<string>)`

    Specifies the class of spellcaster spells used.
    */
    Class(String), // X has the following (...) spells prepared -- may also be used to calculate spell slots
    
    /**
    `IsThirdCaster`

    This is used for calculating spell slots available. Spell slots generated will be similar to the Rogue Arcane Trickster or the Fighter Eldritch Knight.
    */
    IsThirdCaster, // specifies to use the third-caster class tables to determine spell slots
    
    /**
    `IsHalfCaster`

    This is used for calculating spell slots available. Spell slots generated will be similar to Rangers and Paladins.
    */
    IsHalfCaster, // specifies to use the half-caster class tables to determine spell slots
    
    /**
    `IsFullCaster`

    This is used for calculating spell slots available. Spell slots generated will be similar to Wizards and Clerics.
    */
    IsFullCaster, // specifies to use full-caster spell slots
    
    /**
    `IsWarlock`

    This is used for calculating spell slots available, and making some modifications to the description. Spell slots will be based on the Warlock class.
    */
    IsWarlock, // specifies to use the warlock spell slots
    
    /**
    `Ability(<Ability)`

    In addition to specifying the description, this is used to automatically calculate save DC and attack bonus. See [`crate::stats::Ability`].
    */
    Ability(Ability), // spellcasting ability
    
    /**
    `Slots(<integer>,<integer>)`

    This overrides automatically generated spell slots. The first argument is the level, the second the overridden number of slots for that level.
    */
    Slots(u8,u8), // level, slots -- specify if the other methods don't provide the right numbers for a level
    
    /**
    `SaveDC(<integer>)`

    This overrides the automatically generated save DC.
    */
    SaveDC(u8), // specify if this wasn't calculated correctly
    
    /**
    `Attack(<integer>)`

    This overrides the automatically generated attack bonus.
    */
    Attack(i8), // specify if this wasn't calculated correctly
    
    /**
    `Cantrips([<string>...])`

    Adds the specified cantrips to the spell list.
    */
    Cantrips(Vec<String>), // the cantrips available to cast
    
    /**
    `Spells(<integer>,[<string>...])`

    Adds the specified spells to the spell list for the specified spell level.
    */
    Spells(u8,Vec<String>), // level, the spells available to cast
    
    /**
    `BeforeCombat([<string>...])`

    Specifies that the listed spells are cast before combat, which provides an asterisk and note in the description.
    */
    BeforeCombat(Vec<String>), // list of spells cast before combat
    
    /**
    `RemoveSpells([<string>...])`

    Removes the specified spells from the creature, which is useful when basing the creature off another.
    */
    RemoveSpells(Vec<String>), // overrides the existence of spells if the spellcasting trait was previously provided
}

impl SpellcastingCommand {

    pub fn execute(&self,data: &mut Spellcasting) -> Result<(),CreatureError> {
        match self {
            Self::Level(level) => {
                data.set_caster_level(*level);
            },
            Self::Class(class) => {
                data.set_class(class.clone());
            },
            Self::IsThirdCaster => {
                data.set_style(SpellcastingStyle::Third);
            }
            Self::IsHalfCaster => {
                data.set_style(SpellcastingStyle::Half);
            },
            Self::IsFullCaster => {
                data.set_style(SpellcastingStyle::Full);
            },
            Self::IsWarlock => {
                data.set_style(SpellcastingStyle::Warlock);
            }
            Self::Ability(ability) => {
                data.set_ability(ability.clone());
            },
            Self::Slots(level,count) => {
                data.set_spell_slots(*level,*count);
            },
            Self::SaveDC(save_dc) => {
                data.set_save_dc(Some(*save_dc));
            },
            Self::Attack(attack) => {
                data.set_attack_bonus(Some(*attack))
            },
            Self::Cantrips(spells) => {
                data.add_spells(0,spells);
            },
            Self::Spells(level,spells) => {
                data.add_spells(*level,spells);
            },
            Self::BeforeCombat(spells) => {
                data.set_spells_before_combat(spells)
            },
            Self::RemoveSpells(spells) => {
                data.remove_spells(spells);
            }

        }
        Ok(())
    }
}

#[derive(PartialEq,Debug,Clone)]
#[derive(Serialize,Deserialize)]
/**
These commands add information to a creature's innate spellcasting feature.
*/
pub enum InnateSpellcastingCommand {
    
    /**
    `Ability`

    Sets the spellcasting ability.
    */
    Ability(Ability),
    
    /**
    `SaveDC(<integer>)`

    Overrides the save DC automatically calculated based on ability.
    */
    SaveDC(u8), // specify if this wasn't calculated correctly
    
    /**
    `Attack(<integer>)`

    Overrides the attack bonus automatically calculated based on ability.
    */
    Attack(i8), // specify if this wasn't calculated correctly
    
    /**
    `AtWill([<string>...])`

    Adds spells the creature is capable of casting at will, whoever Will is.
    */
    AtWill(Vec<String>), // list of spells castable at will
    
    /**
    `PerDay(<integer>,[<string>...])`

    Adds spells the creature is capable of casting the specified number of times per day.
    */
    PerDay(u8,Vec<String>), // list of spells castable at X per day
    
    /**
    `RemoveSpells([<string>...])`

    Removes the spells from the creature.
    */
    RemoveSpells(Vec<String>), // overrides the usage of spells if the spellcasting trait was previously provided
    
    /**
    `SpellRestriction(<string>,<string>)`

    Adds a restriction (second argument) to the specified spell (first argument). For example, the Efreeti restricts what type of elementals can be conjured with conjure elemental.
    */
    SpellRestriction(String,String), // spell, restriction -- some spells are restricted, such as "conjure elemental" on Efreetie

}

impl InnateSpellcastingCommand {

    pub fn execute(&self,data: &mut InnateSpellcasting) -> Result<(),CreatureError> {
        match self {
            Self::Ability(ability) => data.set_ability(ability.clone()),
            Self::SaveDC(save_dc) => data.set_save_dc(Some(*save_dc)),
            Self::Attack(attack) => data.set_attack_bonus(Some(*attack)),
            Self::AtWill(spells) => data.add_spells(None, spells),
            Self::PerDay(count,spells) => data.add_spells(Some(*count), spells),
            Self::RemoveSpells(spells) => data.remove_spells(spells),
            Self::SpellRestriction(spell,restriction) => data.set_spell_restriction(spell.clone(), restriction.clone())
        }
        Ok(())
    }
}

#[derive(Default)]
struct CreatureCreationHooks {
    multiattacks: Vec<Multiattack>,
    features: Vec<Feature>,
    expected_challenge_rating: Option<ChallengeRating>
}


#[derive(PartialEq,Debug)]
#[derive(Serialize,Deserialize)]
/**

A CreatureCommand represents a task that is completed in the process of creating a creature to generate a stat block. Most of the commands set or add data to the resulting creature. A few commands have other effects, or remove data that has been added. 

While there is no strict order requirement, commands can work differently depending on what order they are added to the list. Listed statistics, like actions, are added in the order they appear in list. Other commands require the existence of other data, such as an action or other named feature, in order to work. Commands which set simple statistics, such as armor, hit points, size, etc. will override previous commands that set the same statistic.

The documentation below is automatically produced with tools which assume you are programming in Rust. If you are only interested in the commands for use in building creatures, pay no attention to the complicated signatures below the comments. The signature for use in the creature file is provided in the notes.

*/
pub enum CreatureCommand {
    /**
     `Monstorr(<number>)`
     `Monstorr(<number>,Some(<number>))`

     Specifies the minimum version of the Monstorr creature format for which your creature is defined, and optionally, a maximum version. If the copy of Monstorr this creature is run with does not meet the requirements, an error will be reported and the creature will not be created.
     
     The API for Monstorr is unstable. This means that commands may work differently in future versions. In general, if a command is removed, or it's data requirements changed, your creature file will simply fail to load. However, there may be some differences in the actual functionality between certain versions of the code. This command can be used to make sure you get an error if you're using it with an unsupported version, so you don't accidentally get unexpected data.
     * */
    Monstorr(f32,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        Option<f32>), // allows specifying the minimum and maximum version of the monstorr program the file was created for. This is optional, but if present can allow for quick errors from using the wrong program, instead of unknown errors when something doesn't work as expected.

    /**
    `Include(<string>,<map(string:string)>)`
     
    This allows you to include commands from another creature file on disk. With this command, you can either take an existing creature and modify it, or add a common set of features to more than one creature.

    The file is not expected to be a pure creature format. It will be interpolated in a similar way to how feature descriptions are interpolated. The second argument allows you to pass named arguments to that interpolation, letting you create more dynamic content. 
    
    The output of this interpolation *is* expected to be in valid Monstorr creature format. 
    
    Open5e JSON format is not supported for this because converting is a rigorous task that would require parsing the English language (an early attempt to do this for Monstorr ended with frustration). If you convert an existing creature from Open5e or the SRD, your contributions of that document to this project are welcome.
    */
    Include(String,HashMap<String,String>), 


    /**
    `Source(<string>)`

    If you wish to assign a source tag to the resulting creature, to give credit in the stat-block itself, you can use this command.
    */
    Source(String),

    /**
    `Name(<string>)`

    Sets the name of the creature as it will appear in the title of the stat-block.
    */
    Name(String),

    /**
    `SubjectName(<string>)`

    When interpolating descriptions, it is often necessary to refer to the creature by name. The interpolation variable `subj` is used when the creature is referred to as the subject of an action. By default, the value returned is the name of the creature with a definite article ('the goblin'). If you need to override this somehow, you can set a different value here.
    */
    SubjectName(String),

    /**
    `CapitalizeSubjectName(<string>)`

    When interpolating descriptions, it is often necessary to refer to the creature by name. The interpolation variable `Subj` is used when the creature is referred to as the subject of an action at the beginning of a sentence. By default, the value returned is the value from `SubjectName` with the first letter capitalized. If you need to override this somehow, you can set a different value here.
    */
    CapitalizeSubjectName(String),

    /**
    `PossessiveName(<string>)`

    When interpolating descriptions, it is often necessary to refer to the creature by name. The interpolation variable `poss` is used when the creature is referred to as posessing something. By default, the value returned is the value from `SubjectName` with an apostrophe 's' appended. If you need to override this somehow, you can set a different value here.
    */
    PossessiveName(String),

    /**
    `PossessiveName(<string>)`

    When interpolating descriptions, it is often necessary to refer to the creature by name. The interpolation variable `Poss` is used when the creature is referred to as posessing something, and this occurs at the beginning of a setence. By default, the value returned is the value from `PossessiveName` with the first letter capitalized. If you need to override this somehow, you can set a different value here.
    */
    CapitalizePossessiveName(String),

    /**
    `Tiny`
    
    Sets the size of the creature to tiny, overriding any previous size set. The default size for a creature is medium.
    */
    Tiny,

    /**
    `Small`
    
    Sets the size of the creature to small, overriding any previous size set. The default size for a creature is medium.
    */
    Small,

    /**
    `Medium`
    
    Sets the size of the creature to medium, overriding any previous size set. The default size for a creature is medium.
    */
    Medium,

    /**
    `Large`
    
    Sets the size of the creature to large, overriding any previous size set. The default size for a creature is medium.
    */
    Large,

    /**
    `Huge`
    
    Sets the size of the creature to huge, overriding any previous size set. The default size for a creature is medium.
    */
    Huge,

    /**
    `Gargantuan`
    
    Sets the size of the creature to gargantuan, overriding any previous size set. The default size for a creature is medium.
    */
    Gargantuan,

    
    /**
    `Aberration`

    Sets the type of the creature to "aberration", overriding any previous type set. The default type for a creature is humanoid.
    */
    Aberration,

    /**
    `Beast`

    Sets the type of the creature to "beast", overriding any previous type set. The default type for a creature is humanoid.
    */
    Beast,

    /**
    `Celestial`

    Sets the type of the creature to "celestial", overriding any previous type set. The default type for a creature is humanoid.
    */
    Celestial,

    /**
    `Construct`

    Sets the type of the creature to "construct", overriding any previous type set. The default type for a creature is humanoid.
    */
    Construct,

    /**
    `Dragon`

    Sets the type of the creature to "dragon", overriding any previous type set. The default type for a creature is humanoid.
    */
    Dragon,

    /**
    `Elemental`

    Sets the type of the creature to "elemental", overriding any previous type set. The default type for a creature is humanoid.
    */
    Elemental,

    /**
    `Fey`

    Sets the type of the creature to "fey", overriding any previous type set. The default type for a creature is humanoid.
    */
    Fey,

    /**
    `Fiend`

    Sets the type of the creature to "fiend", overriding any previous type set. The default type for a creature is humanoid.
    */
    Fiend,

    /**
    `Giant`

    Sets the type of the creature to "giant", overriding any previous type set. The default type for a creature is humanoid.
    */
    Giant,

    /**
    `Humanoid`

    Sets the type of the creature to "humanoid", overriding any previous type set. The default type for a creature is humanoid.
    */
    Humanoid,

    /**
    `Monstrosity`

    Sets the type of the creature to "monstrosity", overriding any previous type set. The default type for a creature is humanoid.
    */
    Monstrosity,

    /**
    `Ooze`

    Sets the type of the creature to "ooze", overriding any previous type set. The default type for a creature is humanoid.
    */
    Ooze,

    /**
    `Plant`

    Sets the type of the creature to "plant", overriding any previous type set. The default type for a creature is humanoid.
    */
    Plant,

    /**
    `Undead`

    Sets the type of the creature to "undead", overriding any previous type set. The default type for a creature is humanoid.
    */
    Undead,

    /**
    `CreatureType(<string>)`

    Sets the type of the creature to some custom value, such as "swarm of Tiny Beasts", overriding any previous type set. The default type for a creature is humanoid.
    */
    CreatureType(String),    


    /**
    `Subtype(<string>)`

    Sets the subtype of the creature, such as "goblinoid", or "dragon", overriding any previous subtype set. By default, creatures do not have a subtype.
    */
    Subtype(String),

    /**
    `Group(<string>)`

    Sets a field called group on the creature. The usage of this field is unknown, but it might be used to specify that the creature belongs under a different heading (such as demons, oozes, etc.) in a creature listing, that is not related to the subtype.
    */
    Group(String),

    /**
    `AnyAlignment`

    Sets the alignment of the creature to "any", overriding any previous alignment set. By default, creatures have an alignment of "any".
    */
    AnyAlignment,

    /**
    `AnyNonGood`

    Sets the alignment of the creature to "any non-good", overriding any previous alignment set. By default, creatures have an alignment of "any".
    */
    AnyNonGood,

    /**
    `AnyNonEvil`

    Sets the alignment of the creature to "any non-evil", overriding any previous alignment set. By default, creatures have an alignment of "any".
    */
    AnyNonEvil,

    /**
    `AnyNonLawful`

    Sets the alignment of the creature to "any non-lawful", overriding any previous alignment set. By default, creatures have an alignment of "any".
    */
    AnyNonLawful,

    /**
    `AnyNonChaotic`

    Sets the alignment of the creature to "any non-chaotic", overriding any previous alignment set. By default, creatures have an alignment of "any".
    */
    AnyNonChaotic,

    /**
    `AnyGood`

    Sets the alignment of the creature to "any good", overriding any previous alignment set. By default, creatures have an alignment of "any".
    */
    AnyGood,

    /**
    `AnyEvil`

    Sets the alignment of the creature to "any evil", overriding any previous alignment set. By default, creatures have an alignment of "any".
    */
    AnyEvil,

    /**
    `AnyLawful`

    Sets the alignment of the creature to "any lawful", overriding any previous alignment set. By default, creatures have an alignment of "any".
    */
    AnyLawful, 

    /**
    `AnyChaotic`

    Sets the alignment of the creature to "any chaotic", overriding any previous alignment set. By default, creatures have an alignment of "any".
    */
    AnyChaotic,

    /**
    `LawfulGood`

    Sets the alignment of the creature to "lawful good", overriding any previous alignment set. By default, creatures have an alignment of "any".
    */
    LawfulGood,

    /**
    `NeutralGood`

    Sets the alignment of the creature to "neutral good", overriding any previous alignment set. By default, creatures have an alignment of "any".
    */
    NeutralGood,

    /**
    `ChaoticGood`

    Sets the alignment of the creature to "chaotic good", overriding any previous alignment set. By default, creatures have an alignment of "any".
    */
    ChaoticGood,

    /**
    `LawfulNeutral`

    Sets the alignment of the creature to "lawful neutral", overriding any previous alignment set. By default, creatures have an alignment of "any".
    */
    LawfulNeutral,

    /**
    `Neutral`

    Sets the alignment of the creature to "neutral", overriding any previous alignment set. By default, creatures have an alignment of "any".
    */
    Neutral,

    /**
    `ChaoticNeutral`

    Sets the alignment of the creature to "chaotic neutral", overriding any previous alignment set. By default, creatures have an alignment of "any".
    */
    ChaoticNeutral,

    /**
    `LawfulEvil`

    Sets the alignment of the creature to "lawful evil", overriding any previous alignment set. By default, creatures have an alignment of "any".
    */
    LawfulEvil,

    /**
    `NeutralEvil`

    Sets the alignment of the creature to "neutral evil", overriding any previous alignment set. By default, creatures have an alignment of "any".
    */
    NeutralEvil,

    /**
    `ChaoticEvil`

    Sets the alignment of the creature to "chaotic evil", overriding any previous alignment set. By default, creatures have an alignment of "any".
    */
    ChaoticEvil,

    /**
    `Unaligned`

    Sets the alignment of the creature to "unaligned", overriding any previous alignment set. By default, creatures have an alignment of "any".
    */
    Unaligned,

    /**
    `Alignment(<string>)`

    Sets the alignment of the creature to some custom value, overriding any previous alignment set. By default, creatures have an alignment of "any".
    */
    Alignment(String), // custom alignment, such as (50% good and 50% evil)

    /**
    `HitDie(<die-string>)`

    Sets the base hit die used for calculating hit points on the creature. This only specifies the actual dice used, use 'HitDiceCount' to specify the number of dice. The hit points are normally calculated using the die, the count, and a bonus calculated from constitution.

    By default, the hit die used is assigned based on the creature's size, according to the official rules.
    */
    HitDie(Die), 

    /**
    `HitDiceCount(<integer>)`

    Sets the number of dice rolled to calculate hit points on the creature. This only specifies the number of cie, use `HitDie` to override what dice is used. The hit points are normally calculated using the die, the count, and a bonus calculated from constitution. By default, the hit dice count is 1.

    */
    HitDiceCount(u8),

    /**
    `HitPoints(<integer>)`

    Sets the actual hit points for the creature as shown in the stat block. This does not change the dice expression used for calculation, use `HitDie` and `HitDiceCount` for that. By default, the hit points are calculated as the average of the dice expression calculated from those values.
    */
    HitPoints(u16), // If you want custom hit point count, say you rolled a different value, put them here.

    /**
    `Armor(<Armor>)`

    Sets the type of armor worn by the creature, which is used to determine its armor class and the armor class description. See [`crate::stats::Armor`] for the available values for this. By default, the creature has no armor.
    */
    Armor(Armor),

    /**
    `Shield`

    Adds a shield to the creature, which is used to determine its armor class and armor class description.
    */
    Shield, 

    /**
    `NoShield`

    Removes a shield from the creature. This is useful if you are overriding a creature that was already specified to have a shield.
    */
    NoShield, 


    // speed 
    /**
    `Walk(<integer>)`

    Adds a walk speed to the creature. Setting this to 0 will show a speed of 0 ft. By default, the creature's only speed is a walk speed of 30 ft.
    */
    Walk(u8),
    
    /**
    `Swim(<integer>)`

    Adds a swim speed to the creature. Setting this to 0 will remove the value that was already added. By default, the creature's only speed is a walk speed of 30 ft.
    */
    Swim(u8),
    
    /**
    `Fly(<integer>)`

    Adds a fly speed to the creature. Setting this to 0 will remove the value that was already added, and remove the hover if that was set. By default, the creature's only speed is a walk speed of 30 ft.
    */
    Fly(u8),
    
    /**
    `Hover`

    Adds a hover tag to the creatures' flying speed.
    */
    Hover,

    /**
    `Burrow(<integer>)`

    Adds a burrow speed to the creature. Setting this to 0 will remove the value that was already added. By default, the creature's only speed is a walk speed of 30 ft.
    */
    Burrow(u8),
    
    /**
    `Climb(<integer>)`

    Adds a climb speed to the creature. Setting this to 0 will remove the value that was already added. By default, the creature's only speed is a walk speed of 30 ft.
    */
    Climb(u8),

    /**
    `SpeedNotes(<string>)`

    Adds custom notes for display in parentheses at the end of the speed property in the final stat-block.
    */
    SpeedNotes(String),

    /**
    `Speed(<string>,<integer>)`

    Adds a custom speed to the creature with the specified name. Setting this to 0 will remove the value that was already added.
    */
    Speed(String,u8), // for custom speed 

    // ability
    /**
    `Str(<integer>)`
    
    Sets the creature's Strength score. By default, this score will be 10.
    */
    Str(u8),

    /**
    `Dex(<integer>)`
    
    Sets the creature's Dexterity score. By default, this score will be 10.
    */
    Dex(u8),

    /**
    `Con(<integer>)`
    
    Sets the creature's Constitution score. By default, this score will be 10.
    */
    Con(u8),

    /**
    `Int(<integer>)`
    
    Sets the creature's Intelligence score. By default, this score will be 10.
    */
    Int(u8),

    /**
    `Wis(<integer>)`
    
    Sets the creature's Wisdom score. By default, this score will be 10.
    */
    Wis(u8),

    /**
    `Cha(<integer>)`
    
    Sets the creature's Charisma score. By default, this score will be 10.
    */
    Cha(u8),


    /**
    `Saves([<Ability>...])`

    Adds save proficiencies for the specified Abilities. The actual bonuses are calculated from other information. See [`crate::stats::Ability`] for possible values.

    */
    Saves(Vec<Ability>),

    /**
    `Skills([<Skill>...])`

    Adds skill proficiencies. The bonuses are calculated from other information given. See [`crate::stats::Skill`] for possible values.
    */
    Skills(Vec<Skill>),

    /**
    `Expertise([<Skill>...])`

    Adds skill proficiencies with expertise (doubling proficiency bonus for these skills). The bonuses are calculated from other information given. See [`crate::stats::Skill`] for possible values.
    */
    Expertise(Vec<Skill>),

    /**
    `RemoveSaves([<Ability>...])`

    Removes save proficiencies for the specified Abilities. See [`crate::stats::Ability`] for possible values.
    */
    RemoveSaves(Vec<Ability>),

    /**
    `RemoveSkills([<Skill>...])`

    Removes skill proficiencies and expertise for the specified Skills. See [`crate::stats::Skill`] for possible values.
    */
    RemoveSkills(Vec<Skill>), 
    
    /**
    `ConditionImmunity(<Condition>)`

    Adds a condition immunity to the creature. See [`crate::stats::Condition`] for possible values.
    */
    ConditionImmunity(Condition),
    
    /**
    `Vulnerability(<Damage>)`

    Adds vulnerability to a damage type. See [`crate::stats::Damage`] for possible values.
    */
    Vulnerability(Damage),
    
    /**
    `AllVulnerability`

    Adds vulnerability to all damage.
    */
    AllVulnerability,

    /**
    `CustomVulnerability(<string>)`

    Overrides the text of the vulnerabilities property.
    */
    CustomVulnerability(String),
    
    /**
    `Resistance(<Damage>)`

    Adds resistance to a damage type. See [`crate::stats::Damage`] for possible values.
    */
    Resistance(Damage),
    
    /**
    `AllResistance`

    Adds resistance to all damage.
    */
    AllResistance,

    /**
    `NonmagicalResistance`
    
    Adds resistance to non-magical attacks.
    */
    NonmagicalResistance,

    /**
    `NonSilveredResistance`
    
    Adds resistance to non-silvered and non-magical attacks.
    */
    NonSilveredResistance,

    /**
    `NonAdamantineResistance`
    
    Adds resistance to non-adamantine and non-magical attacks.
    */
    NonAdamantineResistance,


    /**
    `CustomResistance(<string>)`

    Overrides the text of the resistances property.
    */
    CustomResistance(String),
    
    /**
    `Immunity(<Damage>)`

    Adds immunity to a damage type. See [`crate::stats::Damage`] for possible values.
    */
    Immunity(Damage),
    
    /**
    `AllImmunity`

    Adds immunity to all damage.
    */
    AllImmunity,

    /**
    `NonmagicalImmunity`
    
    Adds immunity to non-magical attacks.
    */
    NonmagicalImmunity,

    /**
    `NonSilveredImmunity`
    
    Adds immunity to non-silvered and non-magical attacks.
    */
    NonSilveredImmunity,
    
    /**
    `NonAdamantineImmunity`
    
    Adds immunity to non-adamantine and non-magical attacks.
    */
    NonAdamantineImmunity,

    /**
    `CustomImmunity(<string>)`

    Overrides the text of the immunities property.
    */
    CustomImmunity(String),

    /**
    `Languages([<Language>])`

    Adds the specified languages to the creature. See [`crate::stats::Language`] for possible values.

    */
    Languages(Vec<Language>),

    /**
    `UnspokenLanguages([<Language>])`

    Adds the specified languages to the creature as unspoken. See [`crate::stats::Language`] for possible values.

    */
    UnspokenLanguages(Vec<Language>),

    /**
    `Darkvision(<integer>)`
    
    Adds a darkvision sense to the creature with the specified distance range, or removes it if the value is zero. 
    */
    Darkvision(u8),

    /**
    `Blindsight(<integer>)`
    
    Adds a blindsight sense to the creature with the specified distance range, or removes it if the value is zero. 
    */
    Blindsight(u8),

    /**
    `BlindsightBlindBeyond(<integer>)`
    
    Adds a blindsight sense to the creature with the specified distance range and the clause "blind beyond", or removes it if the value is zero. 
    */
    BlindsightBlindBeyond(u8), 

    /**
    `Truesight(<integer>)`
    
    Adds a truesight sense to the creature with the specified distance range, or removes it if the value is zero. 
    */
    Truesight(u8),

    /**
    `Tremorsense(<integer>)`
    
    Adds a tremorsense sense to the creature with the specified distance range, or removes it if the value is zero. 
    */
    Tremorsense(u8),

    /**
    `CustomSense(<string>,<integer>)`

    Adds a custom sense to the creature with the specified name and distance range, or removes it if the value is zero.
    */
    CustomSense(String,u8),


    /**
    `ExpectNoChallenge`

    "Expects" the creature to have no challenge rating (0 and 0 XP) when the creature is complete.

    The challenge rating expectations will cause an error to occur if the calculated challenge rating is different from expected when creature creation is complete. Further expectations will override previous ones. Note that currently the only way to establish a challenge rating is with the Override challenge rating commands. Calculating challenge ratings remains a goal of Monstorr, but has been deferred to some time in the future.
    
    */
    ExpectNoChallenge, 

    /**
    `ExpectChallenge(<integer>)`

    "Expects" the creature to have a challenge rating of the specified whole number when complete. This number can be 0, which is different from no challenge, as it still provides 10 XP.
    
    The challenge rating expectations will cause an error to occur if the calculated challenge rating is different from expected when creature creation is complete. Further expectations will override previous ones. Note that currently the only way to establish a challenge rating is with the Override challenge rating commands. Calculating challenge ratings remains a goal of Monstorr, but has been deferred to some time in the future.
    
    */
    ExpectChallenge(u8),

    /**
    `ExpectHalfChallenge`

    "Expects" the creature to have a challenge rating of 1/2 when complete.
    
    The challenge rating expectations will cause an error to occur if the calculated challenge rating is different from expected when creature creation is complete. Further expectations will override previous ones. Note that currently the only way to establish a challenge rating is with the Override challenge rating commands. Calculating challenge ratings remains a goal of Monstorr, but has been deferred to some time in the future.
    
    */
    ExpectHalfChallenge,

    /**
    `ExpectQuarterChallenge`
    
    "Expects" the creature to have a challenge rating of 1/4 when complete.
    
    The challenge rating expectations will cause an error to occur if the calculated challenge rating is different from expected when creature creation is complete. Further expectations will override previous ones. Note that currently the only way to establish a challenge rating is with the Override challenge rating commands. Calculating challenge ratings remains a goal of Monstorr, but has been deferred to some time in the future.
    
    */
    ExpectQuarterChallenge,

    /**
    `ExpectEighthChallenge`
    
    "Expects" the creature to have a challenge rating of 1/8 when complete.
    
    The challenge rating expectations will cause an error to occur if the calculated challenge rating is different from expected when creature creation is complete. Further expectations will override previous ones. Note that currently the only way to establish a challenge rating is with the Override challenge rating commands. Calculating challenge ratings remains a goal of Monstorr, but has been deferred to some time in the future.
    
    */
    ExpectEighthChallenge,


    // these specify the challenge rating and related details
    /**
    `OverrideNoChallenge`

    Sets the challenge rating to 0 (0 XP).
    
    */
    OverrideNoChallenge,

    /**
    `OverrideChallenge(<integer>)`
    
    Sets the challenge rating to specified whole number. This number can be 0, which is different from no challenge, as it still provides 10 XP.
    */
    OverrideChallenge(u8),

    /**
    `OverrideHalfChallenge`
    
    Sets the challenge rating to 1/2.
    */
    OverrideHalfChallenge,

    /**
    `OverrideQuarterChallenge`
    
    Sets the challenge rating to 1/4.
    */
    OverrideQuarterChallenge,

    /**
    `OverrideEighthChallenge`
    
    Sets the challenge rating to 1/8.
    */
    OverrideEighthChallenge,


    /**
    `Multiattack(<string>,<Multiattack>)`

    Adds a "Multiattack" action to the creature. The first argument is the description to use for the action, which can't be easily calculated from the other data given. The second argument is a structure which describes what actions are possible in the multiattack, which will be used someday by Monstorr to calculate challenge ratings. Specifying them now will make it easier to get that functionality when it is available.

    For more information on the Multiattack argument, see [`crate::attacks::Multiattack`].
    */
    Multiattack(String,Multiattack), // description, details for cr calculation


    /**
    `Weapon(<Weapon>)`
    `Weapon(<Weapon>,optional(<CompoundAttackEffect>))`

    Adds a built=in weapon attack action to the creature, using default stats to build the description. The optional compound attack effect is for when the weapon causes additional damage or other effects on a hit, due to the monster's powers, which have nothing to do with the known weapon data. For more information on the arguments, see [`crate::attacks::Weapon`] and [`crate::attacks::CompoundAttackEffect`].
    */
    Weapon(Weapon,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")] 
        Option<CompoundAttackEffect>),

    /**
    `ExpectWeaponAttack(<Weapon>,<Attack>)`

    Sometimes a small change to another stat can change what a weapon's attack and hit bonuses, which can change your carefully balanced encounters. If this begins to be a problem, you can use this command to make sure the weapon attack has not changed when you make those small changes. This will report an error if the attack automatically calculated from the weapon is different than what is expected. For information on the arguments, see [`crate::attacks::Weapon`] and [`crate::attacks::Attack`].
    */
    ExpectWeaponAttack(Weapon,Attack),

    /**
    `ExpectWeaponEffect(<Weapon>,<AttackEffect>)`

    Sometimes a small change to another stat can change what a weapon's attack and hit bonuses, which can change your carefully balanced encounters. If this begins to be a problem, you can use this command to make sure the weapon's damage effect has not changed when you make those small changes. This will report an error if the effect automatically calculated from the weapon is different than what is expected. For information on the arguments, see [`crate::attacks::Weapon`] and [`crate::attacks::AttackEffect`].
    */
    ExpectWeaponEffect(Weapon,AttackEffect),

    /**
    `OverrideWeaponAttack(<Weapon>,<Attack>)`

    If you disagree with the calculated information about a weapon's attack, you can override it with this command. For information on the arguments, see [`crate::attacks::Weapon`] and [`crate::attacks::Attack`].
    */
    OverrideWeaponAttack(Weapon,Attack),

    /**
    `OverrideWeaponEffect(<Weapon>,<AttackEffect>)`
    
    If you disagree with the calculated information about a weapon's damage effects, you can override it with this command. For information on the arguments, see [`crate::attacks::Weapon`] and [`crate::attacks::Attack`].
    */
    OverrideWeaponEffect(Weapon,AttackEffect),

    /**
    `OverrideWeaponDescription(<Weapon>,<string>)` 

    If you disagree with the final description created for a weapon's damage effects, you can override it with this command. This description will be interpolated (see [`crate::interpolation`]).
    */
    OverrideWeaponDescription(Weapon,String), 


    /**
    `Action(<Action>)`
    `Action(<Action>,option(<UsageLimit>))`

    Adds a non-weapon action to the creature. The description will be calculated based on the information you provide about the action. You can specify a usage limit for the action, such as once per day, requiring a roll on 1d6, etc. For more information on the values you provide here, see [`crate::actions::Action`] and [`crate::actions::UsageLimit`].
    */
    Action(Action,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        Option<UsageLimit>),

    /**
    `OverrideActionDescription(<string>,<string>)`

    If you disagree with the final description created for an attack, this replaces the calculated description for the action specified in the first argument, with the text in the second. The description will be interpolated (see [`crate::interpolation`]).

    */
    OverrideActionDescription(String,String), 
    
    /**
    `RemoveWeapon(<Weapon>)`

    Removes the first action matching the specified weapon from the list of actions. See [`crate::attacks::Weapon`] for possible values.
    */
    RemoveWeapon(Weapon),

    /**
    `RemoveAction(<string>)`

    Removes the first action with the specified name from the list of actions.
    */
    RemoveAction(String),

    /**
    `Reaction(<Reaction>)`
    `Reaction(<Reaction>,option(<UsageLimit>))`

    Adds a reaction. For more information on the arguments, see [`crate::reactions::Reaction`] and [`crate::actions::UsageLimit`].
    */
    Reaction(Reaction,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        Option<UsageLimit>),

    /**
    `RemoveReaction(<string>)`

    Removes the first reaction with the specified name.
    */
    RemoveReaction(String),

    /**
    `Feature(<Feature>)`
    `Feature(<Feature>,option(<UsageLimit>))`

    Adds a feature. Some features have effects on other parts of the stat block. Do not use this to add spellcasting and innate spellcasting features, those have their own commands. For more information on the arguments, see [`crate::features::Feature`] and [`crate::actions::UsageLimit`].
    */
    Feature(Feature,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        Option<UsageLimit>),
    
    /**
    `RemoveFeature(<string>)`

    Removes the first feature with the specified name. This can also be used to remove spellcasting and innate spellcasting.
    */
    RemoveFeature(String),

    /**
    `Spellcasting([<SpellcastingCommand>...])`

    Calls the listed commands to add spellcasting features to the creature. For more information on what commands are available, see [`SpellcastingCommand`].
    */
    Spellcasting(Vec<SpellcastingCommand>),

    /**
    `InnateSpellcasting([<InnateSpellcastingCommand>...])`

    Calls the listed commands to add innate spellcasting features to the creature. For more information on what commands are available, see [`InnateSpellcastingCommand`].
    */
    InnateSpellcasting(Vec<InnateSpellcastingCommand>),

    /**
    `LegendaryActions(<integer>,[<LegendaryAction>...])`

    Adds legendary actions to the creature. The first argument is the number of legendary actions the creature can take per round. The second is the list of actions. For more information on this, see [`crate::actions::LegendaryAction`].
    */
    LegendaryActions(u8,Vec<LegendaryAction>), 
    /**
    `RemoveLegendaryAction(<string>)`

    Removes the specified legendary action by name from the creature. When the last one is removed, will remove the legendary action feature.
    */
    RemoveLegendaryAction(String),

    /** Currently Unsupported */
    LairActions(String,Vec<String>,String), // beginning description, list of lair actions, ending description
    /** Currently Unsupported */
    RegionalEffects(String,Vec<String>,String), // beginning description, list of regional effects, ending description


}

pub const MONSTORR_VERSION: f32 = 1.0;

impl CreatureCommand {

    fn execute(&self, working_dir: &PathBuf, creature: &mut Creature, hooks: &mut CreatureCreationHooks) -> Result<(),CreatureError> {
        
        match self {
            CreatureCommand::Monstorr(min,max) => {
                if *min > MONSTORR_VERSION {
                    Err(CreatureError::MonstorrVersionNotSupported(MONSTORR_VERSION))?
                }
                if let Some(max) = max {
                    if *max < MONSTORR_VERSION {
                        Err(CreatureError::MonstorrVersionNotSupported(MONSTORR_VERSION))?
                    }
                }
            },
            CreatureCommand::Include(file,parameters) => {
                // make it absolute
                let mut source_file = working_dir.clone();
                source_file.push(file);
                // get the data from the file
                let source = fs::read_to_string(&source_file).map_err(|a| CreatureError::include_error(file,a))?;
                // interpolate the text
                let interpolated = interpolate_str_for_deserialization(&source, file, parameters,false).map_err(|a| CreatureError::include_error(file,a))?;
                // deserialize the commands
                let commands = CreatureCreator::load_from_str(&interpolated).map_err(|a| CreatureError::include_error(file,a))?;
                source_file.pop();
                commands.apply_commands(&source_file,creature,hooks).map_err(|a| CreatureError::include_error(file,a))?
            },
            CreatureCommand::Source(name) => creature.set_source(name),
            CreatureCommand::Name(name) => creature.set_name(name),
            CreatureCommand::SubjectName(name) => creature.set_subject(name),
            CreatureCommand::CapitalizeSubjectName(name) => creature.set_capitalized_subject(name),
            CreatureCommand::PossessiveName(name) => creature.set_possessive(name),
            CreatureCommand::CapitalizePossessiveName(name) => creature.set_capitalized_possessive(name),
            CreatureCommand::Tiny => creature.set_tiny(),
            CreatureCommand::Small => creature.set_small(),
            CreatureCommand::Medium => creature.set_medium(),
            CreatureCommand::Large => creature.set_large(),
            CreatureCommand::Huge => creature.set_huge(),
            CreatureCommand::Gargantuan => creature.set_gargantuan(),
            CreatureCommand::Aberration => creature.set_aberration(),
            CreatureCommand::Beast => creature.set_beast(),
            CreatureCommand::Celestial => creature.set_celestial(),
            CreatureCommand::Construct => creature.set_construct(),
            CreatureCommand::Dragon => creature.set_dragon(),
            CreatureCommand::Elemental => creature.set_elemental(),
            CreatureCommand::Fey => creature.set_fey(),
            CreatureCommand::Fiend => creature.set_fiend(),
            CreatureCommand::Giant => creature.set_giant(),
            CreatureCommand::Humanoid => creature.set_humanoid(),
            CreatureCommand::Monstrosity => creature.set_monstrosity(),
            CreatureCommand::Ooze => creature.set_ooze(),
            CreatureCommand::Plant => creature.set_plant(),
            CreatureCommand::Undead => creature.set_undead(),
            CreatureCommand::CreatureType(name) => creature.set_custom_type(name),
            CreatureCommand::Subtype(subtype) => creature.set_subtype(subtype),
            CreatureCommand::Group(group) => creature.set_group(group),
            CreatureCommand::AnyAlignment => creature.set_any_alignment(),
            CreatureCommand::AnyNonGood => creature.set_any_non_good(),
            CreatureCommand::AnyNonEvil => creature.set_any_non_evil(),
            CreatureCommand::AnyNonLawful => creature.set_any_non_lawful(),
            CreatureCommand::AnyNonChaotic => creature.set_any_non_chaotic(),
            CreatureCommand::AnyGood => creature.set_any_good(),
            CreatureCommand::AnyEvil => creature.set_any_evil(),
            CreatureCommand::AnyLawful => creature.set_any_lawful(),
            CreatureCommand::AnyChaotic => creature.set_any_chaotic(),
            CreatureCommand::LawfulGood => creature.set_lawful_good(),
            CreatureCommand::NeutralGood => creature.set_neutral_good(),
            CreatureCommand::ChaoticGood => creature.set_chaotic_good(),
            CreatureCommand::LawfulNeutral => creature.set_lawful_neutral(),
            CreatureCommand::Neutral => creature.set_neutral(),
            CreatureCommand::ChaoticNeutral => creature.set_chaotic_neutral(),
            CreatureCommand::LawfulEvil => creature.set_lawful_evil(),
            CreatureCommand::NeutralEvil => creature.set_neutral_evil(),
            CreatureCommand::ChaoticEvil => creature.set_chaotic_evil(),
            CreatureCommand::Unaligned => creature.set_unaligned(),
            CreatureCommand::Alignment(alignment) => creature.set_custom_alignment(alignment),
            CreatureCommand::HitDie(die) => creature.set_hit_die(die),
            CreatureCommand::HitDiceCount(count) => creature.set_hit_dice_count(count),
            CreatureCommand::HitPoints(points) => creature.set_hit_points_override(points),
            CreatureCommand::Armor(armor) => creature.set_armor(armor),
            CreatureCommand::Shield => creature.enable_shield(),
            CreatureCommand::NoShield => creature.disable_shield(),
            CreatureCommand::Walk(speed) => creature.walk(speed),
            CreatureCommand::Swim(speed) => creature.swim(speed),
            CreatureCommand::Fly(speed) => creature.fly(speed),
            CreatureCommand::Burrow(speed) => creature.burrow(speed),
            CreatureCommand::Climb(speed) => creature.climb(speed),
            CreatureCommand::Hover => creature.enable_hover(),
            CreatureCommand::SpeedNotes(notes) => creature.speed_notes(notes),
            CreatureCommand::Speed(movement,speed) => creature.custom_speed(movement, speed),
            CreatureCommand::Str(score) => creature.set_str(score),
            CreatureCommand::Dex(score) => creature.set_dex(score),
            CreatureCommand::Con(score) => creature.set_con(score),
            CreatureCommand::Int(score) => creature.set_int(score),
            CreatureCommand::Wis(score) => creature.set_wis(score),
            CreatureCommand::Cha(score) => creature.set_cha(score),
            CreatureCommand::Saves(abilities) => creature.add_saves(abilities),
            CreatureCommand::Skills(skills) => creature.add_skills(skills),
            CreatureCommand::Expertise(skills) => creature.add_expertise(skills),
            CreatureCommand::RemoveSaves(abilities) => creature.remove_saves(abilities),
            CreatureCommand::RemoveSkills(skills) => creature.remove_skills(skills),
            CreatureCommand::ConditionImmunity(condition) => creature.add_condition_immunity(condition),
            CreatureCommand::Vulnerability(damage) => creature.add_vulnerability(damage),
            CreatureCommand::AllVulnerability => creature.add_all_vulnerability(),
            CreatureCommand::CustomVulnerability(name) => creature.add_custom_vulnerability(name),
            CreatureCommand::Resistance(damage) => creature.add_resistance(damage),
            CreatureCommand::AllResistance => creature.add_all_resistance(),
            CreatureCommand::NonmagicalResistance => creature.add_nonmagical_resistance(),
            CreatureCommand::NonSilveredResistance => creature.add_nonsilvered_resistance(),
            CreatureCommand::NonAdamantineResistance => creature.add_nonadamantine_resistance(),
            CreatureCommand::CustomResistance(custom) => creature.add_custom_resistance(custom),
            CreatureCommand::Immunity(damage) => creature.add_immunity(damage),
            CreatureCommand::AllImmunity => creature.add_all_immunities(),
            CreatureCommand::NonmagicalImmunity => creature.add_nonmagical_immunity(),
            CreatureCommand::NonSilveredImmunity => creature.add_nonsilvered_immunity(),
            CreatureCommand::NonAdamantineImmunity => creature.add_nonadamantine_immunity(),
            CreatureCommand::CustomImmunity(custom) => creature.add_custom_immunity(custom),
            CreatureCommand::Languages(languages) => creature.add_languages(languages),
            CreatureCommand::UnspokenLanguages(languages) => creature.add_unspoken_languages(languages),
            CreatureCommand::Darkvision(distance) => creature.add_darkvision(distance),
            CreatureCommand::Blindsight(distance) => creature.add_blindsight(distance),
            CreatureCommand::BlindsightBlindBeyond(distance) => creature.add_blindsight_blind_beyond(distance),
            CreatureCommand::Truesight(distance) => creature.add_truesight(distance),
            CreatureCommand::Tremorsense(distance) => creature.add_tremorsense(distance),
            CreatureCommand::CustomSense(sense,distance) => creature.add_custom_sense(sense, distance),
            CreatureCommand::ExpectNoChallenge => hooks.expected_challenge_rating = Some(ChallengeRating::None),
            CreatureCommand::ExpectChallenge(cr) => hooks.expected_challenge_rating = Some(ChallengeRating::Whole(*cr)),
            CreatureCommand::ExpectHalfChallenge => hooks.expected_challenge_rating = Some(ChallengeRating::Half),
            CreatureCommand::ExpectQuarterChallenge => hooks.expected_challenge_rating = Some(ChallengeRating::Quarter),
            CreatureCommand::ExpectEighthChallenge => hooks.expected_challenge_rating = Some(ChallengeRating::Eighth),
            CreatureCommand::OverrideNoChallenge => creature.set_no_challenge_rating(),
            CreatureCommand::OverrideChallenge(cr) => creature.set_challenge_rating(cr),
            CreatureCommand::OverrideHalfChallenge => creature.set_half_challenge_rating(),
            CreatureCommand::OverrideQuarterChallenge => creature.set_quarter_challenge_rating(),
            CreatureCommand::OverrideEighthChallenge => creature.set_eighth_challenge_rating(),
            CreatureCommand::Multiattack(description,details) => {
                hooks.multiattacks.push(details.clone());
                creature.set_multiattack(description.to_owned(), details);
            }
            CreatureCommand::Weapon(weapon,compound) => creature.add_weapon(weapon, compound),
            CreatureCommand::ExpectWeaponAttack(weapon,attack) => creature.expect_weapon_attack(weapon, attack)?,
            CreatureCommand::ExpectWeaponEffect(weapon,effect) => creature.expect_weapon_effect(weapon, effect)?,
            CreatureCommand::OverrideWeaponAttack(weapon,attack) => creature.override_weapon_attack(weapon, attack.clone())?,
            CreatureCommand::OverrideWeaponEffect(weapon,effect) => creature.override_weapon_effect(weapon, effect.clone())?,
            CreatureCommand::OverrideWeaponDescription(weapon,description) => creature.override_weapon_description(weapon, description.to_owned())?,
            CreatureCommand::Action(action,usage_limit) => creature.add_action(action, usage_limit),
            CreatureCommand::OverrideActionDescription(name,description) => creature.override_action_description(name, description.to_owned())?,
            CreatureCommand::RemoveWeapon(weapon) => creature.remove_weapon(weapon),
            CreatureCommand::RemoveAction(name) => creature.remove_action(name),
            CreatureCommand::Reaction(reaction,usage_limit) => creature.add_reaction(reaction.clone(), usage_limit.clone()),
            CreatureCommand::RemoveReaction(name) => creature.remove_reaction(name),
            CreatureCommand::Feature(feature,usage_limit) => {
                hooks.features.push(feature.clone());
                creature.add_feature(feature.clone(), usage_limit.clone());
            },
            CreatureCommand::RemoveFeature(name) => creature.remove_feature(name),
            CreatureCommand::Spellcasting(spellcasting_commands) => {
                let spellcasting = creature.get_or_add_spellcasting_mut();
                
                for command in spellcasting_commands {
                    command.execute(spellcasting)?;
                }
            },
            CreatureCommand::InnateSpellcasting(spellcasting_commands) => {
                let spellcasting = creature.get_or_add_innate_spellcasting_mut();
                
                for command in spellcasting_commands {
                    command.execute(spellcasting)?;
                }
            },
            CreatureCommand::LegendaryActions(total,actions) => {
                let description = LegendaryAction::get_main_description(total);
                let actions = actions.iter().map(|a| {
                    Ok(CreatureLegendaryAction::new(a,creature)?)
                }).collect::<Result<Vec<CreatureLegendaryAction>,CreatureError>>()?;
                creature.set_legendary_actions(description,actions);
                
            },
            CreatureCommand::RemoveLegendaryAction(name) => creature.remove_legendary_action(name),
            CreatureCommand::LairActions(..) => {
                Err(CreatureError::LairActionsNotSupportedYet)?
            },
            CreatureCommand::RegionalEffects(..) => {
                Err(CreatureError::RegionalEffectsNotSupportedYet)?
            }

        
        
        

            
            


        }
        Ok(())

    }
}


#[derive(PartialEq,Debug)]
#[derive(Serialize,Deserialize)]
/**
`([<CreatureCommand>...])`

A creature document contains a tuple with a single list inside it. The list contains tagged variants of the `CreatureCommand` type. The only required command is `Name`, which specifies the name of the creature. If no other command is provided, the resulting stat-block will have reasonable defaults for every other property. Ability scores will all be 10, size will be medium, no skills, no actions, etc.

See [`CreatureCommand`] for the allowed commands.

For some example creatures, use the 'validate' command on Monstorr to retrieve the 'stored' creatures such as 'dragon', 'goblin', 'bugbear', or 'efreeti'.


*/
pub struct CreatureCreator(pub Vec<CreatureCommand>);

impl CreatureCreator {

    pub fn create_creature(&self, working_dir: &PathBuf) -> Result<Creature,CreatureError> {
        let mut result = Creature::default();
        let mut hooks = CreatureCreationHooks::default();
        self.apply_commands(working_dir,&mut result,&mut hooks)?;

        for multiattack in hooks.multiattacks {
            result.check_multiattack(&multiattack)?
        }

        for feature in hooks.features {
            Self::apply_feature(&mut result,feature)?
        }

        if let Some(expected_challenge_rating) = hooks.expected_challenge_rating {
            if result.challenge_rating != expected_challenge_rating {
                Err(CreatureError::ChallengeRatingNotAsExpected(expected_challenge_rating.to_string(),result.challenge_rating.to_string()))?
            }

        }

        if result.name == "" {
            Err(CreatureError::CreatureHasNoName)
        } else {
            Ok(result)
        }



    }

    fn apply_commands(&self, working_dir: &PathBuf, creature: &mut Creature, hooks: &mut CreatureCreationHooks) -> Result<(),CreatureError> {
        for command in &self.0 {
            command.execute(working_dir,creature,hooks)?
        }
        Ok(())
    
    }

    fn apply_feature(creature: &mut Creature, feature: Feature) -> Result<(),CreatureError> {
        match &feature {
            Feature::Brute(actions) => {
                // "A melee weapon deals one extra die of its damage when ${{subj}} hits with it (included in the attack)."
                // for each action
                // - if it exists:
                //   - if it is an attack
                //     - if it has a 'reach' and no range:
                //       - add one die to the die expression in the effect
                //       - regenerate the attack's description
                //     - if it has a reach and a range
                //       - change the effect to a Double Or
                //       - the first clause adds one die to the die expression
                //       - the first condition is "in melee"
                //       - the second condition is "at range"
                for name in actions {
                    if let Some(action) = creature.find_action_mut(&name) {
                        if let Some(attack) = &action.attack {
                            if let Some(AttackEffect::Damage(effect_dice,effect_bonus,effect_damage)) = &action.effect {
                                let new_dice = effect_dice.coefficient_add(1);
                                let new_effect = match (attack.reach,attack.range) {
                                    (Some(_),None) => {
                                        AttackEffect::Damage(new_dice,
                                                             effect_bonus.clone(),
                                                             effect_damage.clone())
                                    }
                                    (Some(_),Some(_)) => {
                                        AttackEffect::DoubleOr(new_dice,
                                                               effect_bonus.clone(),
                                                               effect_damage.clone(),
                                                               "in melee".to_owned(),
                                                               effect_dice.clone(),
                                                               effect_bonus.clone(),
                                                               effect_damage.clone(),
                                                               "at range".to_owned())
                                    },
                                    (None,_) => {
                                        Err(CreatureError::InvalidStateForFeature(format!("Brute can not be applied to the '{}' action, which is not a melee attack.",name)))?
                                    }
                                };
                                action.description = attack.get_description(Some(&new_effect), &action.compound);
                                action.effect = Some(new_effect);


                            } else {
                                Err(CreatureError::InvalidStateForFeature(format!("Brute can not be applied to the '{}' action, which does not have a standard damage effect.",name)))?;
                            }

                        } else {
                            Err(CreatureError::InvalidStateForFeature(format!("Brute can not be applied to the '{}' action, which is not an attack.",name)))?;
                        }

                    } else {
                        Err(CreatureError::ActionNotFound(name.clone(),"while applying brute feature".to_owned()))?
                    }


                };




            },
            Feature::AngelicWeapons(extra,actions) => {

                for name in actions {
                    if let Some(action) = creature.find_action_mut(&name) {
                        if let Some(_) = &action.attack {
                            if let Some(_) = &action.compound {
                                Err(CreatureError::InvalidStateForFeature(format!("Angelic Weapons can not be applied to the '{}' action, which already has a compound effect.",name)))?
                            } else {
                                action.compound = Some(CompoundAttackEffect::Plus(AttackEffect::Damage(extra.clone(),AttackBonus::Fixed(0),Damage::Radiant)))
                            }

                        } else {
                            Err(CreatureError::InvalidStateForFeature(format!("Angelic Weapons can not be applied to the '{}' action, which is not an attack.",name)))?;
                        }

                    } else {
                        Err(CreatureError::ActionNotFound(name.clone(),"while applying angelic weapons feature".to_owned()))?
                    }


                };

            },
            _ => ()
        };
        Ok(())


    }

    pub fn save_to_string(&self) -> Result<String,ron::Error> {
        ron::ser::to_string_pretty(&self,ron::ser::PrettyConfig::new())
    }

    pub fn load_from_str(data: &str) -> Result<Self,ron::Error> {
        ron::de::from_str(data)
    }
}
