/*
 * Copyright © 2022 Neil M. Sheldon
 * 
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use serde::Deserialize;
use serde::Serialize;

use crate::utils::AndJoin;
use crate::dice_expression::DiceExpression;
use crate::stats::Damage;
use crate::attacks::AttackEffect;
use crate::attacks::CompoundAttackEffect;

#[derive(PartialEq,Debug,Clone)]
#[derive(Serialize,Deserialize)]
/**

Many of the variants in this value are attempts to mimic common features found in the SRD, as well as actions which the Dungeon Master's Guide says may have an effect on the challenge rating. Other features make automatic changes to the creature at the end of creation. However, since the SRD does not use consistent phrasing for all features, you may not get the output you expect. If you want to use a feature with one of the specified names, use that, even if you have to override the description to fix it later. That way, you'll get all of the benefits of the feature.

Features which make changes to the creature are done after the creature is mostly finished. Therefore it is not necessary to place the feature after other required features.

*/
pub enum Feature {

    
    /**
    `Aggressive`
    */
    Aggressive,
    
    /**
    `Ambusher`
    */
    Ambusher,
    
    /**
    `Amorphous`
    */
    Amorphous,
    
    /**
    `Amphibious`
    */
    Amphibious,
    
    /**
    `AngelicWeapons(<dice-expression-string>,[<string>..])`

    The first argument is the additional damage added to weapon attacks. The second is the list of actions this is automatically applied to.

    This feature will automatically change the specified attacks to add the specified damage. It will cause an error if the actions do not exist, or if they are in a configuration it can not deal with.
    */
    AngelicWeapons(DiceExpression,Vec<String>), // additional damage on weapon attacks, actions to apply it to
    
    /**
    `AntimagicSusceptibility`
    */
    AntimagicSusceptibility,
    
    /**
    `BlindSenses`
    */
    BlindSenses,
    
    /**
    `BloodFrenzy`
    */
    BloodFrenzy,
    
    /**
    `Brute([<string>...])`

    The argument is the list of melee attacks this feature is applied to.

    This feature automatically makes changes to the specified actions. An error will occur if the actions do not exist, are not melee actions, or are in a configuration it does not know how to deal with.
    */
    Brute(Vec<String>), // list of melee attacks this applies to.
    
    /**
    `Charge(<integer>,<string>,<dice-expression-string>,<Damage>)`

    The first argument is the distance of the charge, the second is the type of attack, the third and fourth indicate the extra damage the charge can cause.
    */
    Charge(u8,String,DiceExpression,Damage), // distance, attack type, extra damage dice, damage type
    
    /**
    `DamageTransfer`
    */
    DamageTransfer,
    
    /**
    `DeathBurst(<string>,<AttackEffect>)`
    `DeathBurst(<string>,<AttackEffect>,option(<CompoundAttackEffect>))`

    The first is the description of the explosion, the rest is the effect of the explosion.
    */
    DeathBurst(String,AttackEffect,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        Option<CompoundAttackEffect>), // explode description, range,  effect of explosion
    
    /**
    `DevilSight`
    */
    DevilSight,
    
    /**
    `Echolocation`
    */
    Echolocation,
    
    /**
    `FalseAppearance(<string>)`

    The argument is a description of the false appearance form.
    */
    FalseAppearance(String), // description
    
    /**
    `FeyAncestry`
    */
    FeyAncestry,
    
    /**
    `Flyby`
    */
    Flyby,
    
    /**
    `Grappler`
    */
    Grappler,
    
    /**
    `HoldBreath(<integer>,<boolean>)`

    The first is the duration, the second is whether this is reversed for aquatic creatures (i.e. the creature must hold its breath out of water)
    */
    HoldBreath(u8,bool), // duration, reversed ("out of water")
    
    /**
    `Illumination(<integer>)`

    The integer is the radius of the illumination.
    */
    Illumination(u8), // radius
    
    /**
    `ImmutableForm`
    */
    ImmutableForm,
    
    /**
    `IncorporealMovement(<dice-expression-string>)`

    The argument specifies the force damage roll the creature takes if it ends its turn inside an object.
    */
    IncorporealMovement(DiceExpression), // damage (force) if it ends its turn inside an object
    
    /**
    `Inscrutable`
    */
    Inscrutable,
    
    /**
    `Invisibility`
    */
    Invisibility,
    
    /**
    `KeenSenses(<boolean>,<boolean>,<boolean>)`

    The arguments specify which senses this applies. The first specifies sight, the second hearing, and the third smell.
    */
    KeenSenses(bool,bool,bool), // sight, hearing, smell
    
    /**
    `LabyrinthineRecall`
    */
    LabyrinthineRecall,
    
    /**
    `LegendaryResistance`
    */
    LegendaryResistance,
    
    /**
    `LightSensitivity`
    */
    LightSensitivity,
    
    /**
    `MagicResistance`
    */
    MagicResistance,
    
    /**
    `MagicWeapons`
    */
    MagicWeapons,
    
    /**
    `MartialAdvantage(<dice-expression-string>)`

    The argument specifies the extra damage dice the advantage gives them.
    */
    MartialAdvantage(DiceExpression), // extra damage dice
    
    /**
    `Mimicry(<string>,<integer>)`

    The first argument describes the mimicked speech, the second is the DC for an insight check.
    */
    Mimicry(String,u8), // description, Insight DC
    
    /**
    `NimbleEscape`
    */
    NimbleEscape,
    
    /**
    `PackTactics`
    */
    PackTactics,
    
    /**
    `Pounce(<integer>,<integer>)`

    The first argument is the distance of the pounce, the second is the Strength save DC.
    */
    Pounce(u8,u8), // distance, save DC (Strength)
    
    /**
    `Rampage`
    */
    Rampage,
    
    /**
    `Reactive`
    */
    Reactive,
    
    /**
    `Reckless`
    */
    Reckless,
    
    /**
    `Regeneration(<integer>,[<Damage>...])`

    The first argument is the number of hit points regenerated, the second is the list of damage this doesn't apply to. 
    */
    Regeneration(u8,Vec<Damage>), // number of hp, damage this doesn't apply to (might be based on vulnerabilities, in which case I don't need that)
    
    /**
    `Rejuvenation(<string>)`

    The argument is the description of the feature, as every one is different.
    */
    Rejuvenation(String), // needs a description, every rejuvenation is different
    
    /**
    `Relentless(<integer>)`

    The argument specifies the damage limit.
    */
    Relentless(u8), // damage limit
    
    /**
    `ShadowStealth`
    */
    ShadowStealth,
    
    /**
    `Shapechanger(<string>)`

    The argument describes the shapes available.
    */
    Shapechanger(String),
    
    /**
    `SiegeMonster`
    */
    SiegeMonster,
    
    /**
    `SpiderClimb`
    */
    SpiderClimb,
    
    /**
    `StandingLeap(<integer>)`

    The argument specifies the distance of the leap.
    */
    StandingLeap(u8), // distance
    
    /**
    `Steadfast`
    */
    Steadfast,
    
    /**
    `Stench(<integer>,<integer>)`

    The first integer specifies the range, the second the Constitution save DC.
    */
    Stench(u8,u8), // range, save DC (Constitution)
    
    /**
    `SunlightSensitivity`
    */
    SunlightSensitivity,
    
    /**
    `SureFooted`
    */
    SureFooted,
    
    /**
    `SurpriseAttack(<dice-expression-string>)`

    The argument specifies the extra damage incurred.
    */
    SurpriseAttack(DiceExpression), // extra damage
    
    /**
    `Swarm`
    */
    Swarm,
    
    /**
    `Tunneler(<integer>)`

    The argument specifies the width of the tunnel.
    */
    Tunneler(u8), // width of tunnel
    
    /**
    `TurnResistance`
    */
    TurnResistance,
    
    /**
    `TwoHeads`
    */
    TwoHeads,
    
    /**
    `UndeadFortitude`
    */
    UndeadFortitude,
    
    /**
    `WebSense`
    */
    WebSense,
    
    /**
    `WebWalker`
    */
    WebWalker,

    /**
    `Feature(<string>,<string>)`

    Adds a custom feature to the creature, with the specified name and description. These will be interpolated.
    */
    Feature(String,String), // name, description
}

impl Feature {


    pub fn get_name(&self) -> String {
        match self {
            Feature::Aggressive => "Aggressive".to_owned(),
            Feature::Ambusher => "Ambusher".to_owned(),
            Feature::Amorphous => "Amorphous".to_owned(),
            Feature::Amphibious => "Amphibious".to_owned(),
            Feature::AngelicWeapons(..) => "Angelic Weapons".to_owned(),
            Feature::AntimagicSusceptibility => "Antimagic Susceptibility".to_owned(),
            Feature::BlindSenses => "Blind Senses".to_owned(),
            Feature::BloodFrenzy => "Blood Frenzy".to_owned(),
            Feature::Brute(..) => "Brute".to_owned(),
            Feature::Charge(..) => "Charge".to_owned(),
            Feature::DamageTransfer => "Damage Transfer".to_owned(),
            Feature::DeathBurst(..) => "Death Burst".to_owned(),
            Feature::DevilSight => "Devil Sight".to_owned(),
            Feature::Echolocation => "Echolocation".to_owned(),
            Feature::FalseAppearance(..) => "False Appearance".to_owned(),
            Feature::FeyAncestry => "Fey Ancestry".to_owned(),
            Feature::Flyby => "Flyby".to_owned(),
            Feature::Grappler => "Grappler".to_owned(),
            Feature::HoldBreath(..) => "Hold Breath".to_owned(),
            Feature::Illumination(_) => "Illumination".to_owned(),
            Feature::ImmutableForm => "Immutable Form".to_owned(),
            Feature::IncorporealMovement(_) => "Incorporeal Movement".to_owned(),
            Feature::Inscrutable => "Inscrutable".to_owned(),
            Feature::Invisibility => "Invisibility".to_owned(),
            Feature::KeenSenses(..) => "Keen Senses".to_owned(),
            Feature::LabyrinthineRecall => "Labyrinthine Recall".to_owned(),
            Feature::LegendaryResistance => "Legendary Resistance".to_owned(),
            Feature::LightSensitivity => "Light Sensitivity".to_owned(),
            Feature::MagicResistance => "Magic Resistance".to_owned(),
            Feature::MagicWeapons => "Magic Weapons".to_owned(),
            Feature::MartialAdvantage(_) => "Martial Advantage".to_owned(),
            Feature::Mimicry(..) => "Mimicry".to_owned(),
            Feature::NimbleEscape => "Nimble Escape".to_owned(),
            Feature::PackTactics => "Pack Tactics".to_owned(),
            Feature::Pounce(..) => "Pounce".to_owned(),
            Feature::Rampage => "Rampage".to_owned(),
            Feature::Reactive => "Reactive".to_owned(),
            Feature::Reckless => "Reckless".to_owned(),
            Feature::Regeneration(..) => "Regeneration".to_owned(),
            Feature::Rejuvenation(_) => "Rejuvenation".to_owned(),
            Feature::Relentless(_) => "Relentless".to_owned(),
            Feature::ShadowStealth => "Shadow Stealth".to_owned(),
            Feature::Shapechanger(..) => "Shapechanger".to_owned(),
            Feature::SiegeMonster => "Siege Monster".to_owned(),
            Feature::SpiderClimb => "Spider Climb".to_owned(),
            Feature::StandingLeap(_) => "Standing Leap".to_owned(),
            Feature::Steadfast => "Steadfast".to_owned(),
            Feature::Stench(..) => "Stench".to_owned(),
            Feature::SunlightSensitivity => "Sunlight Sensitivity".to_owned(),
            Feature::SureFooted => "Sure Footed".to_owned(),
            Feature::SurpriseAttack(_) => "Surprise Attack".to_owned(),
            Feature::Swarm => "Swarm".to_owned(),
            Feature::Tunneler(_) => "Tunneler".to_owned(),
            Feature::TurnResistance => "Turn Resistance".to_owned(),
            Feature::TwoHeads => "Two Heads".to_owned(),
            Feature::UndeadFortitude => "Undead Fortitude".to_owned(),
            Feature::WebSense => "Web Sense".to_owned(),
            Feature::WebWalker => "Web Walker".to_owned(),
            Feature::Feature(name,..) => name.clone(),
       
        }
    }

    pub fn get_description(&self) -> String {
        match self {
            Feature::Aggressive =>format!("As a bonus action, ${{subj}} can move up to its speed toward a hostile creature that it can see."),
            Feature::Ambusher => format!("${{Subj}} has advantage on attack rolls against any creature it has surprised."),
            Feature::Amorphous => format!("${{Subj}} can move through a space as narrow as 1 inch wide without squeezing."),
            Feature::Amphibious => format!("${{Subj}} can breathe air and water."),
            Feature::AngelicWeapons(dice,..) => format!("${{Poss}} weapon attacks are magical. When ${{subj}} hits with any weapon, the weapon deals an extra {} radiant damage (included in the attack).",dice), 
            Feature::AntimagicSusceptibility => format!("${{Subj}} is incapacitated while in the area of an antimagic field. If targeted by dispel magic, ${{subj}} must succeed on a Constitution saving throw against the caster's spell save DC or fall unconscious for 1 minute."),
            Feature::BlindSenses => format!("${{Subj}} can't use its blindsight while deafened and unable to smell."),
            Feature::BloodFrenzy => format!("${{Subj}} has advantage on melee attack rolls against any creature that doesn't have all its hit points."),
            Feature::Brute(..) => format!("A melee weapon deals one extra die of its damage when ${{subj}} hits with it (included in the attack)."),
            Feature::Charge(distance,attack,extra_dice,damage) => format!("If ${{subj}} moves at least {} ft. straight toward a target and then hits it with a {} attack on the same turn, the target takes an extra {} {} damage.",distance,attack,extra_dice,damage),
            Feature::DamageTransfer => format!("While it is grappling a creature, ${{subj}} takes only half the damage dealt to it, and the creature grappled by ${{subj}} takes the other half."),
            Feature::DeathBurst(description,attack,compound) => format!("When ${{subj}} dies, it explodes in a cloud of {}, 5 ft. in radius. {}",description,attack.get_description("0",compound)), // explode description, range,  effect of explosion
            Feature::DevilSight => format!("Magical darkness doesn't impede ${{poss}} darkvision."),
            Feature::Echolocation => format!("${{Subj}} can't use its blindsight while deafened."),
            Feature::FalseAppearance(description) => description.clone(),
            Feature::FeyAncestry => format!("${{Subj}} has advantage on saving throws against being charmed, and magic can't put ${{subj}} to sleep."),
            Feature::Flyby => format!("${{Subj}} doesn't provoke opportunity attacks when it flies out of an enemy's reach."),
            Feature::Grappler => format!("${{Subj}} has advantage on attack rolls against any creature grappled by it."),
            Feature::HoldBreath(duration,reversed) => if *reversed {
                format!("While out of water, ${{subj}} can hold its breath for {} minutes.",duration)
            } else {
                format!("${{Subj}} can hold its breath for {} minutes.",duration)
            }, // duration, reversed ("out of water")
            Feature::Illumination(radius) => format!("${{Subj}} sheds bright light in a {}-foot radius and dim light in an additional {} ft.",radius,radius), // radius
            Feature::ImmutableForm => format!("${{Subj}} is immune to any spell or effect that would alter its form."),
            Feature::IncorporealMovement(damage) => format!("${{Subj}} can move through other creatures and objects as if they were difficult terrain. It takes {} force damage if it ends its turn inside an object.",damage), // damage (force) if it ends its turn inside an object
            Feature::Inscrutable => format!("${{Subj}} is immune to any effect that would sense its emotions or read its thoughts, as well as any divination spell that it refuses. Wisdom (Insight) checks made to ascertain ${{poss}} intentions or sincerity have disadvantage."),
            Feature::Invisibility => format!("${{Subj}} magically turns invisible until it attacks, or until its concentration ends (as if concentrating on a spell). Any equipment ${{subj}} wears or carries is invisible with it."),
            Feature::KeenSenses(sight,hearing,smell) => format!("${{Subj}} has advantage on Wisdom (Perception) checks that rely on {}.",match (hearing,sight,smell) {
                (true,true,true) => "sight, hearing, or smell",
                (true,true,false) => "sight or hearing",
                (true,false,true) => "sight or smell",
                (true,false,false) => "sight",
                (false,true,true) => "hearing or smell",
                (false,true,false) => "hearing",
                (false,false,true) => "smell",
                (false,false,false) => "no senses"
            }), // hearing, sight, smell
            Feature::LabyrinthineRecall => format!("${{Subj}} can perfectly recall any path it has traveled."),
            Feature::LegendaryResistance => format!("If ${{subj}} fails a saving throw, it can choose to succeed instead."),
            Feature::LightSensitivity => format!("While in bright light, ${{subj}} has disadvantage on attack rolls and Wisdom (Perception) checks that rely on sight."),
            Feature::MagicResistance => format!("${{Subj}} has advantage on saving throws against spells and other magical effects."),
            Feature::MagicWeapons => format!("${{Poss}} weapon attacks are magical."),
            Feature::MartialAdvantage(dice) => format!("Once per turn, ${{subj}} can deal an extra {} damage to a creature it hits with a weapon attack if that creature is within 5 ft. of an ally of ${{subj}} that isn't incapacitated.",dice), // extra damage dice
            Feature::Mimicry(sounds,check_dc) => format!("${{Subj}} can mimic {}. A creature that hears the sounds can tell they are imitations with a successful DC {} Wisdom (Insight) check.",sounds,check_dc), // description, Insight DC
            Feature::NimbleEscape => format!("${{Subj}} can take the Disengage or Hide action as a bonus action on each of its turns."),
            Feature::PackTactics => format!("${{Subj}} has advantage on an attack roll against a creature if at least one of ${{poss}} allies is within 5 ft. of the creature and the ally isn't incapacitated."),
            Feature::Pounce(distance,save_dc) => format!("If ${{subj}} moves at least {} ft. straight toward a creature and then hits it with a claw attack on the same turn, that target must succeed on a DC {} Strength saving throw or be knocked prone. If the target is prone, ${{subj}} can make one bite attack against it as a bonus action.",distance,save_dc), // distance, save DC (Strength)
            Feature::Rampage => format!("When ${{subj}} reduces a creature to 0 hit points with a melee attack on its turn, ${{subj}} can take a bonus action to move up to half its speed and make a bite attack."),
            Feature::Reactive => format!("${{Subj}} can take one reaction on every turn in combat."),
            Feature::Reckless => format!("At the start of its turn, ${{subj}} can gain advantage on all melee weapon attack rolls during that turn, but attack rolls against it have advantage until the start of its next turn."),
            Feature::Regeneration(regain_hp,vulnerabilities) => format!("${{Subj}} regains {} hit points at the start of its turn if it has at least 1 hit point. {}${{Subj}} dies only if it starts its turn with 0 hit points and doesn't regenerate.",regain_hp,vulnerabilities.and_join()), // number of hp, damage this doesn't apply to (might be based on vulnerabilities, in which case I don't need that)
            // "The vampire regains 20 hit points at the start of its turn if it has at least 1 hit point and isn't in sunlight or running water. If the vampire takes radiant damage or damage from holy water, this trait doesn't function at the start of the vampire's next turn."
            // "The oni regains 10 hit points at the start of its turn if it has at least 1 hit point."
            // "The troll regains 10 hit points at the start of its turn. If the troll takes acid or fire damage, this trait doesn't function at the start of the troll's next turn. The troll dies only if it starts its turn with 0 hit points and doesn't regenerate."
            Feature::Rejuvenation(description) => description.to_owned(), // needs a description, every rejuvenation is different
            Feature::Relentless(limit) => format!("If ${{subj}} takes {} damage or less that would reduce it to 0 hit points, it is reduced to 1 hit point instead.",limit), // damage limit
            Feature::ShadowStealth => format!("While in dim light or darkness, ${{subj}} can take the Hide action as a bonus action."),
            Feature::Shapechanger(description) => description.to_owned(),
            Feature::SiegeMonster => format!("${{Subj}} deals double damage to objects and structures."),
            Feature::SpiderClimb => format!("${{Subj}} can climb difficult surfaces, including upside down on ceilings, without needing to make an ability check."),
            Feature::StandingLeap(long) => format!("${{Poss}} long jump is up to {} ft. and its high jump is up to {} ft., with or without a running start.",long,long / 2), // distance
            Feature::Steadfast => format!("${{Subj}} can't be frightened while it can see an allied creature within 30 feet of it."),
            Feature::Stench(range,save_dc) => format!("Any creature that starts its turn within {} ft. of ${{subj}} must succeed on a DC {} Constitution saving throw or be poisoned until the start of its next turn. On a successful saving throw, the creature is immune to ${{poss}} stench for 24 hours.",range,save_dc), // range, save DC (Constitution)
            Feature::SunlightSensitivity => format!("While in sunlight, ${{subj}} has disadvantage on attack rolls, as well as on Wisdom (Perception) checks that rely on sight."),
            Feature::SureFooted => format!("${{Subj}} has advantage on Strength and Dexterity saving throws made against effects that would knock it prone."),
            Feature::SurpriseAttack(damage) => format!("If ${{subj}} surprises a creature and hits it with an attack during the first round of combat, the target takes an extra {} damage from the attack.",damage), // extra damage
            Feature::Swarm => format!("The swarm can occupy another creature's space and vice versa, and the swarm can move through any opening large enough for a Tiny individual. The swarm can't regain hit points or gain temporary hit points."),
            Feature::Tunneler(diameter) => format!("${{Subj}} can burrow through solid rock at half its burrow speed and leaves a {}-foot-diameter tunnel in its wake.",diameter), // width of tunnel
            Feature::TurnResistance => format!("${{Subj}} has advantage on saving throws against any effect that turns undead."),
            Feature::TwoHeads => format!("${{Subj}} has advantage on Wisdom (Perception) checks and on saving throws against being blinded, charmed, deafened, frightened, stunned, and knocked unconscious."),
            Feature::UndeadFortitude => format!("If damage reduces ${{subj}} to 0 hit points, it must make a Constitution saving throw with a DC of 5+the damage taken, unless the damage is radiant or from a critical hit. On a success, ${{subj}} drops to 1 hit point instead."),
            Feature::WebSense => format!("While in contact with a web, ${{subj}} knows the exact location of any other creature in contact with the same web."),
            Feature::WebWalker => format!("${{Subj}} ignores movement restrictions caused by webbing."),
            Feature::Feature(_,description) => description.clone(), // name, description
        }
        
    }

}
