/*
 * Copyright © 2022 Neil M. Sheldon
 * 
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::parse_position::PositionRange;

#[derive(Clone,Debug)]
pub enum TokenError {
    InvalidEscape,
    InvalidNumber,
    InvalidDice,
    UnterminatedString,
    UnexpectedCharacter,
    SlashIsNotValid
}

impl std::fmt::Display for TokenError {

    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(),std::fmt::Error> {
        match self {
            TokenError::InvalidEscape => write!(f,"Invalid escape in text or string"),
            TokenError::InvalidNumber => write!(f,"Invalid number"),
            TokenError::InvalidDice => write!(f,"Invalid dice"),
            TokenError::UnterminatedString => write!(f,"Unterminated string"),
            TokenError::UnexpectedCharacter => write!(f,"Unexpected character"),
            TokenError::SlashIsNotValid => write!(f,"Slash is not a valid token, must be followed by '>' or '<'")
        }
    }
}

#[derive(Clone,Debug)]
pub struct TokenErrorDetails {
    pub source_name: String,
    pub position: PositionRange,
    pub error: TokenError

}

#[derive(Debug)]
pub enum InterpolationError {
    ScanError(TokenError),
    ExpectedIdentifier(String), // found
    ExpectedCloseParen(String), // found
    ExpectedExpression(String), // found
    CantNegateString,
    CantNegateObject,
    CantMultiplyDice,
    CantMultiplyStrings,
    CantMultiplyObjects,
    CantDivideStrings,
    CantDivideByDice,
    CantDivideObjects,
    CantConcatenateNonStrings,
    CantAddObjects,
    CantSubtractStrings,
    CantSubtractObjects,
    StringIsAlreadyStringified,
    CantStringifyObjects,
    UnknownVariable,
    UnknownProperty,
    InvalidIndex,
    CantSignObject,
    CantSignString,
    EmptyStack(String),
    UnexpectedStructuredText,
    TextIsAlreadyItalic,
    TextIsAlreadyBold,
    TextIsNotBold,
    TextIsNotItalic
}

impl std::fmt::Display for InterpolationError {

    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(),std::fmt::Error> {
        match self {
            Self::ScanError(err) => write!(f,"Token Error: {}",err),
            Self::ExpectedIdentifier(found) => write!(f,"Expected identifier, found {}",found),
            Self::ExpectedCloseParen(found) => write!(f,"Expected ')', found {}",found),
            Self::ExpectedExpression(found) => write!(f,"Expected expression, found {}",found),
            Self::CantNegateString => write!(f,"Strings can't be negated"),
            Self::CantNegateObject => write!(f,"Objects can't be negated"),
            Self::CantMultiplyDice => write!(f,"Terms can't be multiplied by a dice expression"),
            Self::CantMultiplyStrings => write!(f,"Strings can't be multiplied"),
            Self::CantMultiplyObjects => write!(f,"Objects can't be multiplied"),
            Self::CantDivideStrings => write!(f,"Strings can't be divided"),
            Self::CantDivideByDice => write!(f,"Terms can't be divided by a dice expression"),
            Self::CantDivideObjects => write!(f,"Objects can't be divided"),
            Self::CantConcatenateNonStrings => write!(f,"Non-strings can't be concatenated"),
            Self::CantAddObjects => write!(f,"Objects can't be added"),
            Self::CantSubtractStrings => write!(f,"Strings can't be subtracted"),
            Self::CantSubtractObjects => write!(f,"Objects can't be subtracted"),
            Self::StringIsAlreadyStringified => write!(f,"String is already stringified"),
            Self::CantStringifyObjects => write!(f,"Objects can't be stringified"),
            Self::UnknownVariable => write!(f,"Unknown variable"),
            Self::UnknownProperty => write!(f,"Unknown property"),
            Self::InvalidIndex => write!(f,"Invalid index"),
            Self::CantSignObject => write!(f,"Objects can't be signed"),
            Self::CantSignString => write!(f,"Strings can't be signed"),
            Self::EmptyStack(operation) => write!(f,"Internal error: stack is empty at operation {}",operation),
            Self::UnexpectedStructuredText => write!(f,"Internal error: unexpected structured text in deserialization"),
            Self::TextIsAlreadyBold => write!(f,"Text is already bold"),
            Self::TextIsAlreadyItalic => write!(f,"Text is already italic"),
            Self::TextIsNotBold => write!(f,"Text is not bold"),
            Self::TextIsNotItalic => write!(f,"Text is not italic")
        
        }
    }
}


#[derive(Debug)]
pub struct InterpolationErrorDetails {
    pub error: InterpolationError,
    pub source_name: String,
    pub position: PositionRange,
    pub full_text: Option<String>,
}

impl InterpolationErrorDetails {

    pub fn with_full_text(self, text: &str) -> Self {
        Self {
            error: self.error,
            source_name: self.source_name,
            position: self.position,
            full_text: Some(text.to_owned())
        }
    }

}

impl std::fmt::Display for InterpolationErrorDetails {

    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(),std::fmt::Error> {
        write!(f,"{} [{}:{}-{}:{}] {}",self.source_name,
                                    self.position.start.line,
                                    self.position.start.column,
                                    self.position.end.line,
                                    self.position.end.column,
                                    self.error)?;
        if let Some(full_text) = &self.full_text {
            writeln!(f)?;
            f.write_str(full_text)?;
        }
        Ok(())
    }
}


impl InterpolationError {

    pub fn details(self, source: &str, position: &PositionRange) -> InterpolationErrorDetails {
        InterpolationErrorDetails {
            error: self,
            source_name: source.to_owned(),
            position: position.clone(),
            full_text: None
        }
        
    }

}


impl From<TokenErrorDetails> for InterpolationErrorDetails {

    fn from(error: TokenErrorDetails) -> Self {
        Self::from(&error)
    }
}

impl From<&TokenErrorDetails> for InterpolationErrorDetails {

    fn from(error: &TokenErrorDetails) -> Self {
        Self {
            error: InterpolationError::ScanError(error.error.clone()),
            source_name: error.source_name.clone(),
            position: error.position.clone(),
            full_text: None
        }
    }
}



#[derive(Debug)]
pub enum IncludeError {
    FileError(std::io::Error),
    InterpolationError(InterpolationErrorDetails),
    DeserializationError(ron::Error),
    CreatureError(Box<CreatureError>),

}

impl std::fmt::Display for IncludeError {

    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(),std::fmt::Error> {
        match self {
            Self::FileError(err) => write!(f,"I/O Error: {}",err),
            Self::InterpolationError(err) => write!(f,"While interpolating parameters: {}",err),
            Self::DeserializationError(err) => write!(f,"While loading interpolated commands: {}",err),
            Self::CreatureError(err) => write!(f,"{}",err),        
        }
    }
}


impl std::convert::From<std::io::Error> for IncludeError {

    fn from(error: std::io::Error) -> Self {
        IncludeError::FileError(error)
    }

}

impl std::convert::From<InterpolationErrorDetails> for IncludeError {

    fn from(error: InterpolationErrorDetails) -> Self {
        IncludeError::InterpolationError(error)
    }

}

impl std::convert::From<ron::Error> for IncludeError {

    fn from(error: ron::Error) -> Self {
        IncludeError::DeserializationError(error)
    }

}

#[derive(Debug)]
pub enum CreatureError {
   MonstorrVersionNotSupported(f32), 
   CreatureHasNoName,
   IncludeError(String,IncludeError),
   WeaponAttackDoesNotMatchExpectation(String),
   WeaponEffectDoesNotMatchExpectation(String),
   WeaponNotFound(String,String), // name, action
   ActionNotFound(String,String), // name, action
   LairActionsNotSupportedYet,
   RegionalEffectsNotSupportedYet,
   ChallengeRatingNotAsExpected(String,String), // expected, found
   InvalidStateForFeature(String)
}


impl CreatureError {

    pub fn include_error<Error: std::convert::Into<IncludeError>>(file: &str, error: Error) -> Self {
        CreatureError::IncludeError(file.to_owned(),error.into())
    }
}

impl std::fmt::Display for CreatureError {

    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(),std::fmt::Error> {
        match self {
            Self::MonstorrVersionNotSupported(version) => write!(f,"The creature file can not be loaded with this version of Monstorr ({})",version),
            Self::CreatureHasNoName => write!(f,"Creature was not given a name"),
            Self::IncludeError(file,err) => write!(f,"In included file {}: {}",file,err),
            Self::WeaponAttackDoesNotMatchExpectation(name) => write!(f,"Weapon {} attack does not match expected.",name),
            Self::WeaponEffectDoesNotMatchExpectation(name) => write!(f,"Weapon {} effect does not match expected.",name),
            Self::WeaponNotFound(name,action) => write!(f,"Could not find weapon named {} {}.",name,action),
            Self::ActionNotFound(name,action) => write!(f,"Could not find action named {} {}.",name,action),
            Self::LairActionsNotSupportedYet => write!(f,"Lair actions are not supported yet."),
            Self::RegionalEffectsNotSupportedYet => write!(f,"Regional effects are not supported yet."),
            Self::ChallengeRatingNotAsExpected(expected,found) => write!(f,"Challenge rating {} did not match expected {}",found,expected),
            Self::InvalidStateForFeature(error) => write!(f,"{}",error),
        }
    }

}


impl std::convert::From<CreatureError> for IncludeError {

    fn from(error: CreatureError) -> Self {
        IncludeError::CreatureError(Box::new(error))
    }

}

