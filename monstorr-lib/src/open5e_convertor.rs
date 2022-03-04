/*
 * Copyright © 2022 Neil M. Sheldon
 * 
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::convert::TryFrom;

use monstorr_open5e::Open5eMonster;
use monstorr_open5e::Open5eMonsterAction;

use crate::stat_block::CreatureStatBlock;
use crate::stat_block::StatBlockFeature;
use crate::stat_block::StatBlockLegendary;
use crate::stats::Ability;
use crate::interpolation::interpolate_simple_markdown_naively;
use crate::errors::InterpolationErrorDetails;

fn str_to_option(source: String) -> Option<String> {
    if source.is_empty() {
        None
    } else {
        Some(source)
    }
}

fn armor_to_stat_block(ac: u8, desc: Option<String>) -> String {
    if let Some(desc) = desc {
        format!("{} ({})",ac,desc)
    } else {
        format!("{}",ac)
    }

}

fn saving_throws_to_stat_block(strength: Option<i8>, dexterity: Option<i8>, constitution: Option<i8>, intelligence: Option<i8>, wisdom: Option<i8>, charisma: Option<i8>) -> Option<String> {
    let mut result = vec![];

    macro_rules! add_saving_throw {
        ($ability: ident, $abbrev: literal) => {
            if let Some(save) = $ability {
                result.push(format!(concat!($abbrev," {:+}"),save))
            }
                    
        };
    }

    add_saving_throw!(strength,"Str");
    add_saving_throw!(dexterity,"Dex");
    add_saving_throw!(constitution,"Con");
    add_saving_throw!(intelligence,"Int");
    add_saving_throw!(wisdom,"Wis");
    add_saving_throw!(charisma,"Cha");

    if result.len() > 0 {
        Some(result.join(", ")) 
    } else {
        None
    }
        


}

fn actions_to_stat_block(source: Vec<Open5eMonsterAction>) -> Result<Vec<StatBlockFeature>,InterpolationErrorDetails> {

    source.iter().map(|a| Ok(StatBlockFeature {
        text: interpolate_simple_markdown_naively(&a.name, &a.desc, &a.name, false, true)?
    })).collect()
}

fn legendary_to_stat_block(description: String, actions: Vec<Open5eMonsterAction>) -> Result<Option<StatBlockLegendary>,InterpolationErrorDetails> {
    Ok(if description.is_empty() && actions.is_empty() {
        None
    } else {
        Some(StatBlockLegendary {
            description: interpolate_simple_markdown_naively("", &description, "legendary description", false, true)?,
            // this isn't exactly the same as actions_to_statblock, since we want subparagraphs
            actions: actions.iter().map(|a| Ok(StatBlockFeature {
                text: interpolate_simple_markdown_naively(&a.name, &a.desc, "legendary action", true, true)?
            })).collect::<Result<Vec<StatBlockFeature>,InterpolationErrorDetails>>()?
        })    
    })
}

impl TryFrom<Open5eMonster> for CreatureStatBlock {

    type Error = String;

    fn try_from(creature: Open5eMonster) -> Result<Self,Self::Error> {
        Ok(Self {
            name: creature.name,
            size: creature.size,
            type_: creature.type_,
            subtype: creature.subtype,
            group: creature.group,
            alignment: creature.alignment,
            armor: armor_to_stat_block(creature.armor_class,creature.armor_desc),
            hit_points: format!("{} ({})",creature.hit_points,creature.hit_dice),
            speed: creature.speed.to_string(),
            strength: Ability::to_stat_block(creature.strength),
            dexterity: Ability::to_stat_block(creature.dexterity),
            constitution: Ability::to_stat_block(creature.constitution),
            intelligence: Ability::to_stat_block(creature.intelligence),
            wisdom: Ability::to_stat_block(creature.wisdom),
            charisma: Ability::to_stat_block(creature.charisma),
            saving_throws: saving_throws_to_stat_block(    
                creature.strength_save,
                creature.dexterity_save,
                creature.constitution_save,
                creature.intelligence_save,
                creature.wisdom_save,
                creature.charisma_save
            ),
            skills: creature.skills.to_string(),
            damage_vulnerabilities: str_to_option(creature.damage_vulnerabilities),
            damage_resistances: str_to_option(creature.damage_resistances),
            damage_immunities: str_to_option(creature.damage_immunities),
            condition_immunities: str_to_option(creature.condition_immunities),
            senses: creature.senses,
            languages: str_to_option(creature.languages),
            challenge_rating: creature.challenge_rating,
            actions: actions_to_stat_block(creature.actions).map_err(|e| format!("{}",e))?,
            reactions: actions_to_stat_block(creature.reactions).map_err(|e| format!("{}",e))?,
            legendary_actions: legendary_to_stat_block(creature.legendary_desc,creature.legendary_actions).map_err(|e| format!("{}",e))?,
            special_abilities: actions_to_stat_block(creature.special_abilities).map_err(|e| format!("{}",e))?,
            lair_actions: None,
            regional_effects: None,
            source: str_to_option(creature.document_title)
        })

    }
}