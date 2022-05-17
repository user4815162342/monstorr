/*
 * Copyright © 2022 Neil M. Sheldon
 * 
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use serde::Deserialize;
use serde::Serialize;

use crate::stats::Damage;
use crate::dice::Die;
use crate::dice::Dice;
use crate::dice_expression::DiceExpression;
use crate::stats::Ability;
use crate::stats::CreatureSize;



// An AttackEffect specifies the result of an attack hitting a target
#[derive(PartialEq,Debug,Clone)]
#[derive(Serialize,Deserialize)]
/**
An attack effect represents the results of an attack, or an action in general. Usually, this effect is to cause damage to the target, but there are also other choices. Many of the variants here are based on conditional results, which cause different damage or types of damage based on that condition. These conditions change the wording of the descriptions created with the effect, even if they don't change the damage per round.

You are usually looking for the `Damage` variant. Some other value types referenced include [`AttackBonus`] and [`crate::stats::Damage`]. The attack bonus is a way to reference the stats of the creature to generate the bonus.

All string arguments are subject to interpolation in the final description.
*/
pub enum AttackEffect {
    // FUTURE: I'm using DiceExpression here instead of Dice, despite having a separate bonus, because that's
    // the only way to add the magic bonus to the dice. See the next note regarding optional arguments for why I
    // don't just have a <dice> and an optional magic bonus. This one could also be fixed with a custom deserialization.
    // FUTURE: Attempting to "skip" the AttackBonus arguments breaks things since the deserialization can't figure
    // out that it should go on to the next if it can't find it. That sort of thing only works in the last argument.
    // the only solutions I can think of are to change these to structs, in which case it gets too wordy, or
    // customize the serializing/deserializing, which is not worth the trouble since serde makes these things
    // harder than they should be. A future version of monstorr which does it's own parsing (since we've implemented
    // a parser anyway) might fix all of this, as it would allow me to specify true function overloads, but that
    // is reserved for some day in the future instead.

    /**
    `Damage(<dice-expression-string>,<AttackBonus>,<Damage>)`

    The attack causes the rolled amount of specified damage type.
    */
    Damage(DiceExpression,AttackBonus,Damage),
    /**
    `FixedDamage(<integer>,<AttackBonus>,<Damage>)`

    The attack causes a fixed amount of damage every time it is used. This is used for attacks like unarmed strikes, where the damage started as a fixed value of 1, but with magic or size bonuses increased by a specified amount. It really shouldn't be used for regular attacks.
    */
    FixedDamage(u8,AttackBonus,Damage), // fixed amount, magic, bonus, damage type
    /**
    `Special(<string>)`

    The attack causes an effect that would not change damage per round. This might be a condition effect such as grappled or poisoned, or non-immediate damage that continues per turn until something happens. These would have to be artificially added to the damage per round later.

    In order to make descriptions work, the sentence should be worded something like "the target is...".
    */
    Special(String),
    /**
    `Or(<dice-expression-string>,<AttackBonus>,<Damage>,<dice-expression-string>,<AttackBonus>,<Damage>,<String>)`

    If an attack can have two results depending on some condition, this can be used. The wording of the attack is will be "<damage> or <damage> <condition>.". You should specify an "if", "while" or other conditional in the last string.
    */
    Or(DiceExpression,AttackBonus,Damage,DiceExpression,AttackBonus,Damage,String),
    /**
    `DoubleOr(<dice-expression-string>,<AttackBonus>,<Damage>,<string>,<dice-expression-string>,<AttackBonus>,<Damage>,<String>)`

    This is similare to `or`, except there is a condition attached to both results. The wording of the attack is will be "<damage> <first condition> or <damage> <second condition>.". You should specify an "if", "while" or other conditional in the strings.
    */
    DoubleOr(DiceExpression,AttackBonus,Damage,String,DiceExpression,AttackBonus,Damage,String),
    /**
    `DjinnisChoice(<dice-expression-string>,<AttackBonus>,<Damage>,<Damage>)`

    In the core books, the djinni has a choice between the damage types it can cause. This is abstracted from that, and will result in the wording "<dice> <damage> or <damage> damage (${poss} choice)".
    */
    DjinnisChoice(DiceExpression,AttackBonus,Damage,Damage),
    /**
    `SaveAll(<integer>,<Ability>,<dice-expression-string>,<AttackBonus>,<Damage>)`

    The target must make a saving roll, and only takes damage if they succeed. The initial number is the DC for the saving roll. See [`crate::stats::Ability`].
    */
    SaveAll(u8,Ability,DiceExpression,AttackBonus,Damage),
    /**
    `SaveHalf(<integer>,<Ability>,<dice-expression-string>,<AttackBonus>,<Damage>)`

    The target must make a saving roll, and takes half damage if they succeed. The initial number is the DC for the saving roll.  See [`crate::stats::Ability`].
    */
    SaveHalf(u8,Ability,DiceExpression,AttackBonus,Damage),


    /**
    `AreaDamage(<dice-expression-string>,<AttackBonus>,<Damage>)`

    This causes damage to all targets in an area.
    */
    AreaDamage(DiceExpression,AttackBonus,Damage),
    /**
    `AreaSaveAll(<integer>,<Ability>,<dice-expression-string>,<AttackBonus>,<Damage>)`

    This is the `SaveAll` version for `AreaDamage`.
    */
    AreaSaveAll(u8,Ability,DiceExpression,AttackBonus,Damage),
    /**
    `AreaSaveHalf(<integer>,<Ability>,<dice-expression-string>,<AttackBonus>,<Damage>)`

    This is the `SaveHalf` version for `AreaDamage`.
    */
    AreaSaveHalf(u8,Ability,DiceExpression,AttackBonus,Damage),
}

impl AttackEffect {

    fn get_base_description(&self, default_bonus: &str) -> String {
        match self {
            AttackEffect::FixedDamage(amount,bonus,damage) => 
                format!("${{{} + {}}} {} damage",
                        amount,
                        bonus.get_expr(default_bonus,false),
                        damage),
            AttackEffect::Damage(dice,bonus,damage) => 
                format!("${{{} + {}}} {} damage",
                        dice.serialize_to_string(),
                        bonus.get_expr(default_bonus,false),
                        damage),
            AttackEffect::Special(special) => 
                special.clone(),
            AttackEffect::Or(dice,bonus,damage,alt_dice,alt_bonus,alt_damage,condition) => 
                format!("${{{} + {}}} {} damage, or ${{{} + {}}} {} damage {}",
                        dice.serialize_to_string(),
                        bonus.get_expr(default_bonus,false),
                        damage,
                        alt_dice.serialize_to_string(),
                        alt_bonus.get_expr(default_bonus,false),
                        alt_damage,
                        condition),
            AttackEffect::DoubleOr(dice,bonus,damage,condition,alt_dice,alt_bonus,alt_damage,alt_condition) => 
                format!("${{{} + {}}} {} damage {} or ${{{} + {}}} {} damage {}",
                        dice.serialize_to_string(),
                        bonus.get_expr(default_bonus,false),
                        damage,
                        condition,
                        alt_dice.serialize_to_string(),
                        alt_bonus.get_expr(default_bonus,false),
                        alt_damage,
                        alt_condition),
            // The Djinni, at least, has a "plus" effect for which it can choose between two types of damage
            AttackEffect::DjinnisChoice(dice,bonus,damage,alt_damage) =>
               format!("${{{} + {}}} {} or {} damage (${{poss}} choice)",
                       dice.serialize_to_string(),
                       bonus.get_expr(default_bonus,false),
                       damage,
                       alt_damage),
            // Represents an attack result which only causes damage if the target fails to save on a certain ability.
            AttackEffect::SaveAll(save_dc,save_ability,dice,bonus,damage) =>
               format!("The target must make a DC {} {} saving throw, taking ${{{} + {}}} {} damage on a failed save",
                       save_dc,
                       save_ability,
                       dice.serialize_to_string(),
                       bonus.get_expr(default_bonus,false),
                       damage),
            AttackEffect::SaveHalf(save_dc,save_ability,dice,bonus,damage) =>
               format!("The target must make a DC {} {} saving throw, taking ${{{} + {}}} {} damage on a failed save, or half as much damage on a successful one",
                       save_dc,
                       save_ability,
                       dice.serialize_to_string(),
                       bonus.get_expr(default_bonus,false),
                       damage),
            AttackEffect::AreaDamage(dice,bonus,damage) => // FUTURE: I don't know which creature I got this from, so wording might be wrong.
                format!("Each target in the area takes ${{{} + {}}} {} damage",
                       dice.serialize_to_string(),
                       bonus.get_expr(default_bonus,false),
                       damage),
            AttackEffect::AreaSaveAll(save_dc,save_ability,dice,bonus,damage) =>
                format!("Each target in the area must make a DC {} {} saving throw, taking ${{{} + {}}} {} damage on a failed save",
                        save_dc,
                        save_ability,
                        dice.serialize_to_string(),
                        bonus.get_expr(default_bonus,false),
                        damage),
            AttackEffect::AreaSaveHalf(save_dc,save_ability,dice,bonus,damage) =>
                format!("Each target in the area must make a DC {} {} saving throw, taking ${{{} + {}}} {} damage on a failed save, or half as much damage on a successful one",
                        save_dc,
                        save_ability,
                        dice.serialize_to_string(),
                        bonus.get_expr(default_bonus,false),
                        damage),
        }
    }

    pub fn get_description(&self, default_bonus: &str, compound: &Option<CompoundAttackEffect>) -> String {
        let base = self.get_base_description(default_bonus);
        if let Some(compound) = compound {

            match compound {
                CompoundAttackEffect::And(alt_attack) => 
                    format!("{}, and {}.",base,alt_attack.get_base_description(default_bonus)),
                CompoundAttackEffect::AndAnd(alt_attack,and) => 
                    format!("{}, and {} and {}.",base,alt_attack.get_base_description(default_bonus),and),
                CompoundAttackEffect::Additional(alt_attack) => 
                    format!("{}. {}.",base,alt_attack.get_base_description(default_bonus)),
                CompoundAttackEffect::AndAdditional(first_alt,second_alt) => 
                    format!("{}, and {}. {}.",base,first_alt.get_base_description(default_bonus),second_alt.get_base_description(default_bonus)),
                CompoundAttackEffect::Plus(alt) =>
                    format!("{} plus {}.",base,alt.get_base_description(default_bonus)),
                CompoundAttackEffect::PlusAdditional(first,second) =>
                    format!("{} plus {}. {}.",base,first.get_base_description(default_bonus),second.get_base_description(default_bonus)),
                CompoundAttackEffect::PlusAnd(first,second) => 
                    format!("{} plus {}, and {}.",base,first.get_base_description(default_bonus),second.get_base_description(default_bonus))
            }
        } else {
            format!("{}.",base)
        }

    }
}

// A CompoundAttackEffect specifies additional results of an attack hitting a target, much of these
// differ only in the wording provided in the calculated description
#[derive(PartialEq,Debug,Clone)]
#[derive(Serialize,Deserialize)]
/**
Some creatures cause extra damage or other effects when they have a successful attack. It was simpler to model this as a separate structure, rather than add a lot more choices to the [`Attack`] variant. Many of the variants in the CompoundAttackEffect simply add one more effect. However, the compound attack effect can be used to generate a description, and the variants given can be used to generate descriptions for 95% of the attacks on creatures in the core books.

For more information on the actual effects, see [`AttackEffect`].

*/
pub enum CompoundAttackEffect {
    /**
    `And(<AttackEffect>)`

    The resulting description of the attack has the format "<original effect>, and <next effect>."
    */
    And(AttackEffect),
    /**
    `AndAnd(<AttackEffect>,<string>)`

    This is used where the second effect is a SavAll or SaveHalf, and there are additional effects that occur if the save is failed.

    The resulting description of the attack has the format "<original effect>, and <next effect> and <string>." The string can contain interpolation content.

    */
    AndAnd(AttackEffect,String),
    /**
    `Additional(<AttackEffect>,<string>)`

    This is similar to `AndAnd`, but used where the additional effects occur all the time. Usually, the next effect is a `Special` that doesn't change damage per round.

    The resulting description of the attack has the format "<original effect>. <next effect>."
    */
    Additional(AttackEffect),
    /**
    `AndAdditional(<AttackEffect>,<AttackEffect>)`

    This is a combination of `And` and `Additional`.

    The resulting description of the attack has the format "<original effect>, and <next efffect>. <last effect>."
    */
    AndAdditional(AttackEffect,AttackEffect),
    /**
    `Plus(<AttackEffect>)`

    While similar to `And`, this changes the wording to use "plus". There is little semantic difference between the two, I believe that some of the writers of the core books just wanted to add a little variety. This wording does help with the `PlusAnd` variant, however.

    The resulting description of the attack has the format "<original effect> plus <next effect>."
    */
    Plus(AttackEffect),
    /**
    `PlusAdditional(<AttackEffect>,<AttackEffect>)`

    This is a combination of `Plus` and `Additional`.

    The resulting description of the attack has the format "<original effect> plus <next efffect>. <last effect>."
    */
    PlusAdditional(AttackEffect,AttackEffect),
    /**
    `PlusAnd(<AttackEffect>,<AttackEffect>)`

    This is great for when you have two damaging effects.

    The resulting description of the attack has the format "<original effect> plus <next effect>, and <last effect>."
    */
    // <original effect> + "plus (...), and (...)" -- the 'and' is usually non-DPR effect
    PlusAnd(AttackEffect,AttackEffect)

}


// FUTURE: I'd like to change how this (de)serializes:
// -- anywhere there is a Vec, I want the enum to flatten out the vec into arguments. So, Or([...]) becomes Or(...), count(1,[...]) becomes count(1,...)
// -- anywhere there an "Attack", it should be serialized only as a string
// but I can also see how those options might be inconsistent with the rest. Instead, perhaps go back to the original wording of things in the SRD and
// match the structures more to those somehow.
#[derive(PartialEq,Debug,Clone)]
#[derive(Serialize,Deserialize)]
/**
The description for a "Multiattack" action is written by hand, not generated from multiattack values. The Multiattack value's primary use is for calculating the Challenge Rating, as that calculation needs to know what attacks a creature can make in each round. While the challenge rating is not yet automatically calculated, it is good to have this information for when it will be.

A Multiattack is a simple structure complicated by multiple combinations. While some creatures can simply make any attacks of their attacks a certain number of times, others are limited in unusual ways. For example, a creature might be able to make one bite and two claw attacks. Another might make three melee, but only one spell, and if they attack with their staff they can only attack twice. There are few very complicated multiattacks in the official books, but being able to model even a mixed multiattack like that first requires a few unusual structures. 

Many of the Multiattack variants are recursive, taking vectors of other multiattacks as arguments. The other values simple specify what types of attacks should be used in those arguments. All attacks reference non-legendary actions.
 */
pub enum Multiattack {
    /**
    `Any`

    The creature can attack with any of its attacks.
    */
    Any,
    /**
    `Ranged`

    The creature can make any ranged attack once.
    */
    Ranged, 
    /**
    `Melee`

    The creature can make any melee attack once.
    */
    Melee, 
    /**
    `Spell`

    The creature can make a any spell attack once, or cast one spell that normally takes an action to do so.
    */
    Spell, 
    /**
    `Attack(<string>)`

    The creature takes the specified action once. If the attack has not been added by the end of creature completion, an error will be reported.
    */
    Attack(String), 
    /**
    `Weapon(<weapon>)`

    The creature attacks with the specified weapon once. If the weapon has not been added by the end of creature completion, an error will be reported.
    */
    Weapon(Weapon),
    /**
    `Except([<Multiattack>...])`

    The creature makes any attack, except the ones specified, once.
    */
    Except(Vec<Multiattack>), 
    /**
    `Or([<Multiattack>...])`

    The creature makes one of the attacks specified. 
    */
    Or(Vec<Multiattack>),
    /**
    `Count(<integer>,[<Multiattack>...])`

    The creature uses any of the listed attacks the specified number of times.
    */
    Count(u8,Vec<Multiattack>),
    /**
    `And([<Multiattack>...])`

    The creature makes all of the specified attacks.
    */
    And(Vec<Multiattack>),
    /**
    `Dice(<dice-expression-string>,[<Multiattack>...])`

    The creature makes any of the specified attacks the number of times determined by a dice roll.

    In the core books, this version is used by the Violet Fungus.
    */
    Dice(DiceExpression,Vec<Multiattack>) // roll the specified die to determine how many of any one of these specified attacks
}

#[derive(PartialEq,Debug,Clone)]
#[derive(Serialize,Deserialize)]
/**
These values are used to indicate what abilities attack bonuses will come from. For hit rolls, these bonuses are added to the proficiency bonus. For damage, these abilities are used alone.
*/
pub enum AttackBonus {
    /**
    `Default`

    Specifies that the attack bonus is determined by the type of attack. If it is a melee attack, use Strength, if it is ranged, use Dexterity. If it is both kinds of attacks, the attack is assumed to be a flexible weapon, and the best of each of those is chosen.
    */
    Default,
    /**
    `Strength`

    Use Strength for the bonus.
    */
    Strength, 
    
    /**
    `Dexterity`

    Use Dexterity for the bonus.
    */
    Dexterity,
    
    /**
    `Constitution`

    Use Constitution for the bonus.
    */
    Constitution,
    
    /**
    `Wisdom`

    Use Wisdom for the bonus.
    */
    Wisdom,
    
    /**
    `Intelligence`

    Use Intelligence for the bonus.
    */
    Intelligence,
    
    /**
    `Charisma`

    Use Charisma for the bonus.
    */
    Charisma,
    /**
    `Best`

    The attack bonus is calculated based on the best of Strength or Dexterity.
    */
    Best,
    /**
    `Spell`

    The attack bonus is calculated based on the creature's spell attack bonus. If the creature has both innate and regular spellcasting, the innate spellcasting bonus is taken. This will cause interpolation errors if the creature has no spellcasting ability.
    */
    Spell,
    /**
    `Fixed(<integer>)`

    The attack bonus is fixed to the specific value, and not based on ability.
    */
    Fixed(u8),
    /**
    `Zero`
    
    The attack bonus is 0. This has the same effect as `Fixed(0)
    */
    Zero
}

impl AttackBonus {

    fn is_default(&self) -> bool {
        if let AttackBonus::Default = self {
            true
        } else {
            false
        }
    }

    // get the interpolation expression
    pub fn get_expr(&self, default: &str, use_prof: bool) -> String {
        match self {
            AttackBonus::Default => format!("{}{}",default,if use_prof { " + prof" } else { "" }),
            AttackBonus::Strength => format!("str{}",if use_prof { " + prof" } else { "" }),
            AttackBonus::Dexterity => format!("dex{}",if use_prof { " + prof" } else { "" }),
            AttackBonus::Constitution => format!("con{}",if use_prof { " + prof" } else { "" }),
            AttackBonus::Wisdom => format!("wis{}",if use_prof { " + prof" } else { "" }),
            AttackBonus::Intelligence => format!("int{}",if use_prof { " + prof" } else { "" }),
            AttackBonus::Charisma => format!("cha{}",if use_prof { " + prof" } else { "" }),
            AttackBonus::Best => format!("atk{}",if use_prof { " + prof" } else { "" }),
            AttackBonus::Spell => format!("spell_atk{}",if use_prof { " + prof" } else { "" }),
            AttackBonus::Fixed(number) => number.to_string(),
            AttackBonus::Zero => "0".to_owned(),
        }
    }
}

impl Default for AttackBonus {

    fn default() -> Self {
        AttackBonus::Default
    }
}

impl std::convert::TryFrom<u8> for AttackBonus {

    type Error = String; // I just need something that implements display, since I won't ever be returning an error.

    fn try_from(input: u8) -> Result<Self,Self::Error> {
        Ok(AttackBonus::Fixed(input))
    }

}

#[derive(PartialEq,Debug,Clone)]
#[derive(Serialize,Deserialize)]
/**
Used for adding the words "Weapon" or "Spell" into the type of the attack.
*/
pub enum AttackType {
    /// `Weapon`
    Weapon,
    /// `Spell`
    Spell
}


#[derive(PartialEq,Debug,Clone)]
#[derive(Serialize,Deserialize)]
/**
`(type: option(<AttackType>)?, bonus: <AttackBonus>?, magic: option(<integer>)?, reach: option(<integer>)?, range: option(<integer>)?, long_range: option(<integer>)?, target: <string>)`

An Attack value is a mapped struct containing data needed to generating a description, of the attack attempt of an action. This is the part before the "hit:" phrasing. See below for what the various properties mean.

To specify that an attack can be melee, give it a reach property. To specify that it can be ranged, give it a range property.

*/
// FUTURE: I'd like to be able to make the optional properties simply not required in deserialization, with the 'Some' automatically added if it's not there. This and in all places.
pub struct Attack {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename="type")]
    /**
    `type: option(<AttackType>)?`

    Specifies the [`AttackType`]. The possible values are `Weapon` and `Spell`, but this is optional. The resulting phrasing will say things like "Melee Weapon Attack" or "Ranged Spell Attack", with the word removed if this is not specified.
    */
    pub type_: Option<AttackType>,
    #[serde(skip_serializing_if = "AttackBonus::is_default")]
    #[serde(default)]
    /**
    `bonus: <AttackBonus>?`

    Specifies the ability score to generate the normal bonus for the attack from, in addition to the proficiency bonus and any magic also included. See [`AttackBonus`].

    Note that while this property is not required, it is not an "optional" type, as the value has a 'default' of its own.
    */
    pub bonus: AttackBonus, 
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    /**
    `magic: option(<integer>)?`

    Specifies a fixed bonus (or penalty) to add to the hit roll, in addition to the ability and proficiency bonus. Not specifying this implies a value of 0.
    */
    pub magic: Option<i8>, // to add additional bonus onto a calculated attack.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    /**
    `reach: option(<integer>)?`

    Specifies that this can be a melee attack, and that this value is the reach.
    */
    pub reach: Option<u8>, // indicates that it's a melee attack and this is the reach.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    /**
    `range: option(<integer>)?`
    
    Specifies that this can be a ranged attack, and that this value is the range.
    */
    pub range: Option<u16>, // indicates that it's a ranged attack and this is the (short) range
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    /**
    `range: option(<integer>)?`
    
    Specifies the long range for a ranged attack. This is ignored unless `range` is also set.
    */
    pub long_range: Option<u16>, // indicates that it's a ranged attack and this is the long range (requires "range"),
    /**
    `target: <string>`

    Specifies the phrasing for the target of the attack. Usually "one creature", but occasionally its "one target", and even rarely something like "two Large or Smaller humanoids".
    */ 
    pub target: String, // indicates the type of targets (i.e. "one creature", "one target", "two Large or smaller humanoids", etc.)
}

impl Attack {

    pub fn get_description(&self, effect: Option<&AttackEffect>, compound_effect: &Option<CompoundAttackEffect>) -> String {
    
        let (target,melee,ranged) = match (self.reach,self.range,self.long_range) {
            (Some(reach),Some(short_range),Some(long_range)) => 
                (format!("reach {} ft. or range {}/{} ft., {}",reach,short_range,long_range,self.target),true,true),
            (Some(reach),Some(range),None) => 
                (format!("reach {} ft. or range {} ft., {}",reach,range,self.target),true,true),
            (Some(reach),None,_) => 
                (format!("reach {} ft., {}",reach,self.target),true,false),
            (None,Some(short_range),Some(long_range)) => 
                (format!("range {}/{} ft., {}",short_range,long_range,self.target),false,true),
            (None,Some(range),None) => 
                (format!("range {} ft., {}",range,self.target),false,true),
            (None,None,_) => 
                (format!("{}",self.target),false,false)
        };
        let attack_type = match self.type_ {
            Some(AttackType::Weapon) => "Weapon ",
            Some(AttackType::Spell) => "Spell ",
            None => ""
        };
        let (attack,default_bonus) = match(&self.bonus,self.magic,melee,ranged) {
            (bonus,Some(magic),true,true) => (format!("${{italic(}}Melee or Ranged {}Attack:${{)}} ${{+{} + {}}} to hit, {}.",attack_type,bonus.get_expr("atk",true),magic,target),"atk"),
            (bonus,None,true,true) => (format!("${{italic(}}Melee or Ranged {}Attack:${{)}} ${{+{}}} to hit, {}.",attack_type,bonus.get_expr("atk",true),target),"atk"),
            (bonus,Some(magic),true,false) => (format!("${{italic(}}Melee {}Attack:${{)}} ${{+{} + {}}} to hit, {}.",attack_type,bonus.get_expr("str",true),magic,target),"str"),
            (bonus,None,true,false) => (format!("${{italic(}}Melee {}Attack:${{)}} ${{+{}}} to hit, {}.",attack_type,bonus.get_expr("str",true),target),"str"),
            (bonus,Some(magic),false,true) => (format!("${{italic(}}Ranged {}Attack:${{)}} ${{+{} + {}}} to hit, {}.",attack_type,bonus.get_expr("dex",true),magic,target),"dex"),
            (bonus,None,false,true) => (format!("${{italic(}}Ranged {}Attack:${{)}} ${{+{}}} to hit, {}.",attack_type,bonus.get_expr("dex",true),target),"dex"),
            (bonus,Some(magic),false,false) => (format!("${{italic(}}{}Attack:${{)}} ${{+{} + {}}} to hit, {}.",attack_type,bonus.get_expr("atk",true),magic,target),"atk"),
            (bonus,None,false,false) => (format!("${{italic(}}{}Attack:${{)}} ${{+{}}} to hit, {}.",attack_type,bonus.get_expr("atk",true),target),"atk"),
        };
        if let Some(effect) = effect {
            let effect = format!("${{italic(}}Hit:${{)}} {}",effect.get_description(default_bonus,compound_effect));
            format!("{} {}",attack,effect)
        } else {
            attack
        }
    }    
}

#[derive(PartialEq,Debug,Clone)]
#[derive(Serialize,Deserialize)]
/**
These values are used to represent standard weapons in attacks. Custom weapons are made by added custom attack actions. All weapons take an integer which represents a bonus (or penalty) to be added to attack and damage rolls, such as might come from a magic weapon.
*/
pub enum Weapon { 

    // unarmed
    /// `UnarmedStrike(<integer>)`
    UnarmedStrike(i8),

    // simple melee
    /// `Club(<integer>)`
    Club(i8),
    /// `Dagger(<integer>)`
    Dagger(i8),
    /// `Greatclub(<integer>)`
    Greatclub(i8),
    /// `Handaxe(<integer>)`
    Handaxe(i8),
    /// `Javelin(<integer>)`
    Javelin(i8),
    /// `LightHammer(<integer>)`
    LightHammer(i8),
    /// `Mace(<integer>)`
    Mace(i8),
    /// `Quarterstaff(<integer>)`
    Quarterstaff(i8),
    /// `Sickle(<integer>)`
    Sickle(i8),
    /// `Spear(<integer>)`
    Spear(i8),
    // simple ranged
    /// `LightCrossbow(<integer>)`
    LightCrossbow(i8),
    /// `Dart(<integer>)`
    Dart(i8),
    /// `Shortbow(<integer>)`
    Shortbow(i8),
    /// `Sling(<integer>)`
    Sling(i8),
    // martial melee
    /// `Battleaxe(<integer>)`
    Battleaxe(i8),
    /// `Flail(<integer>)`
    Flail(i8),
    /// `Glaive(<integer>)`
    Glaive(i8),
    /// `Greataxe(<integer>)`
    Greataxe(i8),
    /// `Greatsword(<integer>)`
    Greatsword(i8),
    /// `Halberd(<integer>)`
    Halberd(i8),
    /// `Lance(<integer>)`
    Lance(i8),
    /// `Longsword(<integer>)`
    Longsword(i8),
    /// `Maul(<integer>)`
    Maul(i8),
    /// `Morningstar(<integer>)`
    Morningstar(i8),
    /// `Pike(<integer>)`
    Pike(i8),
    /// `Rapier(<integer>)`
    Rapier(i8),
    /// `Scimitar(<integer>)`
    Scimitar(i8),
    /// `Shortsword(<integer>)`
    Shortsword(i8),
    /// `Trident(<integer>)`
    Trident(i8),
    /// `WarPick(<integer>)`
    WarPick(i8),
    /// `Warhammer(<integer>)`
    Warhammer(i8),
    /// `Whip(<integer>)`
    Whip(i8),
    // martial ranged
    /// `Blowgun(<integer>)`
    Blowgun(i8),
    /// `HandCrossbow(<integer>)`
    HandCrossbow(i8),
    /// `HeavyCrossbow(<integer>)`
    HeavyCrossbow(i8),
    /// `Longbow(<integer>)`
    Longbow(i8),
    /// `Net(<integer>)`
    Net(i8),

}

impl Weapon {

    pub fn get_attack(&self) -> Attack {
        // Finnesse weapons use 'Best' attack bonus.
        // Reach weapons get an extra five feet of reach
        // Thrown weapons get a 'range' even if melee
        let (bonus,magic,reach,range,long_range,target) = match self {
            Weapon::UnarmedStrike(magic) => (AttackBonus::Strength,  Some(*magic),  Some(5),   None,       None,      "one target".to_owned()),
            Weapon::Club(magic) =>          (AttackBonus::Strength,  Some(*magic),  Some(5),   None,       None,      "one target".to_owned()),
            Weapon::Dagger(magic) =>        (AttackBonus::Best,      Some(*magic),  Some(5),   Some(20),   Some(60),  "one target".to_owned()),
            Weapon::Greatclub(magic) =>     (AttackBonus::Strength,  Some(*magic),  Some(5),   None,       None,      "one target".to_owned()),
            Weapon::Handaxe(magic) =>       (AttackBonus::Strength,  Some(*magic),  Some(5),   Some(20),   Some(60),  "one target".to_owned()),
            Weapon::Javelin(magic) =>       (AttackBonus::Strength,  Some(*magic),  Some(5),   Some(30),   Some(120), "one target".to_owned()),
            Weapon::LightHammer(magic) =>   (AttackBonus::Strength,  Some(*magic),  Some(5),   Some(20),   Some(60),  "one target".to_owned()),
            Weapon::Mace(magic) =>          (AttackBonus::Strength,  Some(*magic),  Some(5),   None,       None,      "one target".to_owned()),
            Weapon::Quarterstaff(magic) =>  (AttackBonus::Strength,  Some(*magic),  Some(5),   None,       None,      "one target".to_owned()),
            Weapon::Sickle(magic) =>        (AttackBonus::Strength,  Some(*magic),  Some(5),   None,       None,      "one target".to_owned()),
            Weapon::Spear(magic) =>         (AttackBonus::Strength,  Some(*magic),  Some(5),   Some(20),   Some(60),  "one target".to_owned()),
            
            Weapon::LightCrossbow(magic) => (AttackBonus::Dexterity, Some(*magic),  None,      Some(80),   Some(320), "one target".to_owned()),
            Weapon::Dart(magic) =>          (AttackBonus::Best,      Some(*magic),  None,      Some(20),   Some(60),  "one target".to_owned()),
            Weapon::Shortbow(magic) =>      (AttackBonus::Dexterity, Some(*magic),  None,      Some(80),   Some(320), "one target".to_owned()),
            Weapon::Sling(magic) =>         (AttackBonus::Dexterity, Some(*magic),  None,      Some(30),   Some(120), "one target".to_owned()),
            
            Weapon::Battleaxe(magic) =>     (AttackBonus::Strength,  Some(*magic),  Some(5),   None,       None,      "one target".to_owned()),
            Weapon::Flail(magic) =>         (AttackBonus::Strength,  Some(*magic),  Some(5),   None,       None,      "one target".to_owned()),
            Weapon::Glaive(magic) =>        (AttackBonus::Strength,  Some(*magic),  Some(10),  None,       None,      "one target".to_owned()),
            Weapon::Greataxe(magic) =>      (AttackBonus::Strength,  Some(*magic),  Some(5),   None,       None,      "one target".to_owned()),
            Weapon::Greatsword(magic) =>    (AttackBonus::Strength,  Some(*magic),  Some(5),   None,       None,      "one target".to_owned()),
            Weapon::Halberd(magic) =>       (AttackBonus::Strength,  Some(*magic),  Some(10),  None,       None,      "one target".to_owned()),
            Weapon::Lance(magic) =>         (AttackBonus::Strength,  Some(*magic),  Some(10),  None,       None,      "one target".to_owned()),
            Weapon::Longsword(magic) =>     (AttackBonus::Strength,  Some(*magic),  Some(5),   None,       None,      "one target".to_owned()),
            Weapon::Maul(magic) =>          (AttackBonus::Strength,  Some(*magic),  Some(5),   None,       None,      "one target".to_owned()),
            Weapon::Morningstar(magic) =>   (AttackBonus::Strength,  Some(*magic),  Some(5),   None,       None,      "one target".to_owned()),
            Weapon::Pike(magic) =>          (AttackBonus::Strength,  Some(*magic),  Some(10),  None,       None,      "one target".to_owned()),
            Weapon::Rapier(magic) =>        (AttackBonus::Best,      Some(*magic),  Some(5),   None,       None,      "one target".to_owned()),
            Weapon::Scimitar(magic) =>      (AttackBonus::Best,      Some(*magic),  Some(5),   None,       None,      "one target".to_owned()),
            Weapon::Shortsword(magic) =>    (AttackBonus::Best,      Some(*magic),  Some(5),   None,       None,      "one target".to_owned()),
            Weapon::Trident(magic) =>       (AttackBonus::Strength,  Some(*magic),  Some(5),   Some(20),   Some(60),  "one target".to_owned()),
            Weapon::WarPick(magic) =>       (AttackBonus::Strength,  Some(*magic),  Some(5),   None,       None,      "one target".to_owned()),
            Weapon::Warhammer(magic) =>     (AttackBonus::Strength,  Some(*magic),  Some(5),   None,       None,      "one target".to_owned()),
            Weapon::Whip(magic) =>          (AttackBonus::Best,      Some(*magic),  Some(10),  None,       None,      "one target".to_owned()),
            
            Weapon::Blowgun(magic) =>       (AttackBonus::Dexterity, Some(*magic),  None,      Some(25),   Some(100), "one target".to_owned()),
            Weapon::HandCrossbow(magic) =>  (AttackBonus::Dexterity, Some(*magic),  None,      Some(30),   Some(120), "one target".to_owned()),
            Weapon::HeavyCrossbow(magic) => (AttackBonus::Dexterity, Some(*magic),  None,      Some(100),  Some(400), "one target".to_owned()),
            Weapon::Longbow(magic) =>       (AttackBonus::Dexterity, Some(*magic),  None,      Some(150),  Some(600), "one target".to_owned()),
            Weapon::Net(magic) =>           (AttackBonus::Dexterity, Some(*magic),  None,      Some(5),    Some(15),  "one Large or smaller creature that is not formless".to_owned()),
        };

        Attack {
            type_: Some(AttackType::Weapon),
            bonus, 
            magic,
            reach,
            range,
            long_range,
            target,


        }

    }

    pub fn get_effect(&self, size: &CreatureSize) -> AttackEffect {
        // Versatile weapons should get an Or effect for the two hands
        // The net is a "special" weapon with a weird effects.
        // Damage Dice per size:
        // - Large: Double dice
        // - Huge: Triple dice
        // - Gargantuan: Quadruple Dice
        
        let size_factor = match size {
            CreatureSize::Large => 2,
            CreatureSize::Huge => 3,
            CreatureSize::Gargantuan => 4,
            _ => 1
        };

        macro_rules! dice {
            ($coefficient: literal, $die: ident, $magic: ident) => {
                DiceExpression::from_dice(Dice::new($coefficient * size_factor, &Die::$die),*$magic as isize)
            };
        }

        macro_rules! die {
            ($die: ident, $magic: ident) => {
                dice!(1,$die,$magic)
            };
        }

        macro_rules! fixed {
            ($magic: ident) => {
                (size_factor as i8 + *$magic).max(1) as u8 // if it is always at least 1, then this can't panic
            };
        }

        match self {
            Weapon::UnarmedStrike(magic) => AttackEffect::FixedDamage(fixed!(magic), AttackBonus::Strength, Damage::Bludgeoning),
            Weapon::Club(magic) =>          AttackEffect::Damage(die!(D4,magic), AttackBonus::Strength,    Damage::Bludgeoning),
            Weapon::Dagger(magic) =>        AttackEffect::Damage(die!(D4,magic), AttackBonus::Best,        Damage::Piercing),
            Weapon::Greatclub(magic) =>     AttackEffect::Damage(die!(D8,magic), AttackBonus::Strength,    Damage::Bludgeoning),
            Weapon::Handaxe(magic) =>       AttackEffect::Damage(die!(D6,magic), AttackBonus::Strength,    Damage::Slashing),
            Weapon::Javelin(magic) =>       AttackEffect::Damage(die!(D6,magic), AttackBonus::Strength,    Damage::Piercing),
            Weapon::LightHammer(magic) =>   AttackEffect::Damage(die!(D4,magic), AttackBonus::Strength,    Damage::Bludgeoning),
            Weapon::Mace(magic) =>          AttackEffect::Damage(die!(D6,magic), AttackBonus::Strength,    Damage::Bludgeoning),
            Weapon::Quarterstaff(magic) =>  AttackEffect::Or(die!(D6,magic), AttackBonus::Strength,    Damage::Bludgeoning,
                                                             die!(D8,magic), AttackBonus::Strength,    Damage::Bludgeoning,
                                                             "if used with two hands to make a melee attack".to_owned()),
            Weapon::Sickle(magic) =>        AttackEffect::Damage(die!(D4,magic), AttackBonus::Strength,    Damage::Slashing),
            Weapon::Spear(magic) =>         AttackEffect::Or(die!(D6,magic), AttackBonus::Strength,    Damage::Piercing,
                                                             die!(D8,magic), AttackBonus::Strength,    Damage::Piercing,
                                                             "if used with two hands to make a melee attack".to_owned()),
            Weapon::LightCrossbow(magic) => AttackEffect::Damage(die!(D8,magic), AttackBonus::Dexterity,   Damage::Piercing),
            Weapon::Dart(magic) =>          AttackEffect::Damage(die!(D4,magic), AttackBonus::Best,        Damage::Piercing),
            Weapon::Shortbow(magic) =>      AttackEffect::Damage(die!(D6,magic), AttackBonus::Dexterity,   Damage::Piercing),
            Weapon::Sling(magic) =>         AttackEffect::Damage(die!(D4,magic), AttackBonus::Dexterity,   Damage::Bludgeoning),
            Weapon::Battleaxe(magic) =>     AttackEffect::Or(die!(D8,magic), AttackBonus::Strength,    Damage::Slashing,
                                                             die!(D10,magic), AttackBonus::Strength,    Damage::Slashing,
                                                             "if used with two hands to make a melee attack".to_owned()),
            Weapon::Flail(magic) =>         AttackEffect::Damage(die!(D8,magic), AttackBonus::Strength,    Damage::Bludgeoning),
            Weapon::Glaive(magic) =>        AttackEffect::Damage(die!(D10,magic), AttackBonus::Strength,    Damage::Slashing),
            Weapon::Greataxe(magic) =>      AttackEffect::Damage(die!(D12,magic), AttackBonus::Strength,    Damage::Slashing),
            Weapon::Greatsword(magic) =>    AttackEffect::Damage(dice!(2,D6,magic), AttackBonus::Strength,    Damage::Slashing),
            Weapon::Halberd(magic) =>       AttackEffect::Damage(die!(D10,magic), AttackBonus::Strength,    Damage::Slashing),
            Weapon::Lance(magic) =>         AttackEffect::Damage(die!(D12,magic), AttackBonus::Strength,    Damage::Piercing),
            Weapon::Longsword(magic) =>     AttackEffect::Or(die!(D8,magic), AttackBonus::Strength,    Damage::Slashing,
                                                             die!(D10,magic), AttackBonus::Strength,    Damage::Slashing,
                                                             "if used with two hands to make a melee attack".to_owned()),
            Weapon::Maul(magic) =>          AttackEffect::Damage(dice!(2,D6,magic), AttackBonus::Strength,    Damage::Bludgeoning),
            Weapon::Morningstar(magic) =>   AttackEffect::Damage(die!(D8,magic), AttackBonus::Strength,    Damage::Piercing),
            Weapon::Pike(magic) =>          AttackEffect::Damage(die!(D10,magic), AttackBonus::Strength,    Damage::Piercing),
            Weapon::Rapier(magic) =>        AttackEffect::Damage(die!(D8,magic), AttackBonus::Best,    Damage::Piercing),
            Weapon::Scimitar(magic) =>      AttackEffect::Damage(die!(D6,magic), AttackBonus::Best,    Damage::Slashing),
            Weapon::Shortsword(magic) =>    AttackEffect::Damage(die!(D6,magic), AttackBonus::Best,    Damage::Piercing),
            Weapon::Trident(magic) =>       AttackEffect::Or(die!(D6,magic), AttackBonus::Strength, Damage::Piercing,
                                                             die!(D8,magic), AttackBonus::Strength, Damage::Piercing,
                                                             "if used with two hands to make a melee attack".to_owned()),
            Weapon::WarPick(magic) =>       AttackEffect::Damage(die!(D8,magic), AttackBonus::Strength, Damage::Piercing),
            Weapon::Warhammer(magic) =>     AttackEffect::Or(die!(D8,magic), AttackBonus::Strength, Damage::Bludgeoning,
                                                             die!(D10,magic), AttackBonus::Strength, Damage::Bludgeoning,
                                                             "if used with two hands to make a melee attack".to_owned()),
            Weapon::Whip(magic) =>          AttackEffect::Damage(die!(D4,magic), AttackBonus::Best,    Damage::Slashing),
            Weapon::Blowgun(magic) =>       AttackEffect::FixedDamage(fixed!(magic), AttackBonus::Default, Damage::Piercing),
            Weapon::HandCrossbow(magic) =>  AttackEffect::Damage(die!(D6,magic), AttackBonus::Default, Damage::Piercing),
            Weapon::HeavyCrossbow(magic) => AttackEffect::Damage(die!(D10,magic), AttackBonus::Default, Damage::Piercing),
            Weapon::Longbow(magic) =>       AttackEffect::Damage(die!(D8,magic), AttackBonus::Default, Damage::Piercing),
            Weapon::Net(magic) =>           AttackEffect::Special(format!("Creature is restrained until it is freed. The creature can use its action to make a DC ${{10 + {}}} Strength check, freeing itself or another creature within its reach on a success. Dealing ${{5 + {}}} slashing damage to the net (AC ${{10 + {}}}) also frees the creature without harmint it, ending the effect and destroying the net.",magic,magic,magic)),
        }

    }
}

impl std::fmt::Display for Weapon {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(),std::fmt::Error> {
        let (name,bonus) = match self {
            Weapon::UnarmedStrike(bonus) => ("Unarmed Strike",bonus),
            Weapon::Club(bonus) => ("Club",bonus),
            Weapon::Dagger(bonus) => ("Dagger",bonus),
            Weapon::Greatclub(bonus) => ("Greatclub",bonus),
            Weapon::Handaxe(bonus) => ("Handaxe",bonus),
            Weapon::Javelin(bonus) => ("Javelin",bonus),
            Weapon::LightHammer(bonus) => ("LightHammer",bonus),
            Weapon::Mace(bonus) => ("Mace",bonus),
            Weapon::Quarterstaff(bonus) => ("Quarterstaff",bonus),
            Weapon::Sickle(bonus) => ("Sickle",bonus),
            Weapon::Spear(bonus) => ("Spear",bonus),
            Weapon::LightCrossbow(bonus) => ("LightCrossbow",bonus),
            Weapon::Dart(bonus) => ("Dart",bonus),
            Weapon::Shortbow(bonus) => ("Shortbow",bonus),
            Weapon::Sling(bonus) => ("Sling",bonus),
            Weapon::Battleaxe(bonus) => ("Battleaxe",bonus),
            Weapon::Flail(bonus) => ("Flail",bonus),
            Weapon::Glaive(bonus) => ("Glaive",bonus),
            Weapon::Greataxe(bonus) => ("Greataxe",bonus),
            Weapon::Greatsword(bonus) => ("Greatsword",bonus),
            Weapon::Halberd(bonus) => ("Halberd",bonus),
            Weapon::Lance(bonus) => ("Lance",bonus),
            Weapon::Longsword(bonus) => ("Longsword",bonus),
            Weapon::Maul(bonus) => ("Maul",bonus),
            Weapon::Morningstar(bonus) => ("Morningstar",bonus),
            Weapon::Pike(bonus) => ("Pike",bonus),
            Weapon::Rapier(bonus) => ("Rapier",bonus),
            Weapon::Scimitar(bonus) => ("Scimitar",bonus),
            Weapon::Shortsword(bonus) => ("Shortsword",bonus),
            Weapon::Trident(bonus) => ("Trident",bonus),
            Weapon::WarPick(bonus) => ("WarPick",bonus),
            Weapon::Warhammer(bonus) => ("Warhammer",bonus),
            Weapon::Whip(bonus) => ("Whip",bonus),
            Weapon::Blowgun(bonus) => ("Blowgun",bonus),
            Weapon::HandCrossbow(bonus) => ("Hand Crossbow",bonus),
            Weapon::HeavyCrossbow(bonus) => ("Heavy Crossbow",bonus),
            Weapon::Longbow(bonus) => ("Longbow",bonus),
            Weapon::Net(bonus) => ("Net",bonus),
        };
        if *bonus != 0 {
            write!(f,"{} {:+}",name,bonus)
        } else {
            write!(f,"{}",name)
        }
    }

}

#[derive(Debug)]
pub struct ParseWeaponError;


impl std::str::FromStr for Weapon {
    type Err = ParseWeaponError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name,bonus) = if let Some((name,bonus)) = s.split_once('+') {
            (name,bonus.parse().or_else(|_| Err(ParseWeaponError))?)
        } else {
            (s,0)
        };
        let weapon_fn = match name.trim().to_lowercase().as_str() {
            "unarmed strike" => Weapon::UnarmedStrike,
            "club" => Weapon::Club,
            "dagger" => Weapon::Dagger,
            "greatclub" => Weapon::Greatclub,
            "handaxe" => Weapon::Handaxe,
            "javelin" => Weapon::Javelin,
            "light hammer" => Weapon::LightHammer,
            "mace" => Weapon::Mace,
            "quarterstaff" => Weapon::Quarterstaff,
            "sickle" => Weapon::Sickle,
            "spear" => Weapon::Spear,
            "light crossbow" => Weapon::LightCrossbow,
            "dart" => Weapon::Dart,
            "shortbow" => Weapon::Shortbow,
            "sling" => Weapon::Sling,
            "battleaxe" => Weapon::Battleaxe,
            "flail" => Weapon::Flail,
            "glaive" => Weapon::Glaive,
            "greataxe" => Weapon::Greataxe,
            "greatsword" => Weapon::Greatsword,
            "halberd" => Weapon::Halberd,
            "lance" => Weapon::Lance,
            "longsword" => Weapon::Longsword,
            "maul" => Weapon::Maul,
            "morningstar" => Weapon::Morningstar,
            "pike" => Weapon::Pike,
            "rapier" => Weapon::Rapier,
            "scimitar" => Weapon::Scimitar,
            "shortsword" => Weapon::Shortsword,
            "trident" => Weapon::Trident,
            "war pick" => Weapon::WarPick,
            "warhammer" => Weapon::Warhammer,
            "whip" => Weapon::Whip,
            "blowgun" => Weapon::Blowgun,
            "hand crossbow" => Weapon::HandCrossbow,
            "heavy crossbow" => Weapon::HeavyCrossbow,
            "longbow" => Weapon::Longbow,
            "net" => Weapon::Net,
            _ => Err(ParseWeaponError)?
        };
        Ok(weapon_fn(bonus))
    }    
}
