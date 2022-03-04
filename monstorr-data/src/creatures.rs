/*
 * Copyright © 2022 Neil M. Sheldon
 * 
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

pub struct CreatureSummary<PropertyType> {
    pub name: PropertyType,
    pub slug: PropertyType,
    pub type_: PropertyType,
    pub subtype: Option<PropertyType>,
    pub size: PropertyType,
    pub alignment: PropertyType,
    pub challenge_rating: PropertyType
}

include!("../data/creatures/creature_database.rs.inc");