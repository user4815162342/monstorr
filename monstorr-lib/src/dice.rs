/*
 * Copyright © 2022 Neil M. Sheldon
 * 
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use serde::Deserialize;
use serde::Serialize;
use serde::Serializer;

#[derive(Debug,PartialEq,Clone)]
#[derive(Deserialize)]
#[serde(try_from = "String")]
pub enum Die {
    D4,
    D6,
    D8,
    D10,
    D12,
    D20,
    DCustom(u8)
}

impl Die {

    fn new(sides:u8) -> Self {
        match sides {
            4 => Die::D4,
            6 => Die::D6,
            8 => Die::D8,
            10 => Die::D10,
            12 => Die::D12,
            20 => Die::D20,
            a => Die::DCustom(a)
        }

    }

    fn average(&self) -> f32 {
        match self {
            Die::D4 => 2.5,
            Die::D6 => 3.5, 
            Die::D8 => 4.5,
            Die::D10 => 5.5,
            Die::D12 => 6.5,
            Die::D20 => 10.5,
            Die::DCustom(sides) => ((*sides as f32) + 1f32) / 2f32
        }
    }

    fn to_string(&self) -> String {
        match self {
            Die::D4 => format!("d4"),
            Die::D6 => format!("d6"),
            Die::D8 => format!("d8"),
            Die::D10 => format!("d10"),
            Die::D12 => format!("d12"),
            Die::D20 => format!("d20"),
            Die::DCustom(a) => format!("d{}",a)

        }
    }
}

impl std::str::FromStr for Die {
    type Err = ParseDiceError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let sides = if let Some(value) = s.strip_prefix(&['d','D'][..]) {
            value.parse().or_else(|_| Err(ParseDiceError::ExpectedU8ForSides))?
        } else {
            s.parse().or_else(|_| Err(ParseDiceError::ExpectedU8ForSides))?
        };
        Ok(Self::new(sides))

    }
}

impl std::convert::TryFrom<String> for Die {

    type Error = ParseDiceError;


    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()

    }

}

impl Serialize for Die {

    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}


#[derive(Debug)]
pub enum ParseDiceError {
    ExpectedU8ForSides,
    ExpectedU8ForCount,
    ExpectedDCharacter
}


impl std::fmt::Display for ParseDiceError {

    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(),std::fmt::Error> {
        match self {
            ParseDiceError::ExpectedU8ForSides => write!(f,"Expected number of dice sides to be 1-{}.",std::u8::MAX),
            ParseDiceError::ExpectedU8ForCount => write!(f,"Expected dice count to be 1-{}.",std::u8::MAX),
            ParseDiceError::ExpectedDCharacter => write!(f,"No 'd' found in string")
        }

    }

}

#[derive(Debug,PartialEq,Clone)]
#[derive(Deserialize)]
#[serde(try_from = "String")]
pub struct Dice {
    pub coefficient: u8,
    pub die: Die
}

impl Dice {

    pub fn new(coefficient: u8, die: &Die) -> Dice {
        Dice {
            coefficient,
            die: die.clone()
        }
    }

    pub fn average(&self) -> u16 {
        let result = (self.coefficient as f32) * self.die.average();
        result.floor() as u16
    }

}

impl From<Die> for Dice {
    fn from(die: Die) -> Self {
        Self {
            coefficient: 1,
            die
        }
    }
}
 

impl std::fmt::Display for Dice {

    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(),std::fmt::Error> {
        write!(f,"{}{}",self.coefficient,self.die.to_string()) 
   }
}


impl std::str::FromStr for Dice {
    type Err = ParseDiceError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((coefficient,tail)) = s.split_once(&['d','D'][..]) {
            let coefficient = if coefficient == "" {
                1u8
            } else {
                coefficient.trim().parse().or_else(|_| Err(ParseDiceError::ExpectedU8ForCount))?
            };

            let die = tail.trim().parse()?;
            Ok(Dice {
                coefficient,
                die
            })
        } else {
            Err(ParseDiceError::ExpectedDCharacter)
        }

    }
}

impl std::convert::TryFrom<String> for Dice {

    type Error = ParseDiceError;


    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()

    }

}

impl Serialize for Dice {

    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}


