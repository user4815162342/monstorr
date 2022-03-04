/*
 * Copyright © 2022 Neil M. Sheldon
 * 
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::fmt::Display;
use std::path::Path;
use std::path::PathBuf;

pub trait NextOk<ItemType,ErrorType> {

    fn next_ok(&mut self) -> Result<Option<ItemType>,ErrorType>;
    // fn next_ok(&mut self) -> Result<Option<Self::Item>,Self::Error>;

}

impl<T,ItemType,ErrorType> NextOk<ItemType,ErrorType> for T where T: Iterator<Item = Result<ItemType,ErrorType>> {

    fn next_ok(&mut self)  -> Result<Option<ItemType>,ErrorType> {
        match self.next() {
            Some(Ok(value)) => Ok(Some(value)),
            Some(Err(err)) => Err(err),
            None => Ok(None)
        }
    }
}


pub trait FloorDiv {

    fn div_floor(&self, rhs: &Self) -> Self;

}

macro_rules! impl_floor_div {
    ($int: ty) => {
        impl FloorDiv for $int {
    
            // this is done in the 'num-integer' crate, but I don't want to import an entire crate just for this.
            // FUTURE: This may be implemented in standard rust someday https://github.com/rust-lang/rust/issues/88581
            fn div_floor(&self, rhs: &Self) -> Self {
                let d = self/rhs; // this just does integer division
                let r = self%rhs; // mod keeps sign of the numerator, ignores sign of denominator
                if (r > 0 && *rhs < 0) || (r < 0 && *rhs > 0) {
                    d - 1
                } else {
                    d
                }
        
            }
        }
                
    };
}

impl_floor_div!(i8);
impl_floor_div!(isize);

pub trait CeilingDiv {

    fn div_ceiling(&self, rhs: &Self) -> Self;

}

macro_rules! impl_ceiling_div {
    ($int: ty) => {
        impl CeilingDiv for $int {
    
            // this is done in the 'num-integer' crate, but I don't want to import an entire crate just for this.
            // FUTURE: This may be implemented in standard rust someday https://github.com/rust-lang/rust/issues/88581
            fn div_ceiling(&self, rhs: &Self) -> Self {
                let d = self/rhs;
                let r = self%rhs;
                if (r > 0 && *rhs > 0) || (r < 0 && *rhs < 0) {
                    d + 1
                } else {
                    d
                }
        
            }
        }
                
    };
}

impl_ceiling_div!(isize);

pub trait AndJoin {

    fn and_join(&self) -> String;

}

impl<ItemType: Display> AndJoin for [ItemType] {

    fn and_join(&self) -> String {
        match self.len() {
            0 => String::new(),
            1 => self[0].to_string(),
            2 => format!("{} and {}",self[0],self[1]),
            _ => {
                let mut result = String::new();
                let (head,end) = self.split_at(self.len()-1);
                for item in head {
                    result.push_str(&item.to_string());
                    result.push_str(", ");
                }
                result.push_str("and ");
                for item in end {
                    result.push_str(&item.to_string())
                };
                result
            }
        }
    }


}

impl<ItemType: Display> AndJoin for &Vec<ItemType> {

    fn and_join(&self) -> String {
        match self.len() {
            0 => String::new(),
            1 => self[0].to_string(),
            2 => format!("{} and {}",self[0],self[1]),
            _ => {
                let mut result = String::new();
                let (head,end) = self.split_at(self.len()-1);
                for item in head {
                    result.push_str(&item.to_string());
                    result.push_str(", ");
                }
                result.push_str("and ");
                for item in end {
                    result.push_str(&item.to_string())
                };
                result
            }
        }
    }


}

pub trait Capitalize {

    fn capitalize_first_letter(&self) -> Self;
}

impl Capitalize for String {

    fn capitalize_first_letter(&self) -> Self {
        let mut chars = self.chars();
        match chars.next() {
            None => String::new(),
            Some(char) => char.to_uppercase().collect::<Self>() + chars.as_str(),
        }        
    }

}

pub trait DisplayWithThousands {

    fn display_with_thousands(&self) -> String;
}

impl DisplayWithThousands for u32 {


    fn display_with_thousands(&self) -> String {
        let mut result = String::new();
        let formatted = format!("{}",self);
        let chars = formatted.chars().rev().enumerate();
        for (index,value) in chars {
            if (index != 0) && (index % 3 == 0) {
                result.insert(0,',')
            };
            result.insert(0,value);
        }
        result
        
    }

}


// from https://stackoverflow.com/a/39343127
pub fn path_relative_from(path: &Path, base: &Path) -> Option<PathBuf> {
    use std::path::Component;

    if path.is_absolute() != base.is_absolute() {
        if path.is_absolute() {
            Some(PathBuf::from(path))
        } else {
            None
        }
    } else {
        let mut ita = path.components();
        let mut itb = base.components();
        let mut comps: Vec<Component> = vec![];
        loop {
            match (ita.next(), itb.next()) {
                (None, None) => break,
                (Some(a), None) => {
                    comps.push(a);
                    comps.extend(ita.by_ref());
                    break;
                }
                (None, _) => comps.push(Component::ParentDir),
                (Some(a), Some(b)) if comps.is_empty() && a == b => (),
                (Some(a), Some(b)) if b == Component::CurDir => comps.push(a),
                (Some(_), Some(b)) if b == Component::ParentDir => return None,
                (Some(a), Some(_)) => {
                    comps.push(Component::ParentDir);
                    for _ in itb {
                        comps.push(Component::ParentDir);
                    }
                    comps.push(a);
                    comps.extend(ita.by_ref());
                    break;
                }
            }
        }
        Some(comps.iter().map(|c| c.as_os_str()).collect())
    }
}
