/*
 * Copyright © 2022 Neil M. Sheldon
 * 
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

/*!
Creature stat blocks are a simple property/value mapping, with no functionality. They are intended to be output directly as JSON, or used in a template to generate content for publication.

All property values are strings or lists of strings. Numbers and other data have been transformed into the standard formatting used in the core books. For example, abilities show their score and the modifier in parentheses, skills are already joined into a list with their bonus numbers, etc. Most templates should be able to simply output the property values as-is. The only complexity lay in the descriptions, which contain structured text.

The only exception is the 'languages' property, which does not display an em-dash if it is not present.

The schema for the JSON format is defined on the [`CreatureStatBlock`] struct.
*/

use std::convert::TryInto;

use serde::Deserialize;
use serde::Serialize;

use crate::structured_text::TextBlock;

#[derive(PartialEq,Debug)]
#[derive(Serialize,Deserialize)]
#[serde(deny_unknown_fields)]
/**
Features such as special abilities and actions in a stat block are a simple object that contains a single 'text' property list of [`crate::structured_text::TextBlock`]. The added property makes JSON formatting of the structure a little less confusing visually, and makes it easier to wrap the object in template code.
*/
pub struct StatBlockFeature {
    /**
    `text: list(<TextBlock>)`

    */
    pub text: Vec<TextBlock>
}

#[derive(PartialEq,Debug)]
#[derive(Serialize,Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StatBlockLegendary {
    pub description: Vec<TextBlock>,
    pub actions: Vec<StatBlockFeature>
}

#[derive(PartialEq,Debug)]
#[derive(Serialize,Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StatBlockLairActions {
    pub foreword: Vec<TextBlock>,
    pub actions: Vec<String>,
    pub afterword: Option<String>
}

#[derive(PartialEq,Debug)]
#[derive(Serialize,Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StatBlockRegionalEffects {
    pub foreword: Vec<TextBlock>,
    pub effects: Vec<String>,
    pub afterword: Option<String>
}

// This is the final output of monstorr, except it will be in JSON form.
// It's very similar to Open5e format, but not quite... Basically
// it's a list of keys and strings, with a few arrays, because it's
// just the data that actually gets templated.
#[derive(PartialEq,Debug)]
#[derive(Serialize,Deserialize)]
#[serde(deny_unknown_fields)]
/**
This is the final object output to JSON, or passed through the template at the end. The properties are listed with their types. If a property is optional, it is marked with a '?'.
*/
pub struct CreatureStatBlock {
    
    /**
    `name: <string>`

    */
    pub name: String,
    
    /**
    `size: <string>`

    */
    pub size: String,
    #[serde(rename="type")]
    
    /**
    `type: <string>`

    */
    pub type_: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    
    /**
    `subtype?: <string>`

    */
    pub subtype: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    
    /**
    `group?: <string>`

    */
    pub group: Option<String>,
    
    /**
    `alignment: <string>`

    */
    pub alignment: String,
    
    /**
    `armor: <string>`

    */
    pub armor: String,
    
    /**
    `hit_points: <string>`

    */
    pub hit_points: String,
    
    /**
    `speed: <string>`

    */
    pub speed: String,
    
    /**
    `strength: <string>`

    */
    pub strength: String,
    
    /**
    `dexterity: <string>`

    */
    pub dexterity: String,
    
    /**
    `constitution: <string>`

    */
    pub constitution: String,
    
    /**
    `intelligence: <string>`

    */
    pub intelligence: String,
    
    /**
    `wisdom: <string>`

    */
    pub wisdom: String,
    
    /**
    `charisma: <string>`

    */
    pub charisma: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    
    /**
    `saving_throws?: <string>`

    */
    pub saving_throws: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    
    /**
    `skills?: <string>`

    */
    pub skills: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    
    /**
    `damage_vulnerabilities?: <string>`

    */
    pub damage_vulnerabilities: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    
    /**
    `damage_resistances?: <string>`

    */
    pub damage_resistances: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    
    /**
    `damage_immunities?: <string>`

    */
    pub damage_immunities: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    
    /**
    `condition_immunities?: <string>`

    */
    pub condition_immunities: Option<String>,
    
    /**
    `senses: <string>`

    */
    pub senses: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    
    /**
    `languages?: <string>`

    If there are no languages, then this property is empty. In standard formatting, this is often replaced with an em dash if there are no languages. I find this inconsistent with the other properties (the same is not done for skills and saves), and leave that decision up to the creator.

    */
    pub languages: Option<String>,
    
    /**
    `challenge_rating: <string>`

    */
    pub challenge_rating: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    
    /**
    `special_abilities?: list(<StatBlockFeature>)`

    */
    pub special_abilities: Vec<StatBlockFeature>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    
    /**
    `actions?: list(<StatBlockFeature>)`

    */
    pub actions: Vec<StatBlockFeature>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    
    /**
    `reactions?: list(<StatBlockFeature>)`

    */
    pub reactions: Vec<StatBlockFeature>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    
    /**
    `legendary_actions?: <StatBlockLegendary>`

    */
    pub legendary_actions: Option<StatBlockLegendary>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    
    /**
    `lair_actions?: <StatBlockLairActions>`

    Lair actions are not yet supported.

    */
    pub lair_actions: Option<StatBlockLairActions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    
    /**
    `regional_effects: <StatBlockRegionalEffects>`

    Regional effects are not yet supported.

    */
    pub regional_effects: Option<StatBlockRegionalEffects>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    
    /**
    `source?: <string>`

    */
    pub source: Option<String>
}

impl CreatureStatBlock {

    pub fn write_to_string(&self, ugly: bool) -> Result<String,String> {

        if ugly {
            serde_json::to_string(self).map_err(|e| e.to_string())
        } else {
            serde_json::to_string_pretty(self).map_err(|e| e.to_string())
        }

    }
}


pub trait TryIntoStatBlock {

    type Error;

    fn try_into_stat_block(self) -> Result<CreatureStatBlock,Self::Error>;

}

impl<E, T: TryInto<CreatureStatBlock,Error=E>> TryIntoStatBlock for T {

    type Error=E;

    fn try_into_stat_block(self) -> Result<CreatureStatBlock,Self::Error> {
        self.try_into()
    }


}