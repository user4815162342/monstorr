/*
 * Copyright © 2022 Neil M. Sheldon
 * 
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use serde::Deserialize;
use serde::Serialize;

use crate::dice_expression::DiceExpression;
use crate::attacks::Attack;
use crate::attacks::AttackEffect;
use crate::attacks::CompoundAttackEffect;
use crate::attacks::Weapon;


#[derive(Clone,PartialEq,Debug)]
#[derive(Serialize,Deserialize)]
/**
Specifies usage limits for actions, reactions and other features.
*/
pub enum UsageLimit {
    /**
    `Recharge(<integer>)`

    Specifies that the action recharges on a random roll. This is phrased in the name of the action as "Recharge <number>-6".
    */
    Recharge(u8), // the roll equal or above that the monster must roll to recharge a feature (i.e. Recharge 5-6 would be 5)
    /**
    `PerDay(<integer>)`

    The action can be used a limited number of times per day. This is phrased in the name as "<number>/Day"
    */
    PerDay(u8), // the number of times per day that the monster can use the feature
    /**
    `PerTurn(<integer>)`

    The action can be used a limited number of times per turn. This is phrased in the name as "<number>/Turn"
    */
    PerTurn(u8), // the number of times per turn that the monster can use the feature
    /**
    `RechargeAfterRest`
    
    The action can be used once and recharges only after a short or long rest.
    */
    RechargeAfterRest, // the feature recharges after a short or long rest
    /**
    `AlternateFormOnly(<string>)`

    The action can only be used while the creature is in an alternate form. The string describes this alternate form.
    */
    AlternateFormOnly(String), // The feature is only available while the monster is in an alternate form
}

impl std::fmt::Display for UsageLimit {

    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(),std::fmt::Error> {
        match self {
            UsageLimit::Recharge(6) => write!(f,"Recharge 6"),
            UsageLimit::Recharge(roll) => write!(f,"Recharge {}-6",roll),
            UsageLimit::PerDay(count) => write!(f,"{}/Day",count),
            UsageLimit::PerTurn(count) => write!(f,"{}/Turn",count),
            UsageLimit::RechargeAfterRest => f.write_str("Recharges after a Short or Long Rest"),
            UsageLimit::AlternateFormOnly(form) => write!(f,"{} Form Only",form)
        }
    }

}

// The built-in actions come with free descriptions, so you don't have to rewrite what is given for monsters which use these in the SRD.
// Some of these automaticaly effect CR as well.
#[derive(PartialEq,Debug,Clone)]
#[derive(Serialize,Deserialize)]
/**

Many of the variants in this value are attempts to mimic common actions found in the SRD, as well as actions which the Dungeon Master's Guide says may have an effect on the challenge rating. However, since the SRD does not use consistent phrasing for all actions, you may not get the output you expect. If you want to use an action with one of the specified names, use that, even if you have to override the description to fix it later. That way, you'll get the benefits of challenge rating calculation when that is available.

*/
pub enum Action {
    
    /**
    `ChangeShape(<string>)`

    The specified string is a description of the form the creature can turn into, such as "a humanoid or beast that has a challenge rating no higher than its own"
    */
    ChangeShape(String), // Description of the form the creature can turn into
    
    /**
    `Charm(<integer>,<integer>)`

    The first number is the range of the effect, and the second is the Wisdom save DC.

    */
    Charm(u8,u8), // distance, save dc (Wisdom)
    
    /**
    `Enlarge`
    */
    Enlarge,
    
    /**
    `Etherealness`
    */
    Etherealness,
    
    /**
    `FrightfulPresence(<integer>,<integer>,<boolean>)`

    The first integer is the range of the effect, the second is the Wisdom save DC. The last boolean indicates whether re-saves are done at a disadvantage, as with the Tarrasque.

    */
    FrightfulPresence(u8,u8,bool), // distance, DC (Wisdom), whether re-saves are done at disadvantage (Tarrasque-style)
    
    /**
    `HorrifyingVisage(<integer>,<integer>)`

    The first integer is the range of the effect, the second is the Wisdom save DC.
    */
    HorrifyingVisage(u8,u8), // distance, DC (Wisdom)
    
    /**
    `IllusoryAppearance(<integer>)`

    The integer is the investigation (Intelligence) check DC to discern the true appearance.
    */
    IllusoryAppearance(u8), // DC to discern (Investigation)
    
    /**
    `Leadership`
    */
    Leadership,
    
    /**
    `Possession(<integer>)`

    The integer is the Charisma save DC.
    */
    Possession(u8), // Save DC (Charisma)
    
    /**
    `ReadThoughts(<integer>)`

    The integer is the range of the effect.
    */
    ReadThoughts(u8), // distance
    
    /**
    `Reel`
    */
    Reel,
    
    /**
    `Swallow(<dice-expression-string>,<integer>,<integer>,<integer>)`

    The first argument is the acid damage inflected while swallowed. The second is the damage required to cause the creature to vomit. The third is the Constitution save DC the creature must roll. The last is the distance required to escape.
    */
    Swallow(DiceExpression,u8,u8,u8), // damage (acid) while swallowed, damage required to vomit, creature DC to save (Constitution), distance to escape corpse
    
    /**
    `Teleport(<integer>)`
    `Teleport(<integer>,option(<string>))`

    The first argument is the distance the creature can teleport. The second is the name of an optional action which can be taken prior to teleporting.
    */
    Teleport(u8,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        Option<String>), // distance of teleport, an optional attack which can be made prior to teleport

    /**
    `Attack(<string>,<Attack>,<AttackEffect>)`
    `Attack(<string>,<Attack>,<AttackEffect>,option(<CompoundAttackEffect>))`

    This variant adds a custom attack to the creature. The first argument is the name of the attack, the other arguments describe the attack, and are used to generate the description. See [`crate::attacks::Attack`], [`crate::attacks::AttackEffect`], and [`crate::attacks::CompoundAttackEffect`].
    */
    Attack(String,Attack,AttackEffect,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        Option<CompoundAttackEffect>),

    /**
    `Action(<string>,<string>,option(<AttackEffect>))`
    `Action(<string>,<string>,option(<AttackEffect>),option(<CompoundAttackEffect>))`

    This variant adds a custom action that isn't an attack, or at least doesn't require an attack roll to hit. The first argument is the name, the second is the description, which will be interpolated. The remaining arguments describe the effects of the action, and are used to calculate damage per round. These effects aren't used to calculate the final description.
    */
    Action(String,String,
        // I can't skip serializing this one, as the deserializing would get confused if we had None,Some for these last two
        Option<AttackEffect>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        Option<CompoundAttackEffect>), // name, description, damage effect if necessary.

    // Breath weapon details vary so much between creatures, with each species of dragon (and other creatures) having their 
    // own unique powers, that they are just a custom action. Even some of them are named different (and how do I handle the
    // dual weapons for metallic dragons?) However, they may be important to future Challenge Rating calculation.

    /**
    `BreathWeapon(<string>,<string>,option(<AttackEffect>))`
    `BreathWeapon(<string>,<string>,option(<AttackEffect>),option(<CompoundAttackEffect>))`

    Follows the same signatures as the `Action` variant. Breath weapon actions vary so much between creatures, with each species having their own unique powers, that these are just custom actions. But, their existence does change the challenge rating.
    */
    BreathWeapon(String,String,
        // I can't skip serializing this one, as the deserializing would get confused if we had None,Some for these last two
        Option<AttackEffect>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        Option<CompoundAttackEffect>), // name, description, damage effect if necessary.
    

}

impl Action {


    pub fn get_name(&self) -> String {
        match self {
            Action::ChangeShape(..) => "Change Shape".to_owned(),
            Action::Charm(..) => "Charm".to_owned(),
            Action::Enlarge => "Enlarge".to_owned(),
            Action::Etherealness => "Etherealness".to_owned(),
            Action::FrightfulPresence(..) => "Frightful Presence".to_owned(),
            Action::HorrifyingVisage(..) => "Horrifying Visage".to_owned(),
            Action::IllusoryAppearance(..) => "Illusory Appearance".to_owned(),
            Action::Leadership => "Leadership".to_owned(),
            Action::Possession(..) => "Possession".to_owned(),
            Action::ReadThoughts(..) => "Read Thoughts".to_owned(),
            Action::Reel => "Reel".to_owned(),
            Action::Swallow(..) => "Swallow".to_owned(),
            Action::Teleport(..) => "Teleport".to_owned(),
            Action::Attack(name,..) => name.clone(),
            Action::Action(name,..) => name.clone(),
            Action::BreathWeapon(name,..) => name.clone(),
        }
    }



    pub fn get_description(&self) -> String {
        match self {
          Action::ChangeShape(form) => 
                format!("${{Subj}} magically polymorphs into {}, or back into ${{posspro}} true form. It reverts to ${{posspro}} true form if ${{subjpro}} dies. Any equipment ${{subjpro}} is wearing or carrying is absorbed or borne by the new form (${{poss}} choice). In a new form, ${{subj}} retains ${{posspro}} alignment, hit points, Hit Dice, ability to speak, proficiencies, Legendary Resistance, lair actions, and Intelligence, Wisdom, and Charisma scores, as well as this action. Its statistics and capabilities are otherwise replaced by those of the new form, except any class features or legendary actions of that form.",form), 
            Action::Charm(distance,save_dc) => 
                format!("One humanoid ${{subj}} can see within {} feet of ${{subjpro}} must succeed on a DC {} Wisdom saving throw or be magically charmed for 1 day. The charmed target obeys ${{poss}} verbal or telepathic commands. If the target suffers any harm or receives a suicidal command, it can repeat the saving throw, ending the effect on a success. If the target successfully saves against the effect, or if the effect on it ends, the target is immune to this ${{name}}'s Charm for the next 24 hours.${{par}}${{Subj}} can have only one target charmed at a time. If ${{subjpro}} charms another, the effect on the previous target ends.",distance,save_dc), 
            Action::Enlarge => 
                format!("For 1 minute, ${{subj}} magically increases in size, along with anything ${{subjpro}} is wearing or carrying. While enlarged, ${{subj}} is Large, doubles ${{posspro}} damage dice on Strength-based weapon attacks (included in the attacks), and makes Strength checks and Strength saving throws with advantage. If ${{subj}} lacks the room to become Large, ${{subjpro}} attains the maximum size possible in the space available."),
            Action::Etherealness => 
                format!("${{Subj}} enters the Ethereal Plane from the Material Plane, or vice versa. It is visible on the Material Plane while ${{subjpro}} is in the Border Ethereal, and vice versa, yet ${{subjpro}} can't affect or be affected by anything on the other plane."),
            Action::FrightfulPresence(distance,save_dc,resave_disadvantage) => 
                format!("Each creature of ${{poss}} choice that is within {} feet of ${{subj}} and aware of ${{obpro}} must succeed on a DC {} Wisdom saving throw or become frightened for 1 minute. A creature can repeat the saving throw at the end of each of its turns, {}ending the effect on ${{refpro}} on a success. If a creature's saving throw is successful or the effect ends for it, the creature is immune to ${{poss}} Frightful Presence for the next 24 hours.",distance,save_dc,if *resave_disadvantage {"with disadvantage if ${{subj}} is within line of sight, "} else {""}),
            Action::HorrifyingVisage(distance,save_dc) => 
                format!("Each non-undead creature within {} ft. of ${{subj}} that can see ${{subjpro}} must succeed on a DC {} Wisdom saving throw or be frightened for 1 minute. If the save fails by 5 or more, the target also ages 1d4 x 10 years. A frightened target can repeat the saving throw at the end of each of its turns, ending the frightened condition on ${{refpro}} on a success. If a target's saving throw is successful or the effect ends for it, the target is immune to this ${{name}}'s Horrifying Visage for the next 24 hours. The aging effect can be reversed with a greater restoration spell, but only within 24 hours of it occurring.",distance,save_dc), 
            Action::IllusoryAppearance(discern_dc) => 
                format!("${{Subj}} covers ${{refpro}} and anything ${{subjpro}} is wearing or carrying with a magical illusion that makes ${{obpro}} look like another creature of ${{posspro}} general size and humanoid shape. The illusion ends if ${{subj}} takes a bonus action to end it or if ${{subjpro}} dies.${{par}}The changes wrought by this effect fail to hold up to physical inspection. Otherwise, a creature must take an action to visually inspect the illusion and succeed on a DC {} Intelligence (Investigation) check to discern that ${{subj}} is disguised.",discern_dc), 
            Action::Leadership => 
                format!("For 1 minute, ${{subj}} can utter a special command or warning whenever a nonhostile creature that ${{subjpro}} can see within 30 ft. of it makes an attack roll or a saving throw. The creature can add a d4 to its roll provided it can hear and understand ${{subj}}. A creature can benefit from only one Leadership die at a time. This effect ends if the knight is incapacitated."),
            Action::Possession(save_dc) => 
                format!("One humanoid that ${{subj}} can see within 5 ft. of ${{obpro}} must succeed on a DC {} Charisma saving throw or be possessed by ${{subj}}; ${{subj}} then disappears, and the target is incapacitated and loses control of its body. ${{Subj}} now controls the body but doesn't deprive the target of awareness. ${{Subj}} can't be targeted by any attack, spell, or other effect, except ones that turn undead, and it retains its alignment, Intelligence, Wisdom, Charisma, and immunity to being charmed and frightened. It otherwise uses the possessed target's statistics, but doesn't gain access to the target's knowledge, class features, or proficiencies.${{par}}The possession lasts until the body drops to 0 hit points, the ghost ends it as a bonus action, or ${{subj}} is turned or forced out by an effect like the dispel evil and good spell. When the possession ends, ${{subj}} reappears in an unoccupied space within 5 ft. of the body. The target is immune to this ghost's Possession for 24 hours after succeeding on the saving throw or after the possession ends.",save_dc), 
            Action::ReadThoughts(distance) => 
                format!("${{Subj}} magically reads the surface thoughts of one creature within {} ft. of ${{obpro}}. The effect can penetrate barriers, but 3 ft. of wood or dirt, 2 ft. of stone, 2 inches of metal, or a thin sheet of lead blocks it. While the target is in range, ${{subj}} can continue reading its thoughts, as long as ${{poss}} concentration isn't broken (as if concentrating on a spell). While reading the target's mind, ${{subj}} has advantage on Wisdom (Insight) and Charisma (Deception, Intimidation, and Persuasion) checks against the target.",distance), 
            Action::Reel => 
                format!("${{Subj}} pulls each creature grappled by ${{obpro}} up to 25 ft. straight toward ${{obpro}}."),
            Action::Swallow(damage,vomit_damage,vomit_save_dc,escape_distance) => 
                format!("${{Subj}} makes one bite attack against a Medium or smaller creature ${{subjpro}} is grappling. If the attack hits, that creature takes the bite's damage and is swallowed, and the grapple ends. While swallowed, the creature is blinded and restrained, it has total cover against attacks and other effects outside ${{subj}}, and it takes {} acid damage at the start of each of ${{poss}} turns.${{par}}If ${{subj}} takes {} damage or more on a single turn from a creature inside ${{objpro}}, ${{subj}} must succeed on a DC {} Constitution saving throw at the end of that turn or regurgitate all swallowed creatures, which fall prone in a space within 10 feet oft he ${{name}}. If ${{subj}} dies, a swallowed creature is no longer restrained by ${{objpro}} and can escape from the corpse using {} feet of movement, exiting prone.",damage,vomit_damage,vomit_save_dc,escape_distance), 
            Action::Teleport(distance,opt_attack) => 
                format!("${{Subj}} magically teleports, along with any equipment ${{subjpro}} is wearing or carrying, up to {} ft. to an unoccupied space ${{subjpro}} can see.{}",distance,if let Some(attack) = opt_attack {
                     format!(" Before or after teleporting, ${{subj}} can make one {} attack.",attack)
                 } else { "".to_owned() }),
            Action::Attack(_,attack,effect,compound_effect) => 
                attack.get_description(Some(effect),compound_effect),
            // This is for custom actions that aren't attacks. Although some might still cause damage, these actions don't require an attack roll.
            Action::Action(_,description,..) => description.clone(),
            Action::BreathWeapon(_,description,..) => description.clone()
        }
    }

    pub fn get_attack(&self) -> Option<Attack> {
        match self {
            Action::ChangeShape(..) => None,
            Action::Charm(..) => None,
            Action::Enlarge => None,
            Action::Etherealness => None,
            Action::FrightfulPresence(..) => None,
            Action::HorrifyingVisage(..) => None,
            Action::IllusoryAppearance(..) => None,
            Action::Leadership => None,
            Action::Possession(..) => None,
            Action::ReadThoughts(..) => None,
            Action::Reel => None,
            Action::Swallow(..) => None,
            Action::Teleport(..) => None,
            Action::Attack(_,attack,..) => 
                 Some(attack.clone()),
            // This is for custom actions that aren't attacks. Although some might still cause damage, these actions don't require an attack roll.
            Action::Action(..) => None,
            Action::BreathWeapon(..) => None,
        }        
    }

    pub fn get_effect(&self) -> Option<AttackEffect> {
        match self {
            Action::ChangeShape(..) => None,
            Action::Charm(..) => None,
            Action::Enlarge => None,
            Action::Etherealness => None,
            Action::FrightfulPresence(..) => None,
            Action::HorrifyingVisage(..) => None,
            Action::IllusoryAppearance(..) => None,
            Action::Leadership => None,
            Action::Possession(..) => None,
            Action::ReadThoughts(..) => None,
            Action::Reel => None,
            Action::Swallow(..) => None,
            Action::Teleport(..) => None,
            Action::Attack(_,_,effect,_) => 
                 Some(effect.clone()),
            // This is for custom actions that aren't attacks. Although some might still cause damage, these actions don't require an attack roll.
            Action::Action(_,_,effect,_) => effect.clone(),
            Action::BreathWeapon(_,_,effect,_) => effect.clone(),
        }
    }

    pub fn get_compound_effect(&self) -> Option<CompoundAttackEffect> {
        match self {
            Action::ChangeShape(..) => None,
            Action::Charm(..) => None,
            Action::Enlarge => None,
            Action::Etherealness => None,
            Action::FrightfulPresence(..) => None,
            Action::HorrifyingVisage(..) => None,
            Action::IllusoryAppearance(..) => None,
            Action::Leadership => None,
            Action::Possession(..) => None,
            Action::ReadThoughts(..) => None,
            Action::Reel => None,
            Action::Swallow(..) => None,
            Action::Teleport(..) => None,
            Action::Attack(_,_,_,compound_effect) => 
                 compound_effect.clone(),
            // This is for custom actions that aren't attacks. Although some might still cause damage, these actions don't require an attack roll.
            Action::Action(_,_,_,compound_effect) => compound_effect.clone(),
            Action::BreathWeapon(_,_,_,compound_effect) => compound_effect.clone()
        }        
    }




}




#[derive(PartialEq,Debug,Clone)]
#[derive(Serialize,Deserialize)]
/**
Used to add legendary actions to the descriptions.
*/
pub enum LegendaryAction {
    // an action that's defined elsewhere
    /**
    `UseAction(<integer>,<string>,<string>,<string>)`

    This legendary action allows the creature to use an action that is already described. 

    The first argument is the legendary action cost per round, the second and third are the name and description, which will be interpolated. The last is the name of the action from which damage-per-round data is taken.
    */
    UseAction(u8,String,String,String), // costs per round, name of legendary action, description of legendary action, name of an action to use
    // use a weapon
    /**
    `UseWeapon(<integer>,<string>,<string>,<Weapon>)`

    This legendary action allows the creature to use a weapon attack that is already described. 
    
    The first argument is the legendary action cost per round, the second and third are the name and description, which will be interpolated. The last is the weapon from which damage-per-round data is taken.
    */
    UseWeapon(u8,String,String,Weapon),// costs per round, name of legendary action, description of legendary action, weapon to use
    // An action which isn't defined elsewhere.
    /**
    `LegendaryAction(<integer>,<Action>)`

    A custom legendary action not taken from its normal list.

    The first action is the legendary cost per round, the second describes the action which is taken. See [`Action`].
    */
    LegendaryAction(u8,Action), // costs per round, name, description, damage effect if necessary.

}

impl LegendaryAction {

    pub fn get_main_description(amount: &u8) -> String {
        format!("${{Subj}} can take {} legendary actions, choosing from the options below. Only one legendary action option can be used at a time and only at the end of another creature's turn. ${{Subj}} regains spent legendary actions at the start of ${{posspro}} turn.",amount)
    }

}
