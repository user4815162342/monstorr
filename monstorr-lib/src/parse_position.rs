/*
 * Copyright © 2022 Neil M. Sheldon
 * 
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

#[derive(Clone,Debug)]
pub struct Position {
    pub column: usize,
    pub line: usize
}

#[derive(Clone,Debug)]
pub struct PositionRange {
    pub start: Position,
    pub end: Position
}

impl PositionRange {
    
    #[allow(dead_code)]
    pub fn none() -> Self {
        Self {
            start: Position {
                line: 0,
                column: 0
            },
            end: Position {
                line: 0,
                column: 0
            }
        }
    }
}
