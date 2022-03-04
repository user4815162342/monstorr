/*
 * Copyright © 2022 Neil M. Sheldon
 * 
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::fs::File;
use std::io::BufReader;

use serde::de::Visitor;
use serde::de::SeqAccess;
use serde::de::Unexpected;
use serde::de::Error as SerdeError;

use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde_json;

#[derive(Serialize,Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Open5eMonsterSpeed {
    pub walk: Option<u8>,
    pub swim: Option<u8>,
    pub fly: Option<u8>,
    pub burrow: Option<u8>,
    pub climb: Option<u8>,
    pub hover: Option<bool>, 
    pub notes: Option<String> 
}

impl Open5eMonsterSpeed {

    pub fn to_string(&self) -> String {

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
            if let Some(_) = self.hover {
                result.push(format!("fly {} ft. (hover)",fly))
            } else {
                result.push(format!("fly {} ft.",fly))
            }
        }
        if let Some(swim) = self.swim {
            result.push(format!("swim {} ft.",swim))
        }

        if let Some(notes) = &self.notes {
            format!("{} ({})",result.join(", "),notes)
        } else {
            result.join(", ")
        }
        
    }

}

#[derive(Serialize,Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Open5eMonsterSkills {
    pub athletics: Option<i8>,
    pub acrobatics: Option<i8>,
    pub sleight_of_hand: Option<i8>,
    pub stealth: Option<i8>,
    pub arcana: Option<i8>,
    pub history: Option<i8>,
    pub investigation: Option<i8>,
    pub nature: Option<i8>,
    pub religion: Option<i8>,
    pub animal_handling: Option<i8>, 
    pub insight: Option<i8>,
    pub medicine: Option<i8>,
    pub perception: Option<i8>,
    pub survival: Option<i8>,
    pub deception: Option<i8>,
    pub intimidation: Option<i8>,
    pub performance: Option<i8>,
    pub persuasion: Option<i8>,
}

impl Open5eMonsterSkills {

    pub fn to_string(&self) -> Option<String> {

        let mut result = vec![];

        macro_rules! add_skill {
            ($prop: ident, $name: literal) => {
                if let Some(score) = self.$prop {
                    result.push(format!(concat!($name," {:+}"),score));
                }
            };
        }

        add_skill!(acrobatics,"Acrobatics");
        add_skill!(animal_handling,"Animal Handling");
        add_skill!(arcana,"Arcana");
        add_skill!(athletics,"Athletics");
        add_skill!(deception,"Deception");
        add_skill!(history,"History");
        add_skill!(insight,"Insight");
        add_skill!(intimidation,"Intimidation");
        add_skill!(investigation,"Investigation");
        add_skill!(medicine,"Medicine");
        add_skill!(nature,"Nature");
        add_skill!(perception,"Perception");
        add_skill!(performance,"Performance");
        add_skill!(persuasion,"Persuasion");
        add_skill!(religion,"Religion");
        add_skill!(sleight_of_hand,"Sleight of Hands");
        add_skill!(stealth,"Stealth");
        add_skill!(survival,"Survival");


        if result.len() > 0 {
            Some(result.join(", "))
        } else {
            None
        }

    
    }
}

#[derive(Serialize,Deserialize,Clone)]
#[serde(deny_unknown_fields)]
pub struct Open5eMonsterAction {
    pub name: String,
    pub desc: String,
    pub attack_bonus: Option<i8>,
    pub damage_dice: Option<String>,
    pub damage_bonus: Option<i8>
}

// For the action types, at least, Open5e stores blank strings instead of null or empty array if there
// are none. I don't know why it does this, but this is how we fix it (and an attribute on the specified fields)
fn blank_string_or_vec<'de, D, ItemType: Deserialize<'de>>(deserializer: D) -> Result<Vec<ItemType>, D::Error>
where
    D: Deserializer<'de>,
{
    struct DeserializeVectorOrEmptyString<ItemType>(std::marker::PhantomData<ItemType>);

    impl<'de, ItemType: Deserialize<'de>> Visitor<'de> for DeserializeVectorOrEmptyString<ItemType> {
        type Value = Vec<ItemType>;
    
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("an array or blank string")
        }
    
        fn visit_seq<E>(self, mut seq: E) -> Result<Self::Value, E::Error>
        where
            E: SeqAccess<'de>,
        {
            let mut result = vec![];
            while let Some(item) = seq.next_element()? {
                result.push(item);
            }
            Ok(result)
        }
    
        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: SerdeError,
        {
            if v == "" {
                Ok(vec![])
            } else {
                Err(E::invalid_value(Unexpected::Str(v), &self))
            }
        }
    }
    

    deserializer.deserialize_any(DeserializeVectorOrEmptyString(std::marker::PhantomData))
}

// For some other types, Open5e stores blank strings instead of null or missing property where I want an option.
fn blank_string_is_none<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    struct DeserializeEmptyStringAsOption(std::marker::PhantomData<String>);

    impl<'de> Visitor<'de> for DeserializeEmptyStringAsOption {
        type Value = Option<String>;
    
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a string")
        }
    
        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: SerdeError,
        {
            if v == "" {
                Ok(None)
            } else {
                Ok(Some(v.to_owned()))
            }
        }
    }
    

    deserializer.deserialize_any(DeserializeEmptyStringAsOption(std::marker::PhantomData))
}

#[derive(Serialize,Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Open5eMonster {
    pub slug: String,
    pub name: String,
    pub size: String,
    #[serde(rename="type")]
    pub type_: String,
    #[serde(deserialize_with = "blank_string_is_none")]
    pub subtype: Option<String>,
    pub group: Option<String>,
    pub alignment: String,
    pub armor_class: u8,
    pub armor_desc: Option<String>,
    pub hit_points: u16,
    pub hit_dice: String,
    pub speed: Open5eMonsterSpeed,
    pub strength: u8,
    pub dexterity: u8,
    pub constitution: u8,
    pub intelligence: u8,
    pub wisdom: u8,
    pub charisma: u8,
    pub strength_save: Option<i8>,
    pub dexterity_save: Option<i8>,
    pub constitution_save: Option<i8>,
    pub intelligence_save: Option<i8>,
    pub wisdom_save: Option<i8>,
    pub charisma_save: Option<i8>,
    pub perception: Option<i8>, // Not sure why they have this separate when its already in skills, assuming this was an early iteration of the code
    pub skills: Open5eMonsterSkills,
    pub damage_vulnerabilities: String,
    pub damage_resistances: String,
    pub damage_immunities: String,
    pub condition_immunities: String,
    pub senses: String,
    pub languages: String,
    pub challenge_rating: String,
    #[serde(deserialize_with = "blank_string_or_vec")]
    pub actions: Vec<Open5eMonsterAction>,
    #[serde(deserialize_with = "blank_string_or_vec")]
    pub reactions: Vec<Open5eMonsterAction>,
    pub legendary_desc: String,
    #[serde(deserialize_with = "blank_string_or_vec")]
    pub legendary_actions: Vec<Open5eMonsterAction>,
    #[serde(deserialize_with = "blank_string_or_vec")]
    pub special_abilities: Vec<Open5eMonsterAction>,
    pub spell_list: Vec<String>,
    pub img_main: Option<String>,
    #[serde(rename="document__slug")]
    pub document_slug: String,
    #[serde(rename="document__title")]
    pub document_title: String,
    #[serde(rename="document__license_url")]
    pub document_license_url: String
}

impl Open5eMonster {

    pub fn load_from_str(source: &str) -> Result<Self,Open5eError> {
        match serde_json::from_str(source) {
            Ok(data) => Ok(data),
            Err(err) => Err(Open5eError::DeserializationError(err))
        }
    }

}

#[derive(Serialize,Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Open5eMonsterList {
  pub count: usize,
  pub next: Option<String>,
  pub previous: Option<String>,
  pub results: Vec<Open5eMonster>
}

pub enum Open5eError {
    DeserializationError(serde_json::Error)
}

impl std::fmt::Display for Open5eError {

    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::DeserializationError(err) => write!(f,"Error loading Open5e data: {}",err)
        }

    }
}

pub enum Open5eLoadError {
    IOError(std::io::Error),
    Open5eError(Open5eError)
}

impl std::fmt::Display for Open5eLoadError {

    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Open5eError(err) => write!(f,"{}",err),
            Self::IOError(err) => write!(f,"{}",err)
        }

    }
}



impl Open5eMonsterList {

    pub fn load_from_str(source: &str) -> Result<Self,Open5eError> {
        match serde_json::from_str(source) {
            Ok(data) => Ok(data),
            Err(err) => Err(Open5eError::DeserializationError(err))
        }
    }

    pub fn load<Source: std::io::Read>(data: Source) -> Result<Self,Open5eError> {
        match serde_json::from_reader(data) {
            Ok(data) => Ok(data),
            Err(err) => Err(Open5eError::DeserializationError(err))
        }
    }

    fn open_file(filename: &str) -> Result<BufReader<File>,std::io::Error> {
        let file = File::open(filename)?;
        Ok(BufReader::new(file))

    }

    pub fn load_from_file(filename: &str) -> Result<Self,Open5eLoadError> {
        match Self::open_file(filename) {
            Ok(reader) => {
                match Open5eMonsterList::load(reader) {
                    Ok(data) => Ok(data),
                    Err(err) => Err(Open5eLoadError::Open5eError(err))
                }

            },
            Err(err) => Err(Open5eLoadError::IOError(err))
        }

    }

}

