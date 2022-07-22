/*
 * Copyright © 2022 Neil M. Sheldon
 * 
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::rc::Rc;
use std::collections::HashMap;
use std::convert::TryFrom;

use crate::dice::Die;
use crate::dice::Dice;
use crate::dice_expression::DiceExpression;
use crate::utils::AndJoin;
use crate::utils::Capitalize;
use crate::interpolation::InterpolationObject;
use crate::interpolation::InterpolationValue;
use crate::interpolation::interpolate_str_for_statblock;
use crate::errors::InterpolationErrorDetails;
use crate::errors::CreatureError;
use crate::stats::Damage;
use crate::stats::Ability;
use crate::stats::Skill;
use crate::stats::Condition;
use crate::stats::Language;
use crate::attacks::Attack;
use crate::attacks::AttackEffect;
use crate::attacks::CompoundAttackEffect;
use crate::attacks::Multiattack;
use crate::attacks::Weapon;
use crate::stats::Armor;
use crate::actions::UsageLimit;
use crate::actions::Action;
use crate::spellcasting::Spellcasting;
use crate::spellcasting::InnateSpellcasting;
use crate::actions::LegendaryAction;
use crate::reactions::Reaction;
use crate::features::Feature;
use crate::stats::CreatureSize;
use crate::stats::CreatureType;
use crate::stats::Alignment;
use crate::stats::ChallengeRating;
use crate::stat_block::StatBlockLegendary;
use crate::stat_block::StatBlockFeature;
use crate::stat_block::CreatureStatBlock;
use crate::structured_text::TextBlock;


pub struct CreatureFeature {
    pub name: String,
    pub description: String,
    pub usage_limit: Option<UsageLimit>
}

impl CreatureFeature {

    fn feature_to_text_block(name: &str, description: &str, usage_limit: &Option<UsageLimit>, data: &Rc<Creature>) -> Result<Vec<TextBlock>,InterpolationErrorDetails> {
        let source = format!("${{par(}}{}{}.${{)}}{}",name,if let Some(usage_limit) = usage_limit {
            format!(" ({})",usage_limit)
        } else {
            String::new()
        },description);
        interpolate_str_for_statblock(&source, name, data, true)
    }


    fn to_stat_block(&self, data: &Rc<Creature>) -> Result<StatBlockFeature,InterpolationErrorDetails> {
        Ok(StatBlockFeature {
            text: Self::feature_to_text_block(&self.name, &self.description, &self.usage_limit, data)?
        })
    }
}

pub enum CreatureSpecialAbility {
    Feature(CreatureFeature),
    Spellcasting(Spellcasting),
    InnateSpellcasting(InnateSpellcasting),
}

pub struct CreatureAction {
    pub name: String, 
    pub description: String,
    pub usage_limit: Option<UsageLimit>,
    pub attack: Option<Attack>,
    pub effect: Option<AttackEffect>,
    pub compound: Option<CompoundAttackEffect>
}

impl CreatureAction {


    pub fn new_from_action(action: &Action, usage_limit: &Option<UsageLimit>) -> Self {


        Self {
            name: action.get_name(),
            description: action.get_description(),
            attack: action.get_attack(),
            effect: action.get_effect(),
            compound: action.get_compound_effect(),
            usage_limit: usage_limit.clone()
        }
    }

}

// simmilar to CreatureAction, but there's a 'cost' instead of a UsageLimit
pub struct CreatureLegendaryAction {
    pub cost: u8,
    pub name: String,
    pub description: String,
    pub attack: Option<Attack>,
    pub effect: Option<AttackEffect>,
    pub compound: Option<CompoundAttackEffect>
}

impl CreatureLegendaryAction {

    pub fn new(source: &LegendaryAction, creature: &Creature) -> Result<Self,CreatureError> {
        Ok(match source {
            LegendaryAction::UseAction(cost,name,description,action_name) => {
                if let Some(action) = creature.find_action(action_name) {
                    Self {
                        cost: *cost,
                        name: name.clone(),
                        description: description.clone(),
                        attack: action.attack.clone(),
                        effect: action.effect.clone(),
                        compound: action.compound.clone(),
                    }

                } else {
                    Err(CreatureError::ActionNotFound(name.clone(),"while adding legendary action".to_owned()))?
                }
            },
            LegendaryAction::UseWeapon(cost,name,description,weapon) => {
                if let Some(action) = creature.find_weapon(weapon) {
                    Self {
                        cost: *cost,
                        name: name.clone(),
                        description: description.clone(),
                        attack: action.attack.clone(),
                        effect: action.effect.clone(),
                        compound: action.compound.clone(),

                    }

                } else {
                    Err(CreatureError::WeaponNotFound(name.clone(),"while adding legendary action".to_owned()))?
                }
            },
            LegendaryAction::LegendaryAction(cost,action) => {
                let action = CreatureAction::new_from_action(action, &None); 
                Self {
                    cost: *cost,
                    name: action.name,
                    description: action.description,
                    attack: action.attack,
                    effect: action.effect,
                    compound: action.compound,
        }
            }
        })
    }


}

pub struct CreatureLegendaryActions {
    pub description: String,
    pub actions: Vec<CreatureLegendaryAction>
}

impl CreatureLegendaryActions {

    pub fn to_stat_block(&self, data: &Rc<Creature>) -> Result<StatBlockLegendary,InterpolationErrorDetails> {
        Ok(StatBlockLegendary {
            description: interpolate_str_for_statblock(&self.description,"legendary actions: description",data.as_ref(),true)?,
            actions: self.actions.iter().map(|a| {
                // NOTE: this is different from regular actions, it's a subparagraph block, and has cost instead of usage limit
                let source = format!("${{sub(}}{}{}.${{)}}{}",&a.name,if a.cost != 1 {
                    format!(" (Costs {} Actions)",a.cost)
                } else {
                    "".to_owned()
                },&a.description);
                Ok(StatBlockFeature {
                    text: interpolate_str_for_statblock(&source, &a.name, data, true)?
                })
            }).collect::<Result<Vec<StatBlockFeature>,InterpolationErrorDetails>>()?
        })
    }

    

}

pub struct CreatureSpeed {
    pub walk: Option<u8>,
    pub swim: Option<u8>,
    pub fly: Option<u8>,
    pub burrow: Option<u8>,
    pub climb: Option<u8>,
    pub hover: bool,
    pub notes: Option<String>,
    pub custom: HashMap<String,u8>

}

impl CreatureSpeed {

    pub fn to_stat_block(&self) -> String {

        let mut result = vec![];
        if let Some(walk) = self.walk {
            result.push(format!("{} ft.",walk))
        }
        if let Some(burrow) = self.burrow {
            result.push(format!("burrow {} ft.",burrow))
        }
        if let Some(climb) = self.climb {
            result.push(format!("climb {} ft.",climb))
        }
        if let Some(fly) = self.fly {
            if self.hover {
                result.push(format!("fly {} ft. (hover)",fly))
            } else {
                result.push(format!("fly {} ft.",fly))
            }
        }
        if let Some(swim) = self.swim {
            result.push(format!("swim {} ft.",swim))
        }
        for (movement,feet) in &self.custom {
            result.push(format!("{} {} ft.",movement,feet))
        }

        if let Some(notes) = &self.notes {
            format!("{} ({})",result.join(", "),notes)
        } else {
            result.join(", ")
        }
        
    }
}

impl Default for CreatureSpeed {

    fn default() -> Self {
        Self {
            walk: Some(0),
            swim: None,
            fly: None,
            burrow: None,
            climb: None,
            hover: false,
            notes: None,
            custom: HashMap::new()
        }
    }
}


#[derive(Default)]
pub struct CreatureProficiencies {
    // no proficiency: None,
    // proficiency: Some(false),
    // expertise: Some(true)
    pub athletics: Option<bool>,
    pub acrobatics: Option<bool>,
    pub sleight_of_hand: Option<bool>,
    pub stealth: Option<bool>,
    pub arcana: Option<bool>,
    pub history: Option<bool>,
    pub investigation: Option<bool>,
    pub nature: Option<bool>,
    pub religion: Option<bool>,
    pub animal_handling: Option<bool>,
    pub insight: Option<bool>,
    pub medicine: Option<bool>,
    pub perception: Option<bool>,
    pub survival: Option<bool>,
    pub deception: Option<bool>,
    pub intimidation: Option<bool>,
    pub performance: Option<bool>,
    pub persuasion: Option<bool>,
    pub result_perception: Option<bool>,

}

impl CreatureProficiencies {


    fn calculate_skill_bonus(skill: &Skill, creature: &Creature, expertise: &bool) -> i8 {
        let mod_bonus = Ability::score_to_mod(match skill {
            Skill::Athletics => creature.strength,
            Skill::Acrobatics |
            Skill::SleightOfHand |
            Skill::Stealth => creature.dexterity,
            Skill::Arcana |
            Skill::History |
            Skill::Investigation |
            Skill::Nature |
            Skill::Religion => creature.intelligence,
            Skill::AnimalHandling |
            Skill::Insight |
            Skill::Medicine |
            Skill::Perception |
            Skill::Survival => creature.wisdom,
            Skill::Deception |
            Skill::Intimidation |
            Skill::Performance |
            Skill::Persuasion => creature.charisma      
        });
        if *expertise {
            mod_bonus + (creature.challenge_rating.get_proficiency_bonus() as i8 * 2)
        } else {
            mod_bonus + creature.challenge_rating.get_proficiency_bonus() as i8
        }

    }


    fn to_stat_block(&self, creature: &Creature) -> (Option<String>,Option<i8>) {

        let mut result = vec![];
        let mut result_perception = None;

        macro_rules! check_perception {
            (Perception, $score: ident) => {
                result_perception = Some($score)
            };
            ($skill: ident, $score: ident) => {
                // empty
            };
        }

        macro_rules! add_skill {
            ($prop: ident, $skill: ident, $name: literal) => {
                if let Some(expertise) = self.$prop {
                    let score = Self::calculate_skill_bonus(&Skill::$skill,creature,&expertise);
                    result.push(format!(concat!($name," {:+}"),score));
                    check_perception!($skill,score);
                }
            };
        }

        add_skill!(acrobatics,Acrobatics,"Acrobatics");
        add_skill!(animal_handling,AnimalHandling,"Animal Handling");
        add_skill!(arcana,Arcana,"Arcana");
        add_skill!(athletics,Athletics,"Athletics");
        add_skill!(deception,Deception,"Deception");
        add_skill!(history,History,"History");
        add_skill!(insight,Insight,"Insight");
        add_skill!(intimidation,Intimidation,"Intimidation");
        add_skill!(investigation,Investigation,"Investigation");
        add_skill!(medicine,Medicine,"Medicine");
        add_skill!(nature,Nature,"Nature");
        add_skill!(perception,Perception,"Perception");
        add_skill!(performance,Performance,"Performance");
        add_skill!(persuasion,Persuasion,"Persuasion");
        add_skill!(religion,Religion,"Religion");
        add_skill!(sleight_of_hand,SleightOfHand,"Sleight of Hand");
        add_skill!(stealth,Stealth,"Stealth");
        add_skill!(survival,Survival,"Survival");


        (if result.len() > 0 {
            Some(result.join(", "))
        } else {
            None
        },
        result_perception)


    }

}

#[derive(Default)]
pub struct CreatureSenses {
    pub blindsight: Option<(u8,bool)>,
    pub darkvision: Option<u8>,
    pub tremorsense: Option<u8>,
    pub truesight: Option<u8>,
    pub custom: HashMap<String,u8>
}

impl CreatureSenses {

    fn to_stat_block(&self, perception: Option<i8>, wisdom: u8) -> String {
        
        let mut result = vec![];

        if let Some((distance,blind)) = self.blindsight {
            result.push(format!("blindsight {} ft.{}",distance,if blind {
                " (blind beyond this radius)"
            } else {
                ""
            }))
        };

        if let Some(distance) = self.darkvision {
            result.push(format!("darkvision {} ft.",distance))
        }

        if let Some(distance) = self.tremorsense {
            result.push(format!("tremorsense {} ft.",distance))
        }

        if let Some(distance) = self.truesight {
            result.push(format!("truesight {} ft.",distance))
        }

        for (sense,distance) in &self.custom {
            result.push(format!("{} {} ft.",sense,distance))
        }

        result.push(format!("passive Perception {}",if let Some(perception) = perception {
            10 + perception
        } else {
            10 + Ability::score_to_mod(wisdom)
        }));
        result.join(", ")

    }

}

#[derive(Default)]
pub struct CreatureResistances {
    pub bludgeoning: bool,
    pub piercing: bool,
    pub slashing: bool,
    pub cold: bool,
    pub fire: bool,
    pub thunder: bool,
    pub radiant: bool,
    pub force: bool,
    pub lightning: bool,
    pub poison: bool,
    pub acid: bool,
    pub necrotic: bool,
    pub psychic: bool,
    pub all: bool,
    pub non_magical_attacks: bool,
    pub non_silvered_attacks: bool,
    pub non_adamantine_attacks: bool,
    pub custom: Option<String>,
    
}

impl CreatureResistances {

    fn to_stat_block(&self) -> Option<String> {

        if let Some(custom) = &self.custom {
            Some(custom.clone())
        } else {
            let mut regular_damage = vec![];
            let mut special_damage = vec![];
    
            macro_rules! push_regular {
                ($prop: ident, $damage: ident) => {
                    if self.all || self.$prop {
                        regular_damage.push(Damage::$damage.to_string())
                    }
                };
            }
    
            push_regular!(bludgeoning,Bludgeoning);
            push_regular!(piercing,Piercing);
            push_regular!(slashing,Slashing);
            push_regular!(cold,Cold);
            push_regular!(fire,Fire);
            push_regular!(thunder,Thunder);
            push_regular!(radiant,Radiant);
            push_regular!(lightning,Lightning);
            push_regular!(poison,Poison);
            push_regular!(acid,Acid);
            push_regular!(necrotic,Necrotic);
            push_regular!(psychic,Psychic);
            
            if self.non_magical_attacks {
                special_damage.push("bludgeoning, piercing, and slashing from nonmagical attacks");
            }
            if self.non_adamantine_attacks {
                special_damage.push("bludgeoning, piercing, and slashing from nonmagical attacks that aren't adamantine");
            }
            if self.non_silvered_attacks {
                special_damage.push("bludgeoning, piercing, and slashing from nonmagical attacks that aren't silvered");
            }
    
            if (regular_damage.len() > 0) || (special_damage.len() > 0) {
                let mut result = regular_damage.join(", ");
                if special_damage.len() > 0 {
                    result.push_str("; ");
                    result.push_str(&special_damage.join("; "))
                };
                Some(result)
            } else {
                None
            }
        
        }


    }

}

#[derive(Default)]
pub struct CreatureConditionImmunities {
    pub blinded: bool,
    pub charmed: bool,
    pub deafened: bool,
    pub exhaustion: bool,
    pub frightened: bool,
    pub grappled: bool,
    pub incapacitated: bool,
    pub invisible: bool,
    pub paralyzed: bool,
    pub petrified: bool,
    pub poisoned: bool,
    pub prone: bool,
    pub restrained: bool,
    pub stunned: bool,
    pub unconscious: bool,
}

impl CreatureConditionImmunities {


    fn to_stat_block(&self) -> Option<String> {
        let mut result = vec![];

        macro_rules! add_condition {
            ($prop: ident, $condition: ident) => {
                if self.$prop {
                    result.push(Condition::$condition.to_string())
                }
                
            };
        }

        add_condition!(blinded,Blinded);
        add_condition!(charmed,Charmed);
        add_condition!(deafened,Deafened);
        add_condition!(exhaustion,Exhaustion);
        add_condition!(frightened,Frightened);
        add_condition!(grappled,Grappled);
        add_condition!(incapacitated,Incapacitated);
        add_condition!(invisible,Invisible);
        add_condition!(paralyzed,Paralyzed);
        add_condition!(petrified,Petrified);
        add_condition!(poisoned,Poisoned);
        add_condition!(prone,Prone);
        add_condition!(restrained,Restrained);
        add_condition!(stunned,Stunned);
        add_condition!(unconscious,Unconscious);

        if result.len() > 0 {
            Some(result.join(", "))
        } else {
            None
        }
    }
}

#[derive(Default)]
pub struct CreatureArmor {
    armor: Option<Armor>,
    shield: bool,
    mage_armor: bool
}

impl CreatureArmor {

    fn get_armor_class(&self, dexterity_mod: i8) -> u8 {
        
        (match self.armor {
            Some(Armor::Padded) => (11 + dexterity_mod) as u8,
            Some(Armor::Leather) => (11 + dexterity_mod) as u8,
            Some(Armor::StuddedLeather) => (12 + dexterity_mod) as u8,
            Some(Armor::Hide) => (12 + dexterity_mod.min(2)) as u8,
            Some(Armor::ChainShirt) => (13 + dexterity_mod.min(2)) as u8,
            Some(Armor::ScaleMail) => (14 + dexterity_mod.min(2)) as u8,
            Some(Armor::Breastplate) => (14 + dexterity_mod.min(2)) as u8,
            Some(Armor::HalfPlate) => (15 + dexterity_mod.min(2)) as u8,
            Some(Armor::RingMail) => 14,
            Some(Armor::ChainMail) => 16,
            Some(Armor::Splint) => 17,
            Some(Armor::Plate) => 18,
            Some(Armor::Natural(a)) => (10 + a + dexterity_mod) as u8, // natural bonus
            Some(Armor::Armor(a,_)) => (10 + a as i8 + dexterity_mod) as u8, // value, description
            _ => (10 + dexterity_mod) as u8
        }) + if self.shield {
            2
        } else {
            0
        } // mage armor is only applied to the description, not to the actual ac.
    }

    fn get_description(&self, dexterity_mod: i8) -> Option<String> {
        let mut result = Vec::new();
        if let Some(armor) = &self.armor {
            result.push(match armor {
                Armor::Padded => "padded armor",
                Armor::Leather => "leather armor",
                Armor::StuddedLeather => "studded leather armor",
                Armor::Hide => "hide armor",
                Armor::ChainShirt => "chain shirt",
                Armor::ScaleMail => "scale mail armor",
                Armor::Breastplate => "breastplate",
                Armor::HalfPlate => "half plate armor",
                Armor::RingMail => "ring mail armor",
                Armor::ChainMail => "chain mail armor",
                Armor::Splint => "splint armor",
                Armor::Plate => "plate armor",
                Armor::Natural(_) => "natural armor",
                Armor::Armor(_,b) => b, // value, description
            }.to_owned())
        }

        
        if self.shield {
            result.push("shield".to_owned());
        }

        if self.mage_armor {
            result.push(format!("{} with ${{italic( }}mage armor${{ )}}",(13 + dexterity_mod)))
        }
        
        if result.len() > 0 {
            Some(result.join(", "))
        } else {
            None
        }
        

    }

}


// putting this documentation here because I can't link to the implementation for ImplementationObject.
/**
The following properties can be retrieved during interpolation. Their value types are listed in parentheses after.

* `name (string)`: The name of the creature
* `subj (string)`: The name of the creature as used for the subject of an action.
* `Subj (string)`: The subject of the creature capitalized.
* `poss (string)`: The name of the creature in possessive form.
* `Poss (string)`: The capitalized possessive.
* `subjpro (string)`: The subject pronoun for the creature  (default 'it')
* `Subjpro (string)`: The subject pronoun for the creature capitalized (default 'It')
* `objpro (string)`: the object pronoun for the creature (default 'it')
* `refpro (string)`: the reflexive pronoun for the creature (default 'itself')
* `posspro (string)`: the possessive pronoun for the creature (default 'its`)
* `Posspro (string)`: the possessive pronoun for the creature capitalized (default `Its`)
* `size (string)`: The size of the creature
* `type (string)`: The type of the creature
* `subtype (string)`: The subtype of the creature, or an empty string
* `group (string)`: The creature's group, or an empty string.
* `alignment (string)`: The alignment of the creature
* `hit_dice (dice)`: The calculated hit dice for the creature.
* `hit_points (number)`: The calculate hit points for the creature.
* `armor_class (number)`: The calculated armor class for the creature.
* `strength (number)`: The strength score for the creature.
* `dexterity (number)`: The dexterity score for the creature.
* `constitution (number)`: The constitution score for the creature.
* `intelligence (number)`: The intelligence score for the creature.
* `wisdom (number)`: The wisdom score for the creature.
* `charisma (number)`: The charisma score for the creature.
* `atk (number)`: The best of the dexterity and strength modifiers.
* `spell_atk (number)`: The calculated spell attack bonus. If the creature has both innate and regular spellcasting, this will return the innate bonus. If the creature has neither, this property will not be available, and an error will occur.
* `str (number)`: The calculated strength modifier.
* `dex (number)`: The calculated dexterity modifier.
* `con (number)`: The calculated constitution modifier.
* `int (number)`: The calculated intelligence modifier.
* `wis (number)`: The calculated wisdom modifier.
* `cha (number)`: The calculated charisma modifier.
* `str_save (number)`: The calculated save bonus for strength.
* `dex_save (number)`: The calculated save bonus for dexterity.
* `con_save (number)`: The calculated save bonus for constitution.
* `int_save (number)`: The calculated save bonus for intelligence.
* `wis_save (number)`: The calculated save bonus for wisdom.
* `cha_save (number)`: The calculated save bonus for charisma.
* `prof (number)`: The proficiency bonus, taken from the creatures challenge rating.

*/
pub struct Creature {
    pub name: String,
    pub subject: Option<String>,
    pub subject_cap: Option<String>,
    pub possessive: Option<String>,
    pub possessive_cap: Option<String>,
    pub subject_pronoun: Option<String>,
    pub subject_pronoun_cap: Option<String>,
    pub possessive_pronoun: Option<String>,
    pub possessive_pronoun_cap: Option<String>,
    pub object_pronoun: Option<String>,
    pub reflexive_pronoun: Option<String>,
    pub size: CreatureSize,
    pub type_: CreatureType,
    pub subtype: Option<String>,
    pub group: Option<String>,
    pub alignment: Alignment,
    pub hit_die: Die,
    pub hit_dice_count: u8,
    pub override_hit_points: Option<u16>,
    pub armor: CreatureArmor,
    pub speed: CreatureSpeed,
    pub strength: u8,
    pub dexterity: u8,
    pub constitution: u8,
    pub intelligence: u8,
    pub wisdom: u8,
    pub charisma: u8,
    pub strength_save: bool,
    pub dexterity_save: bool,
    pub constitution_save: bool,
    pub intelligence_save: bool,
    pub wisdom_save: bool,
    pub charisma_save: bool,
    pub skills: CreatureProficiencies, // bool indicates expertise
    pub vulnerabilities: CreatureResistances,
    pub resistances: CreatureResistances,
    pub immunities: CreatureResistances,
    pub condition_immunities: CreatureConditionImmunities,
    pub senses: CreatureSenses,
    pub languages: Vec<(Language,bool)>, // language, whether it is spoken or just understood, a hashmap would be better, but it doesn't keeep the order.
    pub challenge_rating: ChallengeRating,
    pub multiattack: Option<Multiattack>,
    pub special_abilities: Vec<CreatureSpecialAbility>,
    pub actions: Vec<CreatureAction>,
    pub reactions: Vec<CreatureFeature>,
    pub legendary_actions: Option<CreatureLegendaryActions>,
    pub source: Option<String>

}

impl InterpolationObject for Creature {


    fn  get_property(&self, property: &Rc<str>) -> Option<InterpolationValue> {
        match property.as_ref() {
            "name" => Some(InterpolationValue::String(Rc::from(self.name.as_str()))),
            "subj" => Some(InterpolationValue::String(Rc::from(self.get_subject(false).as_str()))),
            "Subj" => Some(InterpolationValue::String(Rc::from(self.get_subject(true).as_str()))),
            "poss" => Some(InterpolationValue::String(Rc::from(self.get_possessive(false).as_str()))),
            "Poss" => Some(InterpolationValue::String(Rc::from(self.get_possessive(true).as_str()))),
            "subjpro" => Some(InterpolationValue::String(Rc::from(self.get_subject_pronoun(false).as_str()))),
            "Subjpro" => Some(InterpolationValue::String(Rc::from(self.get_subject_pronoun(true).as_str()))),
            "posspro" => Some(InterpolationValue::String(Rc::from(self.get_possessive_pronoun(false).as_str()))),
            "Posspro" => Some(InterpolationValue::String(Rc::from(self.get_possessive_pronoun(true).as_str()))),
            "objpro" => Some(InterpolationValue::String(Rc::from(self.get_object_pronoun(false).as_str()))),
            "refpro" => Some(InterpolationValue::String(Rc::from(self.get_reflexive_pronoun(false).as_str()))),
            "size" => Some(InterpolationValue::String(Rc::from(self.size.to_string()))),
            "type" => Some(InterpolationValue::String(Rc::from(self.type_.to_string()))),
            "subtype" => Some(InterpolationValue::String(Rc::from(self.subtype.as_deref().unwrap_or("")))),
            "group" => Some(InterpolationValue::String(Rc::from(self.group.as_deref().unwrap_or("")))),
            "alignment" => Some(InterpolationValue::String(Rc::from(self.alignment.to_string()))),
            "hit_dice" => Some(InterpolationValue::Dice(self.get_hit_dice(),false)), 
            "hit_points" => Some(InterpolationValue::Number(self.override_hit_points.map(|n| n as isize).unwrap_or(self.get_hit_dice().average()),false)),
            //"shield" => FUTURE: Can I do anything with this? I can't handle booleans right now, but maybe...
            "armor_class" => Some(InterpolationValue::Number(self.get_armor_class() as isize,false)),
            // "speed" -- FUTURE: Perhaps support a 'walk' or other speed?
            "strength" => Some(InterpolationValue::Number(self.strength as isize,false)),
            "dexterity" => Some(InterpolationValue::Number(self.dexterity as isize,false)),
            "constitution" => Some(InterpolationValue::Number(self.constitution as isize,false)),
            "intelligence" => Some(InterpolationValue::Number(self.intelligence as isize,false)),
            "wisdom" => Some(InterpolationValue::Number(self.wisdom as isize,false)),
            "charisma" => Some(InterpolationValue::Number(self.charisma as isize,false)),
            "atk" => if self.strength > self.dexterity {
                Some(InterpolationValue::Number(Ability::score_to_mod(self.strength) as isize,false))
            } else {
                Some(InterpolationValue::Number(Ability::score_to_mod(self.dexterity) as isize,false))
            },
            "spell_atk" => self.get_spell_attack_bonus().map(|a| InterpolationValue::Number(a as isize,false)),
            "str" => Some(InterpolationValue::Number(Ability::score_to_mod(self.strength) as isize,false)),
            "dex" => Some(InterpolationValue::Number(Ability::score_to_mod(self.dexterity) as isize,false)),
            "con" => Some(InterpolationValue::Number(Ability::score_to_mod(self.constitution) as isize,false)),
            "int" => Some(InterpolationValue::Number(Ability::score_to_mod(self.intelligence) as isize,false)),
            "wis" => Some(InterpolationValue::Number(Ability::score_to_mod(self.wisdom) as isize,false)),
            "cha" => Some(InterpolationValue::Number(Ability::score_to_mod(self.charisma) as isize,false)),
            "str_save" => Some(InterpolationValue::Number(Creature::saving_throw_to_stat_block(Ability::Strength, self) as isize,false)),
            "dex_save" => Some(InterpolationValue::Number(Creature::saving_throw_to_stat_block(Ability::Dexterity, self) as isize,false)),
            "con_save" => Some(InterpolationValue::Number(Creature::saving_throw_to_stat_block(Ability::Constitution, self) as isize,false)),
            "int_save" => Some(InterpolationValue::Number(Creature::saving_throw_to_stat_block(Ability::Intelligence, self) as isize,false)),
            "wis_save" => Some(InterpolationValue::Number(Creature::saving_throw_to_stat_block(Ability::Wisdom, self) as isize,false)),
            "cha_save" => Some(InterpolationValue::Number(Creature::saving_throw_to_stat_block(Ability::Charisma, self) as isize,false)),
            // FUTURE: skills? vulnerabilitys, resistance, immunities, sense, languages, etc. access data on other actions, reactions, etc?
            "prof" => Some(InterpolationValue::Number(self.challenge_rating.get_proficiency_bonus() as isize,false)),
            _ => None
        }
    }


}

impl Default for Creature {

    fn default() -> Self {
        Self {
            name: "".to_owned(), //String,
            subject: None,
            subject_cap: None,
            possessive: None,
            possessive_cap: None,
            subject_pronoun: None,
            subject_pronoun_cap: None,
            possessive_pronoun: None,
            possessive_pronoun_cap: None,
            object_pronoun: None,
            reflexive_pronoun: None,
            size: CreatureSize::Medium, //CreatureSize,
            type_: CreatureType::Humanoid, //CreatureType,
            subtype: None,//Option<String>,
            group: None, //Option<String>,
            alignment: Alignment::AnyAlignment, //Alignment,
            hit_die: Die::D6,
            hit_dice_count: 1,
            override_hit_points: None, //Option<u16>,
            armor: CreatureArmor::default(), //Armor,
            speed: CreatureSpeed::default(),
            strength: 10, //u8,
            dexterity: 10,//u8,
            constitution: 10,//u8,
            intelligence: 10,//u8,
            wisdom: 10,//u8,
            charisma: 10,//u8,
            strength_save: false, //Option<()>,
            dexterity_save: false, //Option<()>,
            constitution_save: false, //Option<()>,
            intelligence_save: false, //Option<()>,
            wisdom_save: false, //Option<()>,
            charisma_save: false, //Option<()>,
            skills: CreatureProficiencies::default(), 
            vulnerabilities: CreatureResistances::default(),//Vec<Damage>,
            resistances: CreatureResistances::default(), //Vec<Damage>,
            immunities: CreatureResistances::default(), //Vec<Damage>,
            condition_immunities: CreatureConditionImmunities::default(), //Vec<Condition>,
            senses: CreatureSenses::default(), //Vec<Sense>,
            languages: Vec::new(), //Vec<(Language,bool)>, // language, whether it is spoken or just understood
            challenge_rating: ChallengeRating::None, //ChallengeRating,
            multiattack: None,
            actions: Vec::new(),//Vec<CreatureAction>,
            reactions: Vec::new(), //Vec<ReactionData>,
            legendary_actions: None, //Option<CreatureLegendaryActions>,
            special_abilities: Vec::new(), //Vec<CreatureFeature>
            source: None            
        }
        
    }    
}


impl Creature {

    fn get_subject(&self, capitalize: bool) -> String {
        if capitalize {
            if let Some(subject) = &self.subject_cap {
                subject.clone()
            } else if let Some(subject) = &self.subject {
                subject.capitalize_first_letter()
            } else {
                format!("The {}",self.name.to_lowercase())
            }
        } else {
            if let Some(subject) = &self.subject {
                subject.clone()
            } else {
                format!("the {}",self.name.to_lowercase())
            }
        }

    }

    fn get_possessive(&self, capitalize: bool) -> String {
        if capitalize {
            if let Some(possessive) = &self.possessive_cap {
                possessive.clone()
            } else if let Some(posessive) = &self.possessive {
                posessive.capitalize_first_letter()
            } else {
                format!("{}'s",self.get_subject(true))
            }
        } else {
            if let Some(posessive) = &self.possessive {
                posessive.clone()
            } else {
                format!("{}'s",self.get_subject(false))
            }
        }

    }

    fn get_subject_pronoun(&self, capitalize: bool) -> String {
        if capitalize {
            if let Some(pronoun) = &self.subject_pronoun_cap {
                pronoun.clone()
            } else if let Some(pronoun) = &self.subject_pronoun {
                pronoun.capitalize_first_letter()
            } else {
                format!("It")
            }
        } else {
            if let Some(pronoun) = &self.subject_pronoun {
                pronoun.clone()
            } else {
                format!("it")
            }
        }

    }

    fn get_possessive_pronoun(&self, capitalize: bool) -> String {
        if capitalize {
            if let Some(pronoun) = &self.possessive_pronoun_cap {
                pronoun.clone()
            } else if let Some(pronoun) = &self.possessive_pronoun {
                pronoun.capitalize_first_letter()
            } else {
                format!("Its")
            }
        } else {
            if let Some(pronoun) = &self.possessive_pronoun {
                pronoun.clone()
            } else {
                format!("its")
            }
        }

    }

    fn get_object_pronoun(&self, capitalize: bool) -> String {
        if capitalize {
            if let Some(pronoun) = &self.object_pronoun {
                pronoun.capitalize_first_letter()
            } else {
                format!("It")
            }
        } else {
            if let Some(pronoun) = &self.object_pronoun {
                pronoun.clone()
            } else {
                format!("it")
            }
        }

    }

    fn get_reflexive_pronoun(&self, capitalize: bool) -> String {
        if capitalize {
            if let Some(pronoun) = &self.reflexive_pronoun {
                pronoun.capitalize_first_letter()
            } else {
                format!("Itself")
            }
        } else {
            if let Some(pronoun) = &self.reflexive_pronoun {
                pronoun.clone()
            } else {
                format!("itself")
            }
        }

    }

    fn get_hit_dice(&self) -> DiceExpression {
        DiceExpression::from_dice(Dice::new(self.hit_dice_count,&self.hit_die),Ability::score_to_mod(self.constitution) as isize * self.hit_dice_count as isize)
    }

    fn saving_throw_to_stat_block(ability: Ability, creature: &Creature) -> i8 {
        let (score,proficient) = match ability {
            Ability::Strength => (creature.strength,creature.strength_save),
            Ability::Dexterity => (creature.dexterity,creature.dexterity_save),
            Ability::Constitution => (creature.constitution,creature.constitution_save),
            Ability::Intelligence => (creature.intelligence,creature.intelligence_save),
            Ability::Wisdom => (creature.wisdom,creature.wisdom_save),
            Ability::Charisma => (creature.charisma,creature.charisma_save),                    
        };
        let modifier = Ability::score_to_mod(score);
        if proficient {
            modifier + creature.challenge_rating.get_proficiency_bonus() as i8
        } else {
            modifier
        }
    }

    fn saving_throws_to_stat_block(creature: &Creature) -> Option<String> {
        let mut result = vec![];

        macro_rules! add_saving_throw {
            ($ability: ident, $Ability: ident, $save: ident, $abbrev: literal) => {
                if creature.$save {
                    result.push(format!(concat!($abbrev," {:+}"),Self::saving_throw_to_stat_block(Ability::$Ability, creature)))
                }
                        
            };
        }

        add_saving_throw!(strength,Strength,strength_save,"Str");
        add_saving_throw!(dexterity,Dexterity,dexterity_save,"Dex");
        add_saving_throw!(constitution,Constitution,constitution_save,"Con");
        add_saving_throw!(intelligence,Intelligence,intelligence_save,"Int");
        add_saving_throw!(wisdom,Wisdom,wisdom_save,"Wis");
        add_saving_throw!(charisma,Charisma,charisma_save,"Cha");

        if result.len() > 0 {
            Some(result.join(", ")) 
        } else {
            None
        }
            
    }




    fn languages_to_stat_block(list: &Vec<(Language,bool)>) -> Option<String> {

        let mut spoken_languages = vec![];
        let mut understood_languages = vec![];
        let mut telepathy = None;

        for (language,speaks) in list {
            if let Language::Telepathy(distance) = language {
                telepathy = Some(distance)
            } else if *speaks {
                spoken_languages.push(language.to_string())
            } else {
                understood_languages.push(language.to_string())
            };

        }

        // byssal, Aquan, Deep Speech, understands Auran, Celestial, and Ignan, telepathy 15 ft.
        if understood_languages.len() > 0 {
            spoken_languages.push(format!("understand {} but does not speak",understood_languages.and_join()));
        }

        if let Some(telepathy) = telepathy {
            spoken_languages.push(Language::Telepathy(*telepathy).to_string())
        }

        if spoken_languages.len() > 0 {
            Some(spoken_languages.join(", "))
        } else {
            None
        }
    }
    


    fn actions_to_stat_block(vec: &Vec<CreatureAction>, data: &Rc<Creature>) -> Result<Vec<StatBlockFeature>,InterpolationErrorDetails> {

        vec.iter().map(|a| {
            Ok(StatBlockFeature {
                text: CreatureFeature::feature_to_text_block(&a.name, &a.description, &a.usage_limit, data)?
            })
    
        }).collect()

    }

    fn features_to_stat_block(vec: &Vec<CreatureFeature>, data: &Rc<Creature>) -> Result<Vec<StatBlockFeature>,InterpolationErrorDetails> {

        vec.iter().map(|a| a.to_stat_block(data)).collect()

    }

    fn spellcasting_to_stat_block(spells: &Spellcasting, data: &Rc<Creature>) -> Result<StatBlockFeature,InterpolationErrorDetails> {
        Ok(StatBlockFeature {
            text: CreatureFeature::feature_to_text_block(Spellcasting::FEATURE_NAME, &spells.get_description(), &None, data)?
        })
    }

    fn innate_spellcasting_to_stat_block(spells: &InnateSpellcasting, data: &Rc<Creature>) -> Result<StatBlockFeature,InterpolationErrorDetails> {
        Ok(StatBlockFeature {
            text: CreatureFeature::feature_to_text_block(InnateSpellcasting::FEATURE_NAME, &spells.get_description(), &None, data)?
        })
    }

    fn special_abilities_to_stat_block(vec: &Vec<CreatureSpecialAbility>, data: &Rc<Creature>) -> Result<Vec<StatBlockFeature>,InterpolationErrorDetails> {

        vec.iter().map(|a| {
            match &a {
                CreatureSpecialAbility::Feature(feature) => {
                    feature.to_stat_block(data)
                },
                CreatureSpecialAbility::Spellcasting(spellcasting) => {
                    Self::spellcasting_to_stat_block(spellcasting,data)
                },
                CreatureSpecialAbility::InnateSpellcasting(spellcasting) => {
                    Self::innate_spellcasting_to_stat_block(spellcasting,data)
                }

            }
        }).collect()

    }

    fn get_armor_class(&self) -> u8 {
        self.armor.get_armor_class(Ability::score_to_mod(self.dexterity))
    }

    fn armor_to_stat_block(&self) -> String {
        let dex = Ability::score_to_mod(self.dexterity);
        let ac = self.armor.get_armor_class(dex);
        if let Some(description) = self.armor.get_description(dex) {
            format!("{} ({})",ac,description)
        } else {
            format!("{}",ac)
        }
    }


    pub fn set_source(&mut self, name: &str) {
        self.source = Some(name.to_owned())
    }
 
    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_owned()
    }

    pub fn set_subject(&mut self, name: &str) {
        self.subject = Some(name.to_owned())
    }

    pub fn set_capitalized_subject(&mut self, name: &str) {
        self.subject_cap = Some(name.to_owned())
    }

    pub fn set_possessive(&mut self, name: &str) {
        self.possessive = Some(name.to_owned())
    }

    pub fn set_capitalized_possessive(&mut self, name: &str) {
        self.possessive_cap = Some(name.to_owned())
    }

    pub fn set_subject_pronoun(&mut self, pronoun: &str) {
        self.subject_pronoun = Some(pronoun.to_owned())
    }
    
    pub fn set_subject_pronoun_cap(&mut self, pronoun: &str) {
        self.subject_pronoun_cap = Some(pronoun.to_owned())
    }
    
    pub fn set_possessive_pronoun(&mut self, pronoun: &str) {
        self.possessive_pronoun = Some(pronoun.to_owned())
    }
    
    pub fn set_possessive_pronoun_cap(&mut self, pronoun: &str) {
        self.possessive_pronoun_cap = Some(pronoun.to_owned())
    }
    
    pub fn set_object_pronoun(&mut self, pronoun: &str) {
        self.object_pronoun = Some(pronoun.to_owned())
    }

    pub fn set_reflexive_pronoun(&mut self, pronoun: &str) {
        self.reflexive_pronoun = Some(pronoun.to_owned())
    }
    
    pub fn set_tiny(&mut self) {
        self.size = CreatureSize::Tiny;
        self.hit_die = Die::D4;
    }

    pub fn set_small(&mut self) {
        self.size = CreatureSize::Small;
        self.hit_die = Die::D6;
    }

    pub fn set_medium(&mut self) {
        self.size = CreatureSize::Medium;
        self.hit_die = Die::D8;
    }

    pub fn set_large(&mut self) {
        self.size = CreatureSize::Large;
        self.hit_die = Die::D10;
    }

    pub fn set_huge(&mut self) {
        self.size = CreatureSize::Huge;
        self.hit_die = Die::D12;
    }

    pub fn set_gargantuan(&mut self) {
        self.size = CreatureSize::Gargantuan;
        self.hit_die = Die::D20;
    }

    pub fn set_aberration(&mut self) {
        self.type_ = CreatureType::Aberration
    }

    pub fn set_beast(&mut self) {
        self.type_ = CreatureType::Beast
    }

    pub fn set_celestial(&mut self) {
        self.type_ = CreatureType::Celestial
    }

    pub fn set_construct(&mut self) {
        self.type_ = CreatureType::Construct
    }

    pub fn set_dragon(&mut self) {
        self.type_ = CreatureType::Dragon
    }

    pub fn set_elemental(&mut self) {
        self.type_ = CreatureType::Elemental
    }

    pub fn set_fey(&mut self) {
        self.type_ = CreatureType::Fey
    }

    pub fn set_fiend(&mut self) {
        self.type_ = CreatureType::Fiend
    }

    pub fn set_giant(&mut self) {
        self.type_ = CreatureType::Giant
    }

    pub fn set_humanoid(&mut self) {
        self.type_ = CreatureType::Humanoid
    }

    pub fn set_monstrosity(&mut self) {
        self.type_ = CreatureType::Monstrosity
    }

    pub fn set_ooze(&mut self) {
        self.type_ = CreatureType::Ooze
    }

    pub fn set_plant(&mut self) {
        self.type_ = CreatureType::Plant
    }

    pub fn set_undead(&mut self) {
        self.type_ = CreatureType::Undead
    }

    pub fn set_custom_type(&mut self, name: &str) {
        self.type_ = CreatureType::Custom(name.to_owned())
    }

    pub fn set_subtype(&mut self, name: &str) {
        self.subtype = Some(name.to_owned())
    }

    pub fn set_group(&mut self, group: &str) {
        self.group = Some(group.to_owned())
    }

    pub fn set_any_alignment(&mut self) {
        self.alignment = Alignment::AnyAlignment
    }

    pub fn set_any_non_good(&mut self) {
        self.alignment = Alignment::AnyNonGood
    }

    pub fn set_any_non_evil(&mut self) {
        self.alignment = Alignment::AnyNonEvil
    }

    pub fn set_any_non_lawful(&mut self) {
        self.alignment = Alignment::AnyNonLawful
    }

    pub fn set_any_non_chaotic(&mut self) {
        self.alignment = Alignment::AnyNonChaotic
    }

    pub fn set_any_good(&mut self) {
        self.alignment = Alignment::AnyGood
    }

    pub fn set_any_evil(&mut self) {
        self.alignment = Alignment::AnyEvil
    }

    pub fn set_any_lawful(&mut self) {
        self.alignment = Alignment::AnyLawful
    }

    pub fn set_any_chaotic(&mut self) {
        self.alignment = Alignment::AnyChaotic
    }

    pub fn set_lawful_good(&mut self) {
        self.alignment = Alignment::LawfulGood
    }

    pub fn set_neutral_good(&mut self) {
        self.alignment = Alignment::NeutralGood
    }

    pub fn set_chaotic_good(&mut self) {
        self.alignment = Alignment::ChaoticGood
    }

    pub fn set_lawful_neutral(&mut self) {
        self.alignment = Alignment::LawfulNeutral
    }

    pub fn set_neutral(&mut self) {
        self.alignment = Alignment::Neutral
    }

    pub fn set_chaotic_neutral(&mut self) {
        self.alignment = Alignment::ChaoticNeutral
    }

    pub fn set_lawful_evil(&mut self) {
        self.alignment = Alignment::LawfulEvil
    }

    pub fn set_neutral_evil(&mut self) {
        self.alignment = Alignment::NeutralEvil
    }

    pub fn set_chaotic_evil(&mut self) {
        self.alignment = Alignment::ChaoticEvil
    }

    pub fn set_unaligned(&mut self) {
        self.alignment = Alignment::Unaligned
    }

    pub fn set_custom_alignment(&mut self, name: &str) {
        self.alignment = Alignment::Custom(name.to_owned())
    }

    pub fn set_hit_die(&mut self, die: &Die) {
        self.hit_die = die.clone()
    }

    pub fn set_hit_dice_count(&mut self, count: &u8) {
        self.hit_dice_count = *count;
    }

    pub fn set_hit_points_override(&mut self, points: &u16) {
        self.override_hit_points = Some(*points)
    }

    pub fn set_armor(&mut self, armor: &Armor) {
        self.armor.armor = Some(armor.clone())
    }

    pub fn enable_shield(&mut self) {
        self.armor.shield = true
    }

    pub fn disable_shield(&mut self) {
        self.armor.shield = false
    }

    pub fn walk(&mut self, speed: &u8) {
        self.speed.walk = Some(*speed)
    }

    pub fn swim(&mut self, speed: &u8) {
        self.speed.swim = if *speed == 0 {
            None
        } else {
            Some(*speed)
        }
    }

    pub fn fly(&mut self, speed: &u8) {
        self.speed.fly = if *speed == 0 {
            self.speed.hover = false;
            None
        } else {
            Some(*speed)
        }
    }

    pub fn burrow(&mut self, speed: &u8) {
        self.speed.burrow = if *speed == 0 {
            None
        } else {
            Some(*speed)
        }
    }

    pub fn climb(&mut self, speed: &u8) {
        self.speed.climb = if *speed == 0 {
            None
        } else {
            Some(*speed)
        }
    }

    pub fn enable_hover(&mut self)  {
        self.speed.hover = true
    }

    pub fn speed_notes(&mut self, notes: &str) {
        self.speed.notes = if notes == "" {
            None
        } else {
            Some(notes.to_owned())
        }
    }

    pub fn custom_speed(&mut self, movement: &str, speed: &u8) {
        if *speed == 0 {
            self.speed.custom.remove(movement);
        } else {
            self.speed.custom.insert(movement.to_owned(), *speed);
        }

    }

    pub fn set_str(&mut self, score: &u8) {
        self.strength = *score
    }

    pub fn set_dex(&mut self, score: &u8) {
        self.dexterity = *score
    }

    pub fn set_con(&mut self, score: &u8) {
        self.constitution = *score
    }

    pub fn set_int(&mut self, score: &u8) {
        self.intelligence = *score
    }

    pub fn set_wis(&mut self, score: &u8) {
        self.wisdom = *score
    }

    pub fn set_cha(&mut self, score: &u8) {
        self.charisma = *score
    }

    pub fn add_saves(&mut self, abilities: &[Ability]) {
        for ability in abilities {
            match ability {
                Ability::Strength => self.strength_save = true,
                Ability::Dexterity => self.dexterity_save = true,
                Ability::Constitution => self.constitution_save = true,
                Ability::Intelligence => self.intelligence_save = true,
                Ability::Wisdom => self.wisdom_save = true,
                Ability::Charisma => self.charisma_save = true,
            }
        }
    }

    pub fn add_skills(&mut self, skills: &[Skill]) {
        for skill in skills {
            match skill {
                Skill::Athletics => self.skills.athletics = Some(false),
                Skill::Acrobatics => self.skills.acrobatics = Some(false),
                Skill::SleightOfHand => self.skills.sleight_of_hand = Some(false),
                Skill::Stealth => self.skills.stealth = Some(false),
                Skill::Arcana => self.skills.arcana = Some(false),
                Skill::History => self.skills.history = Some(false),
                Skill::Investigation => self.skills.investigation = Some(false),
                Skill::Nature => self.skills.nature = Some(false),
                Skill::Religion => self.skills.religion = Some(false),
                Skill::AnimalHandling => self.skills.animal_handling = Some(false),
                Skill::Insight => self.skills.insight = Some(false),
                Skill::Medicine => self.skills.medicine = Some(false),
                Skill::Perception => self.skills.perception = Some(false),
                Skill::Survival => self.skills.survival = Some(false),
                Skill::Deception => self.skills.deception = Some(false),
                Skill::Intimidation => self.skills.intimidation = Some(false),
                Skill::Performance => self.skills.performance = Some(false),
                Skill::Persuasion => self.skills.persuasion = Some(false),
            }
        }
    }

    pub fn add_expertise(&mut self, skills: &[Skill]) {
        for skill in skills {
            match skill {
                Skill::Athletics => self.skills.athletics = Some(true),
                Skill::Acrobatics => self.skills.acrobatics = Some(true),
                Skill::SleightOfHand => self.skills.sleight_of_hand = Some(true),
                Skill::Stealth => self.skills.stealth = Some(true),
                Skill::Arcana => self.skills.arcana = Some(true),
                Skill::History => self.skills.history = Some(true),
                Skill::Investigation => self.skills.investigation = Some(true),
                Skill::Nature => self.skills.nature = Some(true),
                Skill::Religion => self.skills.religion = Some(true),
                Skill::AnimalHandling => self.skills.animal_handling = Some(true),
                Skill::Insight => self.skills.insight = Some(true),
                Skill::Medicine => self.skills.medicine = Some(true),
                Skill::Perception => self.skills.perception = Some(true),
                Skill::Survival => self.skills.survival = Some(true),
                Skill::Deception => self.skills.deception = Some(true),
                Skill::Intimidation => self.skills.intimidation = Some(true),
                Skill::Performance => self.skills.performance = Some(true),
                Skill::Persuasion => self.skills.persuasion = Some(true),
            }
        }
    }

    pub fn remove_saves(&mut self, abilities: &[Ability]) {
        for ability in abilities {
            match ability {
                Ability::Strength => self.strength_save = false,
                Ability::Dexterity => self.dexterity_save = false,
                Ability::Constitution => self.constitution_save = false,
                Ability::Intelligence => self.intelligence_save = false,
                Ability::Wisdom => self.wisdom_save = false,
                Ability::Charisma => self.charisma_save = false,
            }
        }
    }

    pub fn remove_skills(&mut self, skills: &[Skill]) {
        for skill in skills {
            match skill {
                Skill::Athletics => self.skills.athletics = None,
                Skill::Acrobatics => self.skills.acrobatics = None,
                Skill::SleightOfHand => self.skills.sleight_of_hand = None,
                Skill::Stealth => self.skills.stealth = None,
                Skill::Arcana => self.skills.arcana = None,
                Skill::History => self.skills.history = None,
                Skill::Investigation => self.skills.investigation = None,
                Skill::Nature => self.skills.nature = None,
                Skill::Religion => self.skills.religion = None,
                Skill::AnimalHandling => self.skills.animal_handling = None,
                Skill::Insight => self.skills.insight = None,
                Skill::Medicine => self.skills.medicine = None,
                Skill::Perception => self.skills.perception = None,
                Skill::Survival => self.skills.survival = None,
                Skill::Deception => self.skills.deception = None,
                Skill::Intimidation => self.skills.intimidation = None,
                Skill::Performance => self.skills.performance = None,
                Skill::Persuasion => self.skills.persuasion = None,
            }
        }

    }

    pub fn add_condition_immunity(&mut self, condition: &Condition){
        match condition {
            Condition::Blinded => self.condition_immunities.blinded = true,
            Condition::Charmed => self.condition_immunities.charmed = true,
            Condition::Deafened => self.condition_immunities.deafened = true,
            Condition::Exhaustion => self.condition_immunities.exhaustion = true,
            Condition::Frightened => self.condition_immunities.frightened = true,
            Condition::Grappled => self.condition_immunities.grappled = true,
            Condition::Incapacitated => self.condition_immunities.incapacitated = true,
            Condition::Invisible => self.condition_immunities.invisible = true,
            Condition::Paralyzed => self.condition_immunities.paralyzed = true,
            Condition::Petrified => self.condition_immunities.petrified = true,
            Condition::Poisoned => self.condition_immunities.poisoned = true,
            Condition::Prone => self.condition_immunities.prone = true,
            Condition::Restrained => self.condition_immunities.restrained = true,
            Condition::Stunned => self.condition_immunities.stunned = true,
            Condition::Unconscious => self.condition_immunities.unconscious = true,
        
        }
    }

    pub fn add_vulnerability(&mut self, damage: &Damage) {
        match damage {
            Damage::Bludgeoning => self.vulnerabilities.bludgeoning = true,
            Damage::Piercing => self.vulnerabilities.piercing = true,
            Damage::Slashing => self.vulnerabilities.slashing = true,
            Damage::Cold => self.vulnerabilities.cold = true,
            Damage::Fire => self.vulnerabilities.fire = true,
            Damage::Thunder => self.vulnerabilities.thunder = true,
            Damage::Force => self.vulnerabilities.force = true,
            Damage::Radiant => self.vulnerabilities.radiant = true,
            Damage::Lightning => self.vulnerabilities.lightning = true,
            Damage::Poison => self.vulnerabilities.poison = true,
            Damage::Acid => self.vulnerabilities.acid = true,
            Damage::Necrotic => self.vulnerabilities.necrotic = true,
            Damage::Psychic => self.vulnerabilities.psychic = true,                
        }

    }

    pub fn add_all_vulnerability(&mut self) {
        self.vulnerabilities.all = true;
    }

    pub fn add_custom_vulnerability(&mut self, name: &str) {
        self.vulnerabilities.custom = Some(name.to_owned())
    }

    pub fn remove_vulnerability(&mut self, damage: &Damage) {
        // also turn off all vulnerabilitys if possible.
        self.vulnerabilities.all = false;
        match damage {
            Damage::Bludgeoning => self.vulnerabilities.bludgeoning = false,
            Damage::Piercing => self.vulnerabilities.piercing = false,
            Damage::Slashing => self.vulnerabilities.slashing = false,
            Damage::Cold => self.vulnerabilities.cold = false,
            Damage::Fire => self.vulnerabilities.fire = false,
            Damage::Thunder => self.vulnerabilities.thunder = false,
            Damage::Radiant => self.vulnerabilities.radiant = false,
            Damage::Force => self.vulnerabilities.force = false,
            Damage::Lightning => self.vulnerabilities.lightning = false,
            Damage::Poison => self.vulnerabilities.poison = false,
            Damage::Acid => self.vulnerabilities.acid = false,
            Damage::Necrotic => self.vulnerabilities.necrotic = false,
            Damage::Psychic => self.vulnerabilities.psychic = false 
        }
    }

    pub fn add_resistance(&mut self, damage: &Damage) {
        match damage {
            Damage::Bludgeoning => self.resistances.bludgeoning = true,
            Damage::Piercing => self.resistances.piercing = true,
            Damage::Slashing => self.resistances.slashing = true,
            Damage::Cold => self.resistances.cold = true,
            Damage::Fire => self.resistances.fire = true,
            Damage::Thunder => self.resistances.thunder = true,
            Damage::Radiant => self.resistances.radiant = true,
            Damage::Force => self.resistances.force = true,
            Damage::Lightning => self.resistances.lightning = true,
            Damage::Poison => self.resistances.poison = true,
            Damage::Acid => self.resistances.acid = true,
            Damage::Necrotic => self.resistances.necrotic = true,
            Damage::Psychic => self.resistances.psychic = true,                
        }

    }

    pub fn add_all_resistance(&mut self) {
        self.resistances.all = true;
    }

    pub fn add_custom_resistance(&mut self, name: &str) {
        self.resistances.custom = Some(name.to_owned())
    }

    pub fn add_nonmagical_resistance(&mut self) {
        self.resistances.non_magical_attacks = true;
    }

    pub fn add_nonsilvered_resistance(&mut self) {
        self.resistances.non_silvered_attacks = true;
    }

    pub fn add_nonadamantine_resistance(&mut self) {
        self.resistances.non_adamantine_attacks = true;
    }

    pub fn remove_resistance(&mut self, damage: &Damage) {
        // also turn off all vulnerabilitys if possible.
        self.resistances.all = false;
        match damage {
            Damage::Bludgeoning => self.resistances.bludgeoning = false,
            Damage::Piercing => self.resistances.piercing = false,
            Damage::Slashing => self.resistances.slashing = false,
            Damage::Cold => self.resistances.cold = false,
            Damage::Fire => self.resistances.fire = false,
            Damage::Thunder => self.resistances.thunder = false,
            Damage::Radiant => self.resistances.radiant = false,
            Damage::Force => self.resistances.force = false,
            Damage::Lightning => self.resistances.lightning = false,
            Damage::Poison => self.resistances.poison = false,
            Damage::Acid => self.resistances.acid = false,
            Damage::Necrotic => self.resistances.necrotic = false,
            Damage::Psychic => self.resistances.psychic = false 
        }
    }

    pub fn remove_special_resistance(&mut self) {
        self.resistances.non_magical_attacks = false;
        self.resistances.non_silvered_attacks = false;
        self.resistances.non_adamantine_attacks = false;
        self.resistances.custom = None;
        self.resistances.all = false;
    }

    pub fn add_immunity(&mut self, damage: &Damage) {
        match damage {
            Damage::Bludgeoning => self.immunities.bludgeoning = true,
            Damage::Piercing => self.immunities.piercing = true,
            Damage::Slashing => self.immunities.slashing = true,
            Damage::Cold => self.immunities.cold = true,
            Damage::Fire => self.immunities.fire = true,
            Damage::Thunder => self.immunities.thunder = true,
            Damage::Radiant => self.immunities.radiant = true,
            Damage::Force => self.immunities.force = true,
            Damage::Lightning => self.immunities.lightning = true,
            Damage::Poison => self.immunities.poison = true,
            Damage::Acid => self.immunities.acid = true,
            Damage::Necrotic => self.immunities.necrotic = true,
            Damage::Psychic => self.immunities.psychic = true,                
        }

    }

    pub fn add_all_immunities(&mut self) {
        self.immunities.all = true;
    }

    pub fn add_custom_immunity(&mut self, name: &str) {
        self.immunities.custom = Some(name.to_owned())
    }

    pub fn add_nonmagical_immunity(&mut self) {
        self.immunities.non_magical_attacks = true;
    }

    pub fn add_nonsilvered_immunity(&mut self) {
        self.immunities.non_silvered_attacks = true;
    }

    pub fn add_nonadamantine_immunity(&mut self) {
        self.immunities.non_adamantine_attacks = true;
    }    

    pub fn remove_immunity(&mut self, damage: &Damage) {
        // also turn off all vulnerabilitys if possible.
        self.immunities.all = false;
        match damage {
            Damage::Bludgeoning => self.immunities.bludgeoning = false,
            Damage::Piercing => self.immunities.piercing = false,
            Damage::Slashing => self.immunities.slashing = false,
            Damage::Cold => self.immunities.cold = false,
            Damage::Fire => self.immunities.fire = false,
            Damage::Thunder => self.immunities.thunder = false,
            Damage::Radiant => self.immunities.radiant = false,
            Damage::Force => self.immunities.force = false,
            Damage::Lightning => self.immunities.lightning = false,
            Damage::Poison => self.immunities.poison = false,
            Damage::Acid => self.immunities.acid = false,
            Damage::Necrotic => self.immunities.necrotic = false,
            Damage::Psychic => self.immunities.psychic = false 
        }
    }

    pub fn remove_special_immunity(&mut self) {
        self.immunities.non_magical_attacks = false;
        self.immunities.non_silvered_attacks = false;
        self.immunities.non_adamantine_attacks = false;
        self.immunities.custom = None;
        self.immunities.all = false;
    }

    pub fn set_languages(&mut self, languages: &[Language]) {
        self.languages.clear();
        for language in languages {
            self.languages.push((language.clone(), true));
        }

    }

    pub fn add_unspoken_languages(&mut self, languages: &[Language]) {
        for language in languages {
            self.languages.push((language.clone(), false));
        }

    }

    pub fn add_darkvision(&mut self, distance: &u8) {
        self.senses.darkvision = if *distance == 0 {
            None
        } else {
            Some(*distance)
        }
    }

    pub fn add_blindsight(&mut self, distance: &u8) {
        self.senses.blindsight = if *distance == 0 {
            None
        } else {
            Some((*distance,false))
        }
    }

    pub fn add_blindsight_blind_beyond(&mut self, distance: &u8) {
        self.senses.blindsight = if *distance == 0 {
            None
        } else {
            Some((*distance,true))
        }
    }

    pub fn add_truesight(&mut self, distance: &u8) {
        self.senses.truesight = if *distance == 0 {
            None
        } else {
            Some(*distance)
        }
    }

    pub fn add_tremorsense(&mut self, distance: &u8) {
        self.senses.tremorsense = if *distance == 0 {
            None
        } else {
            Some(*distance)
        }
    }

    pub fn add_custom_sense(&mut self, sense: &str, distance: &u8) {
        if *distance == 0 {
            self.senses.custom.remove(sense);
        } else {
            self.senses.custom.insert(sense.to_owned(),*distance);
        }
    }

    pub fn set_no_challenge_rating(&mut self) {
        self.challenge_rating = ChallengeRating::None
    }

    pub fn set_challenge_rating(&mut self, cr: &u8) {
        self.challenge_rating = ChallengeRating::Whole(*cr)
    }

    pub fn set_half_challenge_rating(&mut self) {
        self.challenge_rating = ChallengeRating::Half
    }

    pub fn set_quarter_challenge_rating(&mut self) {
        self.challenge_rating = ChallengeRating::Quarter
    }

    pub fn set_eighth_challenge_rating(&mut self) {
        self.challenge_rating = ChallengeRating::Eighth
    }

    fn find_action(&self, name: &str) -> Option<&CreatureAction> {
        for action in &self.actions {
            if action.name == name {
                return Some(action)
            }
        }
        None
    }

    fn find_weapon(&self, weapon: &Weapon) -> Option<&CreatureAction> {
        self.find_action(&weapon.to_string())
    }

    pub fn find_action_mut(&mut self, name: &str) -> Option<&mut CreatureAction> {
        for action in &mut self.actions {
            if action.name == name {
                return Some(action)
            }
        }
        None
    }

    fn find_weapon_mut(&mut self, weapon: &Weapon) -> Option<&mut CreatureAction> {
        self.find_action_mut(&weapon.to_string())
    }

    pub fn check_multiattack(&self, details: &Multiattack) -> Result<(),CreatureError> {
        match details {
            Multiattack::Any |
            Multiattack::Ranged |
            Multiattack::Melee |
            Multiattack::Spell => Ok(()), // any spell attack, or a cast spell action
            Multiattack::Attack(name) => if self.find_action(name).is_none() {
                Err(CreatureError::ActionNotFound(name.to_owned(),"while verifying multiattack action".to_owned()))
            } else {
                Ok(())
            }, 
            Multiattack::Weapon(name) => if self.find_weapon(name).is_none() {
                Err(CreatureError::WeaponNotFound(name.to_string(),"while verifying multiattack action".to_owned()))
            } else {
                Ok(())
            }, 
            Multiattack::Except(list) |
            Multiattack::Or(list) |
            Multiattack::Count(_,list) |
            Multiattack::And(list) | 
            Multiattack::Dice(_,list) => {
                for multiattack in list {
                    self.check_multiattack(multiattack)?
                }
                Ok(())
            }
        }
    }

    pub fn set_multiattack(&mut self, description: String, details: &Multiattack) {
        self.remove_action("Multiattack");
        if description.len() > 0 {
            self.multiattack = Some(details.clone());
            self.actions.push(CreatureAction {
                name: "Multiattack".to_owned(),
                description: description,
                attack: None,
                effect: None,
                compound: None,
                usage_limit: None
            })            
        } else {
            self.multiattack = None
        }

    }

    pub fn add_weapon(&mut self, weapon: &Weapon, compound: &Option<CompoundAttackEffect>) {
        let name = weapon.to_string();
        let attack = weapon.get_attack();
        let effect = weapon.get_effect(&self.size);
        let description = attack.get_description(Some(&effect), compound);
        self.actions.push(CreatureAction {
            name,
            description,
            attack: Some(attack),
            effect: Some(effect),
            compound: compound.clone(),
            usage_limit: None
        })            
    }

    pub fn expect_weapon_attack(&self, weapon: &Weapon, attack: &Attack) -> Result<(),CreatureError> {
        if let Some(action) = self.find_weapon(weapon) {
            if let Some(actual_attack) = &action.attack {
                if attack != actual_attack {
                    Err(CreatureError::WeaponAttackDoesNotMatchExpectation(weapon.to_string()))?
                } else {
                    return Ok(())
                }
            } 
        }
        Err(CreatureError::WeaponNotFound(weapon.to_string(),"while expecting a weapon attack to match".to_owned()))
    }

    pub fn expect_weapon_effect(&self, weapon: &Weapon, effect: &AttackEffect) -> Result<(),CreatureError> {
        if let Some(action) = self.find_weapon(weapon) {
            if let Some(actual_effect) = &action.effect {
                if effect != actual_effect {
                    Err(CreatureError::WeaponEffectDoesNotMatchExpectation(weapon.to_string()))?
                } else {
                    return Ok(())
                }
            }
        }
        Err(CreatureError::WeaponNotFound(weapon.to_string(),"while expecting a weapon effect to match".to_owned()))?
    }

    pub fn override_weapon_attack(&mut self, weapon: &Weapon, attack: Attack) -> Result<(),CreatureError> {
        if let Some(action) = self.find_weapon_mut(weapon) {
            // override the description as well. Note that this will overwrite any override_weapon_description stuff, but the user can easily fix that.
            action.description = attack.get_description(action.effect.as_ref(), &action.compound);
            action.attack = Some(attack);
            Ok(())
        } else {
            Err(CreatureError::WeaponNotFound(weapon.to_string(),"while overriding a weapon attack".to_owned()))
        }
    }

    pub fn override_weapon_effect(&mut self, weapon: &Weapon, effect: AttackEffect) -> Result<(),CreatureError> {
        if let Some(action) = self.find_weapon_mut(weapon) {
            // override the description as well. Note that this will overwrite any override_weapon_description stuff, but the user can easily fix that.
            if let Some(attack) = &action.attack {
                action.description = attack.get_description(Some(&effect), &action.compound);
            } else {
                action.description = effect.get_description("0", &action.compound)
            }
            action.effect = Some(effect);
            Ok(())
        } else {
            Err(CreatureError::WeaponNotFound(weapon.to_string(),"while overriding a weapon effect".to_owned()))
        }
    }

    pub fn override_weapon_description(&mut self, weapon: &Weapon, description: String) -> Result<(),CreatureError> {
        if let Some(action) = self.find_weapon_mut(weapon) {
            action.description = description;
            Ok(())
        } else {
            Err(CreatureError::WeaponNotFound(weapon.to_string(),"while overriding a weapon description".to_owned()))
        }
    }

    pub fn add_action(&mut self, action: &Action, usage_limit: &Option<UsageLimit>) {
        self.actions.push(CreatureAction::new_from_action(action, usage_limit))
    }

    pub fn override_action_description(&mut self, name: &str, description: String) -> Result<(),CreatureError> {
        if let Some(action) = self.find_action_mut(&name) {
            action.description = description;
            Ok(())
        } else {
            Err(CreatureError::ActionNotFound(name.to_owned(),"while overriding an action description".to_owned()))
        }
    }

    pub fn remove_weapon(&mut self, weapon: &Weapon) {
        let name = weapon.to_string();
        self.remove_action(&name)
    }

    pub fn move_weapon(&mut self, weapon: &Weapon, delta: &i8) -> Result<(),CreatureError> {
        let name = weapon.to_string();
        self.move_action(&name,delta)
    }

    pub fn remove_action(&mut self, name: &str) {
        let mut found = false;
        self.actions.retain(|a| 
            if !found && (a.name == name) {
                found = true;
                false
            } else {
                true
            }
        )
    }

    pub fn move_action(&mut self, name: &str, delta: &i8) -> Result<(),CreatureError> {
        if let Some(index) = self.actions.iter().position(|a| a.name == name) {
            let item = self.actions.remove(index);
            let delta = *delta;
            let new_index = if delta > 0 {
                index + delta as usize
            } else {
                index - delta.abs() as usize
            };
            self.actions.insert(new_index, item);
            Ok(())
        } else {
            Err(CreatureError::ActionNotFound(name.to_owned(),"while moving action".to_owned()))
        }
    }

    pub fn add_reaction(&mut self, reaction: Reaction, usage_limit: Option<UsageLimit>) {
        self.reactions.push(CreatureFeature {
            name: reaction.get_name(),
            description: reaction.get_description(),
            usage_limit: usage_limit.clone()
        })

    }

    pub fn remove_reaction(&mut self, name: &str) {
        let mut found = false;
        self.reactions.retain(|a| 
            if !found && (a.name == name) {
                found = true;
                false
            } else {
                true
            }
        )
    }

    pub fn add_feature(&mut self, feature: Feature, usage_limit: Option<UsageLimit>) {
        self.special_abilities.push(CreatureSpecialAbility::Feature(CreatureFeature {
            name: feature.get_name(),
            description: feature.get_description(),
            usage_limit: usage_limit.clone()
        }));

    }

    pub fn remove_feature(&mut self, name: &str) {
        let mut found = false;
        self.special_abilities.retain(|a| 
            if found {
                // only remove the first one...
                true
            } else {
                if match a {
                    CreatureSpecialAbility::Feature(feature) => feature.name == name,
                    CreatureSpecialAbility::Spellcasting(_) => name == Spellcasting::FEATURE_NAME,
                    CreatureSpecialAbility::InnateSpellcasting(_) => name == InnateSpellcasting::FEATURE_NAME
                } {
                    found = true;
                    false
                } else {
                    true
                }
            }
        )
    }

    pub fn remove_legendary_action(&mut self, name: &str) {
        if let Some(legendary) = &mut self.legendary_actions {
            let mut found = false;
            legendary.actions.retain(|a| 
                if !found && (a.name == name) {
                    found = true;
                    false
                } else {
                    true
                }
            )
    
        }

    }

    pub fn get_spellcasting(&self) -> Option<&Spellcasting> {
        for feature in &self.special_abilities {
            if let CreatureSpecialAbility::Spellcasting(spellcasting) = feature {
                return Some(&spellcasting)
            }
        }
        None
    }

    pub fn get_or_add_spellcasting_mut(&mut self) -> &mut Spellcasting {
        let index = self.special_abilities.iter().position(|a| if let CreatureSpecialAbility::Spellcasting(_) = a {
            true
        } else {
            false
        });
        let index = if let Some(index) = index {
            index
        } else {
            self.special_abilities.push(CreatureSpecialAbility::Spellcasting(Spellcasting::default()));
            self.special_abilities.len() - 1
        };
        if let CreatureSpecialAbility::Spellcasting(spellcasting) = self.special_abilities.get_mut(index).unwrap() {
            spellcasting
        } else {
            panic!("Not spellcasting")
        }

    }

    pub fn get_innate_spellcasting(&self) -> Option<&InnateSpellcasting> {
        for feature in &self.special_abilities {
            if let CreatureSpecialAbility::InnateSpellcasting(spellcasting) = feature {
                return Some(&spellcasting)
            }
        }
        None
    }

    pub fn get_or_add_innate_spellcasting_mut(&mut self) -> &mut InnateSpellcasting {
        let index = self.special_abilities.iter().position(|a| if let CreatureSpecialAbility::InnateSpellcasting(_) = a {
            true
        } else {
            false
        });
        let index = if let Some(index) = index {
            index
        } else {
            self.special_abilities.push(CreatureSpecialAbility::InnateSpellcasting(InnateSpellcasting::default()));
            self.special_abilities.len() - 1
        };
        if let CreatureSpecialAbility::InnateSpellcasting(spellcasting) = self.special_abilities.get_mut(index).unwrap() {
            spellcasting
        } else {
            panic!("Not spellcasting")
        }
    }

    pub fn get_spell_attack_bonus(&self) -> Option<i8> {
        // prefer innate spellcasting if there's a choice
        let bonus_ability = if let Some(spellcasting) = self.get_innate_spellcasting() {
            Some((spellcasting.attack_bonus,&spellcasting.ability))
        } else if let Some(spellcasting) = self.get_spellcasting() {
            Some((spellcasting.attack_bonus,&spellcasting.ability))
        } else {
            // There is no spell attack bonus
            None
        };
        match bonus_ability {
            Some((Some(num),_)) => Some(num),
            Some((None,ability)) => Some(match ability {
                Ability::Strength => Ability::score_to_mod(self.strength),
                Ability::Dexterity => Ability::score_to_mod(self.dexterity),
                Ability::Constitution => Ability::score_to_mod(self.constitution),
                Ability::Intelligence => Ability::score_to_mod(self.intelligence),
                Ability::Wisdom => Ability::score_to_mod(self.wisdom),
                Ability::Charisma => Ability::score_to_mod(self.charisma),
            }),
            None => None
        }
    }

    pub fn set_legendary_actions(&mut self, description: String, actions: Vec<CreatureLegendaryAction>) {
        self.legendary_actions = Some(CreatureLegendaryActions {
            description,
            actions
        })

    }


}

impl TryFrom<Creature> for CreatureStatBlock {

    type Error = InterpolationErrorDetails;

    fn try_from(creature: Creature) -> Result<Self,Self::Error> {

        let me = Rc::new(creature);

        let armor = me.armor_to_stat_block();
        let saving_throws = Creature::saving_throws_to_stat_block(&me);
        let (skills,perception) = me.skills.to_stat_block(&me);
        let senses = me.senses.to_stat_block(perception,me.wisdom);
        let actions = Creature::actions_to_stat_block(&me.actions,&me)?;
        let reactions = Creature::features_to_stat_block(&me.reactions,&me)?;
        let special_abilities = Creature::special_abilities_to_stat_block(&me.special_abilities,&me)?;
        let legendary_actions = if let Some(legendary_actions) = &me.legendary_actions {
            Some(CreatureLegendaryActions::to_stat_block(legendary_actions,&me)?)
        } else {
            None
        };

        Ok(CreatureStatBlock {
            name: me.name.clone(),
            size: me.size.to_string(),
            type_: me.type_.to_string(),
            subtype: me.subtype.clone(),
            group: me.group.clone(),
            alignment: me.alignment.to_string(),
            hit_points: me.get_hit_dice().display_with_alternate_average(me.override_hit_points.map(|a| a as isize)),
            armor, 
            speed: me.speed.to_stat_block(),
            strength: Ability::to_stat_block(me.strength), // format!("{} ({})",self.strength,Ability::score_to_mod(self.strength)),
            dexterity: Ability::to_stat_block(me.dexterity),
            constitution: Ability::to_stat_block(me.constitution),
            intelligence: Ability::to_stat_block(me.intelligence),
            wisdom: Ability::to_stat_block(me.wisdom),
            charisma: Ability::to_stat_block(me.charisma),
            saving_throws,
            skills,
            damage_vulnerabilities: me.vulnerabilities.to_stat_block(),
            damage_resistances: me.resistances.to_stat_block(),
            damage_immunities: me.immunities.to_stat_block(),
            condition_immunities: me.condition_immunities.to_stat_block(),
            senses,
            languages: Creature::languages_to_stat_block(&me.languages),
            challenge_rating: me.challenge_rating.display_with_xp(),
            actions,
            reactions,
            legendary_actions,
            special_abilities,
            // FUTURE: Not currently supported
            lair_actions: None,
            regional_effects: None,
            source: me.source.clone()
        })
    }

}