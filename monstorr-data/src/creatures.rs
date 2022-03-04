/*
 * Copyright © 2022 Neil M. Sheldon
 * 
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

pub const DRAGON_GOLD_ADULT: &str = include_str!("../data/adult_gold_dragon.creature");
pub const GOBLIN: &str = include_str!("../data/goblin.creature");
pub const BUGBEAR: &str = include_str!("../data/bugbear.creature");
pub const EFREETI: &str = include_str!("../data/efreeti.creature");

/* FUTURE: How could we have a built-in list? 
The list has to have summary data (name, type, size, alignment, cr, other stuff, see monstorr-lib list_creatures) and I don't want to create all of the creatures (assuming all SRD creatures eventually get imported).None
Alternatively, I could run the tool on all of them and store them as JSON instead, then I have the data I actually need, as sort of a "written in itself"
thing.
*/