/*
 * Copyright © 2022 Neil M. Sheldon
 * 
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::collections::HashMap;
use std::collections::BTreeMap; // forces sorting...
use std::collections::HashSet;

use crate::stats::Ability;


pub enum SpellcastingStyle {
    Third,
    Half,
    Full,
    Warlock
}

const THIRD_CAST_SLOTS: [[u8; 4]; 21] = [
    [0,0,0,0],
    [0,0,0,0],
    [0,0,0,0],
    [2,0,0,0],
    [3,0,0,0],
    [3,0,0,0],
    [3,0,0,0],
    [4,2,0,0],
    [4,2,0,0],
    [4,2,0,0],
    [4,3,0,0],
    [4,3,0,0],
    [4,3,0,0],
    [4,3,2,0],
    [4,3,2,0],
    [4,3,2,0],
    [4,3,3,0],
    [4,3,3,0],
    [4,3,3,0],
    [4,3,3,1],
    [4,3,3,1]
];

const HALF_CAST_SLOTS: [[u8; 5]; 21] = [
    [0,0,0,0,0],
    [0,0,0,0,0],
    [2,0,0,0,0],
    [3,0,0,0,0],
    [3,0,0,0,0],
    [4,2,0,0,0],
    [4,2,0,0,0],
    [4,3,0,0,0],
    [4,3,0,0,0],
    [4,3,2,0,0],
    [4,3,2,0,0],
    [4,3,3,0,0],
    [4,3,3,0,0],
    [4,3,3,1,0],
    [4,3,3,1,0],
    [4,3,3,2,0],
    [4,3,3,2,0],
    [4,3,3,3,1],
    [4,3,3,3,1],
    [4,3,3,3,2],
    [4,3,3,3,2]
];

const FULL_CAST_SLOTS: [[u8; 9]; 21] = [
    [0,0,0,0,0,0,0,0,0],
    [2,0,0,0,0,0,0,0,0],
    [3,0,0,0,0,0,0,0,0],
    [4,2,0,0,0,0,0,0,0],
    [4,3,0,0,0,0,0,0,0],
    [4,3,2,0,0,0,0,0,0],
    [4,3,3,0,0,0,0,0,0],
    [4,3,3,1,0,0,0,0,0],
    [4,3,3,2,0,0,0,0,0],
    [4,3,3,3,1,0,0,0,0],
    [4,3,3,3,2,0,0,0,0],
    [4,3,3,3,2,1,0,0,0],
    [4,3,3,3,2,1,0,0,0],
    [4,3,3,3,2,1,1,0,0],
    [4,3,3,3,2,1,1,0,0],
    [4,3,3,3,2,1,1,1,0],
    [4,3,3,3,2,1,1,1,0],
    [4,3,3,3,2,1,1,1,1],
    [4,3,3,3,3,1,1,1,1],
    [4,3,3,3,3,2,1,1,1],
    [4,3,3,3,3,2,2,1,1]
];

const WARLOCK_CAST_SLOTS: [(u8,u8); 21] = [
    (1,0),
    (1,1), // level, count
    (1,2),
    (2,2),
    (2,2),
    (3,2),
    (3,2),
    (4,2),
    (4,2),
    (5,2),
    (5,2),
    (5,3),
    (5,3),
    (5,3),
    (5,3),
    (5,3),
    (5,3),
    (5,4),
    (5,4),
    (5,4),
    (5,4)
];


pub struct Spellcasting {
    pub caster_level: u8,
    pub class: String,
    pub style: SpellcastingStyle,
    pub slots: HashMap<u8,u8>, // level, count
    pub ability: Ability,
    pub save_dc: Option<u8>,
    pub attack_bonus: Option<i8>,
    pub spells: BTreeMap<u8,Vec<String>>, // level, list of spells
    pub cast_before: HashSet<String>

}

impl Default for Spellcasting {

    fn default() -> Self {
        Self {
            caster_level: 0,
            ability: Ability::Intelligence,
            class: "Wizard".to_owned(),
            style: SpellcastingStyle::Full,
            slots: HashMap::new(),
            save_dc: None, 
            attack_bonus: None, 
            spells: BTreeMap::new(),
            cast_before: HashSet::new()
        }
    }


}

impl Spellcasting {

    pub const FEATURE_NAME: &'static str = "Spellcasting";

    pub fn set_caster_level(&mut self, level: u8) {
        self.caster_level = level;
        self.generate_spell_slots();
    }

    pub fn set_class(&mut self, class: String) {
        self.class = class.clone()
    }

    pub fn set_style(&mut self, style: SpellcastingStyle) {
        self.style = style;
        self.generate_spell_slots();

    }

    pub fn set_ability(&mut self, ability: Ability) {
        self.ability = ability.clone();
    }

    pub fn set_spell_slots(&mut self, level: u8, count: u8) {
        self.slots.insert(level,count);
    }

    pub fn set_save_dc(&mut self, save_dc: Option<u8>) {
        self.save_dc = save_dc
    }

    pub fn set_attack_bonus(&mut self, bonus: Option<i8>) {
        self.attack_bonus = bonus
    }

    pub fn add_spells(&mut self, level: u8, spells: &Vec<String>) {
        for spell in spells {
            self.spells.entry(level).or_insert_with(Vec::new).push(spell.clone())
        }
    }

    pub fn set_spells_before_combat(&mut self, spells: &Vec<String>) {
        for spell in spells {
            self.cast_before.insert(spell.clone());
        }
    }

    pub fn remove_spells(&mut self, spells: &Vec<String>) {
        for spell in spells {
            for (_,spells) in &mut self.spells {
                if let Some(index) = spells.iter().position(|a| a == spell) {
                    spells.remove(index);
                }
            }
            self.cast_before.remove(spell);
        }

    }

    pub fn get_description(&self) -> String {

        let save_dc = if let Some(save_dc) = self.save_dc {
            format!("{}",save_dc)
        } else {
            format!("${{8 + prof + {}}}",self.ability.to_short_str())
        };

        let attack_bonus = if let Some(attack_bonus) = self.attack_bonus {
            format!("{:+}",attack_bonus)
        } else {
            format!("${{+prof + {}}}",self.ability.to_short_str())
        };

        let mut found_cast_before = false;

        let warlock_style = if let SpellcastingStyle::Warlock = self.style {
            let (level,count) = WARLOCK_CAST_SLOTS[self.caster_level as usize];
            if count > 0 {
                format!(" and will cast them with {} spell slots of level {}",count,level)
            } else {
                "".to_owned()
            }
        } else {
            "".to_owned()
        };
        
        let mut result = format!("${{Subj}} is a {}-level spellcaster. Its spellcasting ability is {} (spell save DC {}, {} to hit with spell attacks). ${{Subj}} has the following {} spells prepared{}:",self.caster_level,self.ability,save_dc,attack_bonus,self.class,warlock_style);

        for (level,list) in &self.spells {

            result.push_str("${sub}");
            result.push_str(match level {
                0 => "Cantrips",
                1 => "1st level",
                2 => "2nd level",
                3 => "3rd level",
                4 => "4th level",
                5 => "5th level",
                6 => "6th level",
                7 => "7th level",
                8 => "8th level",
                9 => "9th level",
                _ => unreachable!()
            });
            result.push_str(&if *level > 0 {
                if let SpellcastingStyle::Warlock = self.style {
                    ": ".to_owned()
                } else {
                    format!(" ({} slots): ",self.slots.get(&(level-1)).unwrap_or(&0))
                }
            } else {
                " (at will): ".to_owned()
            });

            result.push_str(&list.iter().map(|spell| format!("${{italic( }}{}${{ )}}{}",spell,if self.cast_before.contains(spell) { 
                found_cast_before = true;
                "*" 
            } else { 
                "" 
            })).collect::<Vec<String>>().join(", "));

        }

        if found_cast_before {
            result.push_str("${par}* ${Subj} casts these spells on itself before combat.")
        }

        result

    }

    fn generate_spell_slots(&mut self) {
        self.slots.clear();
        match self.style {
            SpellcastingStyle::Full => {
                let slot_array = FULL_CAST_SLOTS[self.caster_level as usize];
                for i in 0..slot_array.len() as u8 {
                    self.slots.insert(i+1,slot_array[i as usize]);
                }
            },
            SpellcastingStyle::Half => {
                let slot_array = HALF_CAST_SLOTS[self.caster_level as usize];
                for i in 0..slot_array.len() as u8 {
                    self.slots.insert(i+1,slot_array[i as usize]);
                }
            },
            SpellcastingStyle::Third => {
                let slot_array = THIRD_CAST_SLOTS[self.caster_level as usize];
                for i in 0..slot_array.len() as u8 {
                    self.slots.insert(i+1,slot_array[i as usize]);
                }
            },
            SpellcastingStyle::Warlock => {
                // don't bother inserting anything because we're getting them from elsewhere.
                //let slots = WARLOCK_CAST_SLOTS[self.caster_level as usize -1];
                //self.slots.insert(slots.0, slots.1);
            }
        }
    }
}

pub struct InnateSpellcasting {
    pub ability: Ability,
    pub save_dc: Option<u8>,
    pub attack_bonus: Option<i8>,
    pub spells: BTreeMap<Option<u8>,Vec<String>>, // count per day or at will, list of spells
    pub restrictions: HashMap<String,String>
}

impl Default for InnateSpellcasting {

    fn default() -> Self {
        InnateSpellcasting {
            ability: Ability::Charisma,
            save_dc: None,
            attack_bonus: None,
            spells: BTreeMap::new(),
            restrictions: HashMap::new()
        }
    }


}

impl InnateSpellcasting {

    pub const FEATURE_NAME: &'static str = "Innate Spellcasting";

    pub fn set_ability(&mut self, ability: Ability) {
        self.ability = ability.clone();
    }

    pub fn set_save_dc(&mut self, save_dc: Option<u8>) {
        self.save_dc = save_dc
    }

    pub fn set_attack_bonus(&mut self, bonus: Option<i8>) {
        self.attack_bonus = bonus
    }

    pub fn add_spells(&mut self, per_day: Option<u8>, spells: &Vec<String>) {
        for spell in spells {
            self.spells.entry(per_day).or_insert_with(Vec::new).push(spell.clone())
        }
    }

    pub fn set_spell_restriction(&mut self, spell: String, restriction: String) {
        self.restrictions.insert(spell, restriction);
    }

    pub fn remove_spells(&mut self, spells: &Vec<String>) {
        for spell in spells {
            for (_,spells) in &mut self.spells {
                if let Some(index) = spells.iter().position(|a| a == spell) {
                    spells.remove(index);
                }
            }
            self.restrictions.remove(spell);
        }

    }


    pub fn get_description(&self) -> String {

        let save_dc = if let Some(save_dc) = self.save_dc {
            format!("{}",save_dc)
        } else {
            format!("${{8 + prof + {}}}",self.ability.to_short_str())
        };

        let attack_bonus = if let Some(attack_bonus) = self.attack_bonus {
            format!("{:+}",attack_bonus)
        } else {
            format!("${{+prof + {}}}",self.ability.to_short_str())
        };

        
        let mut result = format!("${{Poss}} innate spellcasting ability is {} (spell save DC {}, {} to hit with spell attacks). It can innately cast the following spells, requiring no material components:",self.ability,save_dc,attack_bonus);


        for (count,list) in &self.spells {
            result.push_str("${sub}");
            if let Some(count) = count {
                result.push_str(&format!("{}/day each: ",count))
            } else {
                result.push_str("At will: ")
            };

            result.push_str(&list.iter().map(|spell| format!("${{italic( }}{}${{ )}}{}",spell,if let Some(restriction) = self.restrictions.get(spell) { 
                format!(" ({})",restriction)
            } else { 
                "".to_owned() 
            })).collect::<Vec<String>>().join(", "));

            

        }


        result        
    }

}
