/*
 * Copyright © 2022 Neil M. Sheldon
 * 
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::rc::Rc;

use crate::dice::Dice;
use crate::errors::TokenError;


#[derive(Debug,Clone,PartialEq)]
pub enum Token<Keyword> {
    /*
    
    text (initial state, inclusion mode) = <BOF> text-char* '$<'
    text (continuing state, inclusion mode) = '>' text-char* ('$<' | <EOF>) 
    text (initial state, creation mode) = <BOF> text-char* '${' 
    text (continuing state, creation mode) = '}' text-char* ('${' | <EOF>) 
    text-char = ('\\' | '\$' | [^\$])

    */
    Text(Rc<str>), 

    /*
    identifier = ident-start (ident-start | digit)*

    ident-start = 'A'..'Z' | 'a'..'z' | '$' | '_'

    digit = '0'..'9'
    */
    Identifier(Rc<str>),

    /*
    number = digit+ !'d'
    */
    Number(isize),

    /*
    string = '"' string-char* '"'

    string-char = ('\\' | '\"' | [^\"])
    */
    String(Rc<str>),

    /*
    dice = digit+ ('d' | 'D') digit+ 
    */
    Dice(Dice),

    Keyword(Keyword),

    Plus,
    Minus,
    Asterisk,
    /*
    slash-less-than = '/' '<'
    */
    SlashLessThan, // floor division
    /*
    slash-greater-than = '/' '>'
    */
    SlashGreaterThan, // ceiling division
    OpenParenthesis,
    CloseParenthesis,
    Dot,
    Dollar

}



#[macro_export]
macro_rules! digit {
    () => {
        '0'..='9'
    };
}

#[macro_export]
macro_rules! ident_start {
    () => {
        'A'..='Z' | 'a'..='z' | '_'
    };
}

#[macro_export]
macro_rules! whitespace {
    () => {
        ' ' | '\t' | '\n' | '\r'
    };
}

pub trait Tokenizer<Keyword2> {

    fn resolve_keyword(&self, identifier: &str) -> Option<Keyword2>;

    fn next_char(&mut self) -> Option<char>;

    fn peek_char(&mut self) -> Option<&char>;

    fn template_text(&mut self, interpolation_char: &char) -> Result<Token<Keyword2>,TokenError> {
        let mut result = String::new();
        loop {
            match self.next_char() {
                Some('\\') => match self.peek_char() {
                    Some(char) if matches!(char,'$' | '\\') => {
                        let char = *char;
                        self.next_char();
                        result.push(char);
                    }
                    _ => break Err(TokenError::InvalidEscape)
                },
                Some('$') => match self.peek_char() {
                    Some(char) if char == interpolation_char => {
                        self.next_char();
                        break Ok(Token::Text(Rc::from(result)));
                    },
                    _ => {
                        result.push('$')
                    }
                },
                Some(char) => result.push(char),
                None => break Ok(Token::Text(Rc::from(result)))
            }
        }
    }


    fn string(&mut self) -> Result<Token<Keyword2>,TokenError> {
        let mut result = String::new();
        loop {
            match self.next_char() {
                Some('\\') => match self.peek_char() {
                    Some(char) if matches!(char,'"' | '\\') => {
                        let char = *char;
                        self.next_char();
                        result.push(char);
                    }
                    _ => break Err(TokenError::InvalidEscape)
                },
                Some('"') => break Ok(Token::String(Rc::from(result))),
                Some(char) => result.push(char),
                None => break Err(TokenError::UnterminatedString)
            }
        }

    }

    fn identifier(&mut self, start: char) -> Result<Token<Keyword2>,TokenError> {
        let mut result = start.to_string();
        loop {
            match self.peek_char() {
                Some(char) if matches!(char,ident_start!() | digit!()) => {
                    result.push(*char);
                    self.next_char();
                },
                _ => break {
                    Ok(if let Some(keyword) = self.resolve_keyword(result.as_str()) {
                        Token::Keyword(keyword)
                    } else {
                        Token::Identifier(Rc::from(result))
                    })
                }
            }
        }
    }

    fn integer(&mut self, integer_str: &mut String) {
        loop {
            match self.peek_char() {
                Some(char) if matches!(char,digit!()) => {
                    integer_str.push(*char);
                    self.next_char();
                },
                _ => break
            }
        }

    }

    fn number_or_dice(&mut self, start: char) -> Result<Token<Keyword2>,TokenError> {
        let mut integer_str = start.to_string();
        self.integer(&mut integer_str);

        if let Some('d' | 'D') = self.peek_char() {
            // this is a dice expression
            self.next_char();
            let mut die_str = String::new();
            self.integer(&mut die_str);

            match (integer_str.parse(),die_str.parse()) {
                (Ok(count),Ok(die)) => Ok(Token::Dice(Dice::new(count,&die))),
                _ => Err(TokenError::InvalidDice)
            }

        } else if let Ok(integer) = integer_str.parse() {
            Ok(Token::Number(integer))
        } else {
            Err(TokenError::InvalidNumber)
        }
            
    }

    fn skip_whitespace(&mut self) {
        while let Some(whitespace!()) = self.peek_char() {
            self.next_char();
        }

    }
}

