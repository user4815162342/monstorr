/*
 * Copyright © 2022 Neil M. Sheldon
 * 
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use serde::Deserialize;
use serde::Serialize;

#[derive(PartialEq,Debug,Clone)]
#[derive(Serialize,Deserialize)]
/**

One of the variants in this value is an attempt to mimic a common reaction found in the SRD, which may have an effect on the challenge rating. However, since the SRD does not use consistent phrasing for all actions, you may not get the output you expect. If you want to use the reaction with the specified name, use that, even if you have to override the description to fix it later. That way, you'll get the benefits of challenge rating calculation when that is available.

*/
pub enum Reaction {

    /**
    `Parry(<integer>)`

    Adds a parry reaction to the creature. The integer is the AC bonus they gain by using this reaction.
    */
    Parry(u8), // AC bonus

    /**
    `Reaction(<string>,<string>)`

    Adds a custom reaction to the creature, with the specified name and description. These will be interpolated.
    */
    Reaction(String,String), // name, description

}

impl Reaction {


    pub fn get_name(&self) -> String {
        match self {
            Reaction::Parry(..) => "Parry".to_owned(),
            Reaction::Reaction(name,_) => name.clone()
        }
    }

    pub fn get_description(&self) -> String {
        match self {
            Reaction::Parry(ac) => format!("${{Subj}} adds {} to ${{posspro}} AC against one melee attack that would hit ${{objpro}}. To do so, ${{subj}} must see the attacker and be wielding a melee weapon.",ac),
            Reaction::Reaction(_,description) => description.clone()
        }
    }


}
