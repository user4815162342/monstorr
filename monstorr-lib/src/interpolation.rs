/*
 * Copyright © 2022 Neil M. Sheldon
 * 
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

/*!

In Monstorr, interpolation is the process of substituting variables and expressions with actual values at the last minute. There are actually two types of interpolation that happen in Monstorr, one occurs during the generation of stat blocks from feature descriptions and other strings, the other occurs when including an external creature file into another (see [`crate::creature_commands::CreatureCommand::Include`]). They both work basically the same, but have slightly different inputs and outputs.

Strings which can be interpolated contain normal text interspersed with interpolation expressions delimited from the rest. In order to make it easier to include feature interpolation strings inside an included document, the delimiters for feature interpolation and inclusion interpolation are different.

The result of an interpolation differs between the two modes. For inclusion, the result is a plain string that will then be deserialized into creature commands. For descriptions, however, the result is structured text.

# Expression Delimiters

When embedding interpolation in the included file, delimit expressions with '$<...>'. When embedding interpolation in features, delimit the expressions with '${...}'. 

All text outside these delimiters are treated as normal text. While you can use the backslash ('\') to escape the dollar sign character ('$'), and other backslashes, this is not necessary unless, for some reason, you need to include the '${' character combination in regular text, or worse, the '\\${' character combination. In the former case you can produce that with '\\\\${'. In the latter, you can produce it with '\\\\\\${'. Outside of those instances, a dollar sign and a backslash are treated as normal characters. In interpolating for inclusion, the same applies to '$<' and '\$<' character combinations.

# Expression Syntax

The expression syntax is fairly simple. The following values operations are supported:

*Variable References.* A plain identifier is a variable reference. Variables are provided by Monstorr according to the mode of interpolation. In inclusion, they are string values provided as arguments by the `Include` command, see [`crate::creature_commands::CreatureCommand::Include`]. In descriptions, the variables are properties of the creature, which may be strings, numbers or dice. For available creature properties, see [`crate::creature::Creature`]. If the variable does not exist, or returns no value, an error will occur when it is referenced. 

*Strings*. Strings are delimited by double-quotes. They return a string value.

*Numbers*. Only integers are supported, and consist of a series of decimal digits. They return a number value.

*Dice.* Dice literals are supported in forms like "1d8" and "2d6". Both numbers are required. They return a dice expression value. When a dice expression is converted to a string, it is formatted as in standard descriptions with the average followed by the actual expression in parentheses.

*Parenthesis.* Expressions in between parentheses '(..)' will be calculated first.

*Properties and Index.* Some values returned by variables are objects that have properties, or lists that have indexes. Properties are accessed by using the '.' operator followed by the property's identifier. List items are accessed by using dot operator followed by a number indicating the index to access. 

*Negating Numbers.* The '-' can be used before a number to negate it. The value must be a number, no other value types can be negated. Remember that if you negate a value that occurs after another expression, it may be confused with the minus operator. Use parentheses or replace it with '+ -' to remove this ambiguity.

*Signing Numbers and Dice.* Numbers and dice expressions can be signed, so that when they are formatted as strings, a '+' appears before them if they are positive. The '-' will always appear if they are negative. The '+' operator which appears before the value will ensure that that takes place. It is infectious, so if you add other values to the expression, the final result of the expression will be formatted that way. Remember that if you sign a value that occurs after another expression, it may be confused with the plus operator. Use parentheses or replace it with '+ +' to remove this ambiguity.

*Stringifying Values.* Numbers and dice are automatically converted to strings when they are the final result of an expression. However, for type-safety, they are not if they are used in a more compound expression, and concatenating strings and non-strings is not allowed. This prevents you from making mistakes like '"foo" + 1 + 2' when what you really meant was '"foo" (+1 + 2)'. The '$' operator can be used before a number or dice value to convert it to a string, making concatenation possible.

*Multiplication.* To multiply two number expressions, use the '*' operator. You can multiply dice by numbers to get a new dice expression. If the number is not 1 or -1, then your dice expression will be formatted with a factor. You can not multiply two dice values together, as the result is undetermined. Remember, the coefficient of a dice value is not the same as a numeric factor: "2d6" is not the same as "1d6 x 2", as they have different sets of possible results.

*Division.* As only integer number values are allowed, it is necessary to know how to deal with fractions when dividing them. For that reason, there are two division operators. Use "/<" to divide the numbers and round the result down, use "/>" to divide the numbers and round the result up. Dice values can be divided by numbers, which will result in them having a factor expression when formatted. However, nothing can be divided by dice expressions, as the result of that is undetermined.

*Addition, Subtraction and Concatenation.* To add and subtract number and dice values, use the "+" and "-" characters respectively. Adding dice together can create more complex dice expressions which involve several dice terms. If you add the same dice together, it should result in simply changing the coefficient. For example, "2d6 + 1d6" should result in "3d6". However, "2d4 - 1d4" would not be the same as "1d4", as they would have different sets of results (the first would have values ranging from -1 to 7, while the other has values ranging from 1-4, and that doesn't even consider distribution )

*Consecutive Expressions.* If two expressions are included consecutive to each other, then the results are both pushed out to the resulting string in order. For example, '"foo" "bar"' would output the text "foobar".

*Structured Text Functions.* The result of interpolating a description is not just a simple string, it's a list of text blocks that can contain styled text. The actual result is a list of [`crate::structured_text::TextBlock`]s. There are two kinds of text blocks, and each of these can contain lists of four different kinds of spans. These are achieved through special functions.

A structured text function consists of a keyword, followed by an optional parentheses expression. Unlike normal expressions, these parentheses can contain normal template text, thus "italic( } italicized text ${ )" can be done. There are four such functions, which modify the structured text output.

* `par`: starts a new normal paragraph block, allowing you to break up your description text. There is a variant of this function which takes an argument in parentheses, but it should not be used, as it has the result of creating what looks like a new feature in the stat block.

* `sub(...)`, `sub`: starts a subparagraph, with a heading contained in the parenthesis. In official content, a subparagraph in this case would create a paragraph with a hanging indent. The heading is an optional emphasized text that will appear at the beginning of the first line. In the official content, subparagraphs are used in the metallic dragon breath weapons, and in spellcasting spell lists.

* 'italic(...)': sets the text enclosed in the parentheses as italic. Nesting italic text inside italic text is not allowed, as it has undetermined effects. You can nest bold text inside italics, however.

* 'bold(...)': sets the text enclosed in the parentheses as bold. Nesting bold text inside bold text is not allowed, as it has undetermined effects. You can nest italic text inside bold, however.

# Examples

For examples of interpolation, look at the built-in creatures, whose raw files can be retrieved using the validate command from the Monstorr command line.

*/
use std::iter::Peekable;
use std::rc::Rc;
use std::collections::HashMap;

use crate::dice::Dice;
use crate::dice_expression::DiceExpression;
use crate::tokenizer::Token;
use crate::parse_position::Position;
use crate::parse_position::PositionRange;
use crate::errors::TokenError;
use crate::errors::TokenErrorDetails;
use crate::errors::InterpolationError;
use crate::errors::InterpolationErrorDetails;
use crate::tokenizer::Tokenizer;
use crate::utils::NextOk;
use crate::ident_start;
use crate::digit;
use crate::utils::FloorDiv;
use crate::utils::CeilingDiv;
use crate::structured_text::TextBlock;
use crate::structured_text::TextSpan;

#[derive(Clone,Debug)]
pub enum StructureKeyword {
    Par,
    Sub,
    Italic,
    Bold
}


#[derive(Clone,Debug)]
struct TokenDetails {
    position: PositionRange,
    token: Token<StructureKeyword>
}

enum InterpolationState {
    Initial,
    Continuing
}

enum InterpolationMode {
    DeserializeCreatureCommands,
    CalculateStatBlock
}

struct InterpolationTokenizer<Source: Iterator<Item=char>> {
    source_name: String,
    source: Peekable<Source>,
    mode: InterpolationMode,
    state: InterpolationState,
    position: PositionRange,
    current: Option<Result<(Token<StructureKeyword>,PositionRange),TokenErrorDetails>>
}

impl<Source: Iterator<Item=char>> Tokenizer<StructureKeyword> for InterpolationTokenizer<Source> {

    fn resolve_keyword(&self, identifier: &str) -> Option<StructureKeyword> {
        if let InterpolationMode::CalculateStatBlock = self.mode {
            match identifier {
                "par" => Some(StructureKeyword::Par),
                "sub" => Some(StructureKeyword::Sub),
                "italic" => Some(StructureKeyword::Italic),
                "bold" => Some(StructureKeyword::Bold),
                _ => None
            }
        } else {
            None
        }

    }



    fn next_char(&mut self) -> Option<char> {
        match self.source.next() {
            Some(char) => {
                match char {
                    '\n' => {
                        self.position.end.line += 1;
                        self.position.end.column = 0;    
                    },
                    '\r' => (
                        // this is probably part of a CRLF pair on windows. We don't want to increment characters that will mess up positioning,
                        // and we don't want to increment the line because the \n is going to do it. \r is only supported as a line-break
                        // in things like Commodore and Apple II, which are far from being supported here.
                    ), 
                    _ => self.position.end.column += 1
                }
                Some(char)
            },
            None => None
        }
    }

    fn peek_char(&mut self) -> Option<&char> {
        self.source.peek()
    }

} 

impl<Source: Iterator<Item=char>> InterpolationTokenizer<Source> {

    pub fn new(source_name: &str, source: Source, mode: InterpolationMode) -> Self {
        Self {
            source_name: source_name.to_owned(),
            source: source.peekable(),
            mode,
            state: InterpolationState::Initial,
            position: PositionRange {
                start: Position {
                    line: 1,
                    column: 1
                },
                end: Position {
                    line: 1,
                    column: 1
                }
            },
            current: None
        }
    }

    fn start_token(&mut self) {
        self.position.start = self.position.end.clone()
    }

    fn stop_token(&mut self, token: Result<Token<StructureKeyword>,TokenError>) -> Option<Result<(Token<StructureKeyword>,PositionRange),TokenErrorDetails>> {
        self.current = Some(match token.clone() {
            Ok(token) => Ok((token,self.position.clone())),
            Err(token) => Err(TokenErrorDetails {
                error: token,
                source_name: self.source_name.clone(),
                position: self.position.clone()
            })
        });
        self.current.clone()
    }

    fn no_more_tokens(&mut self) -> Option<Result<(Token<StructureKeyword>,PositionRange),TokenErrorDetails>> {
        self.current = None;
        None
    }


}

impl<Source: Iterator<Item=char>> Iterator for InterpolationTokenizer<Source> {

    type Item = Result<(Token<StructureKeyword>,PositionRange),TokenErrorDetails>;

    fn next(&mut self) -> Option<Self::Item> {

        match self.state {
            InterpolationState::Initial => {
                // automatically switch to the continuing mode. A file always consists of at least one text token,
                // even if it's 0-length.
                self.state = InterpolationState::Continuing;

                self.start_token();

                let mode_char = match self.mode {
                    InterpolationMode::DeserializeCreatureCommands => '<',
                    InterpolationMode::CalculateStatBlock => '{'
                };
                let text = self.template_text(&mode_char);
                self.stop_token(text)
            },
            InterpolationState::Continuing => {
                self.skip_whitespace();

                self.start_token();
                match self.next_char() {
                    Some(char) if matches!(char,ident_start!()) => {
                        let identifier = self.identifier(char);
                        self.stop_token(identifier)
                    },
                    Some(char) if matches!(char,digit!()) => {
                        let result = self.number_or_dice(char);
                        self.stop_token(result)
                    },
                    Some('"') => {
                        let string = self.string();
                        self.stop_token(string)
                    },
                    Some('+') => self.stop_token(Ok(Token::Plus)),
                    Some('-') => self.stop_token(Ok(Token::Minus)),
                    Some('*') => self.stop_token(Ok(Token::Asterisk)),
                    Some('/') => match self.peek_char() {
                        Some('<') => {
                            self.next_char();
                            self.stop_token(Ok(Token::SlashLessThan))
                        },
                        Some('>') => {
                            self.next_char();
                            self.stop_token(Ok(Token::SlashGreaterThan))
                        },
                        _ => {
                            self.stop_token(Err(TokenError::SlashIsNotValid))
                        }
                    },
                    Some('(') => self.stop_token(Ok(Token::OpenParenthesis)),
                    Some(')') => self.stop_token(Ok(Token::CloseParenthesis)),
                    Some('.') => self.stop_token(Ok(Token::Dot)),
                    Some('$') => self.stop_token(Ok(Token::Dollar)),
                    Some('>') if matches!(self.mode,InterpolationMode::DeserializeCreatureCommands) => {
                        let text = self.template_text(&'<');
                        self.stop_token(text)
                    },
                    Some('}') if matches!(self.mode,InterpolationMode::CalculateStatBlock) => {
                        let text = self.template_text(&'{');
                        self.stop_token(text)
                    },
                    Some(_) => self.stop_token(Err(TokenError::UnexpectedCharacter)),
                    None => self.no_more_tokens()
                }
            }
        }
        

    }

}



pub trait InterpolationObject {

    fn get_property(&self, property: &Rc<str>) -> Option<InterpolationValue>;

    // this only needs to be implemented by an array object
    fn get_index(&self, _index: &usize) -> Option<InterpolationValue> {
        None
    }
}

impl InterpolationObject for () {

    fn get_property(&self, _property: &Rc<str>) -> Option<InterpolationValue> {
        None
    }

}

// TODO: Get rid of this.
pub struct OverriddenInterpolationObject {
    overridden_namespace: Rc<str>,
    overridden: Rc<dyn InterpolationObject>,
    overrider: Rc<dyn InterpolationObject>
}

impl InterpolationObject for OverriddenInterpolationObject {

    fn get_property(&self, property: &Rc<str>) -> Option<InterpolationValue> {
        if property == &self.overridden_namespace {
            self.overridden.get_property(property)
        } else if let Some(value) = self.overrider.get_property(property) {
            Some(value)
        } else {
            self.overridden.get_property(property)
        }
    }

    // this only needs to be implemented by an array object
    fn get_index(&self, index: &usize) -> Option<InterpolationValue> {
        if let Some(value) = self.overrider.get_index(index) {
            Some(value)
        } else {
            self.overridden.get_index(index)
        }
    }

}

impl<T: InterpolationObject> InterpolationObject for Rc<T> {
 
    fn get_property(&self, property: &Rc<str>) -> Option<InterpolationValue> {
        self.as_ref().get_property(property)
    }

    fn get_index(&self, index: &usize) -> Option<InterpolationValue> {
        self.as_ref().get_index(index)
    }

}

impl InterpolationObject for HashMap<String,String> {

    fn get_property(&self, property: &Rc<str>) -> Option<InterpolationValue> {
        if let Some(value) = self.get(property.as_ref()) {
            Some(InterpolationValue::String(Rc::from(value.as_str())))
        } else {
            None
        }
    }

}


#[derive(Clone)]
pub enum InterpolationValue {
    String(Rc<str>),
    Number(isize,bool), // value, whether to display sign in string
    Dice(DiceExpression,bool), // value, whether to display sign in string
    #[allow(dead_code)] // FUTURE: I'm leaving this because it might come in handy some day.
    Object(Rc<dyn InterpolationObject>) 
}


impl InterpolationValue {

    fn get_property(&self, property: &Rc<str>) -> Option<InterpolationValue> {
        match self {
            InterpolationValue::Object(object) => object.get_property(property),
            _ => None
        }
    }

    fn get_index(&self, index: &usize) -> Option<InterpolationValue> {
        match self {
            InterpolationValue::Object(object) => object.get_index(index),
            _ => None
        }
    }

    fn negate(&self) -> Result<InterpolationValue,InterpolationError> {
        match self {
            InterpolationValue::Number(num,sign) => Ok(InterpolationValue::Number(-num,*sign)),
            InterpolationValue::Dice(dice,sign) => Ok(InterpolationValue::Dice(dice.multiply(&-1),*sign)),
            InterpolationValue::String(_) => Err(InterpolationError::CantNegateString),
            InterpolationValue::Object(_) => Err(InterpolationError::CantNegateObject)
                
        }
    }

    fn stringify(&self) -> Result<InterpolationValue,InterpolationError> {
        match self {
            InterpolationValue::Number(_,_) |
            InterpolationValue::Dice(_,_) => Ok(InterpolationValue::String(Rc::from(format!("{}",self)))),
            InterpolationValue::String(_) => Err(InterpolationError::StringIsAlreadyStringified),
            InterpolationValue::Object(_) => Err(InterpolationError::CantStringifyObjects)
                
        }
    }

    fn signed(&self) -> Result<InterpolationValue, InterpolationError> {
        match self {
            InterpolationValue::Number(num,_) => Ok(InterpolationValue::Number(*num,true)),
            InterpolationValue::Dice(dice,_) => Ok(InterpolationValue::Dice(dice.clone(),true)),
            InterpolationValue::String(_) => Err(InterpolationError::CantSignString),
            InterpolationValue::Object(_) => Err(InterpolationError::CantSignObject)
                
        }

    }

    fn multiply(&self,rhs: &InterpolationValue) -> Result<InterpolationValue,InterpolationError> {
        match (self,rhs) {
            (InterpolationValue::Number(num,sign),InterpolationValue::Number(rhs,_)) => Ok(InterpolationValue::Number(num*rhs,*sign)),
            (InterpolationValue::Dice(dice,sign),InterpolationValue::Number(rhs,_)) |
            (InterpolationValue::Number(rhs,sign),InterpolationValue::Dice(dice,_)) => Ok(InterpolationValue::Dice(dice.multiply(rhs),*sign)),
            (InterpolationValue::String(_),_) | (_,InterpolationValue::String(_)) => Err(InterpolationError::CantMultiplyStrings),
            (InterpolationValue::Object(_),_) | (_,InterpolationValue::Object(_)) => Err(InterpolationError::CantMultiplyObjects),
            (InterpolationValue::Dice(..),InterpolationValue::Dice(..)) => Err(InterpolationError::CantMultiplyDice),
            
        }
    }

    fn divide_ceiling(&self,rhs: &InterpolationValue) -> Result<InterpolationValue,InterpolationError> {
        match (self,rhs) {
            (InterpolationValue::Number(num,sign),InterpolationValue::Number(rhs,_)) => Ok(InterpolationValue::Number(num.div_ceiling(rhs),*sign)),
            (InterpolationValue::Dice(dice,sign),InterpolationValue::Number(rhs,_)) => Ok(InterpolationValue::Dice(dice.div_ceiling(rhs),*sign)),
            (InterpolationValue::String(_),_) | (_,InterpolationValue::String(_)) => Err(InterpolationError::CantDivideStrings),
            (InterpolationValue::Object(_),_) | (_,InterpolationValue::Object(_)) => Err(InterpolationError::CantDivideObjects),
            (_,InterpolationValue::Dice(..)) => Err(InterpolationError::CantDivideByDice),

        }
    }

    fn divide_floor(&self,rhs: &InterpolationValue) -> Result<InterpolationValue,InterpolationError> {
        match (self,rhs) {
            (InterpolationValue::Number(num,sign),InterpolationValue::Number(rhs,_)) => Ok(InterpolationValue::Number(num.div_floor(rhs),*sign)),
            (InterpolationValue::Dice(dice,sign),InterpolationValue::Number(rhs,_)) => Ok(InterpolationValue::Dice(dice.div_floor(rhs),*sign)),
            (InterpolationValue::String(_),_) | (_,InterpolationValue::String(_)) => Err(InterpolationError::CantDivideStrings),
            (InterpolationValue::Object(_),_) | (_,InterpolationValue::Object(_)) => Err(InterpolationError::CantDivideObjects),
            (_,InterpolationValue::Dice(..)) => Err(InterpolationError::CantDivideByDice),

        }
    }

    fn add(&self,rhs: &InterpolationValue) -> Result<InterpolationValue,InterpolationError> {
        match (self,rhs) {
            (InterpolationValue::Number(num,sign),InterpolationValue::Number(rhs,_)) => Ok(InterpolationValue::Number(num+rhs,*sign)),
            (InterpolationValue::Dice(dice,sign),InterpolationValue::Number(rhs,_)) |
            (InterpolationValue::Number(rhs,sign),InterpolationValue::Dice(dice,_)) => Ok(InterpolationValue::Dice(dice.add(rhs),*sign)),
            (InterpolationValue::String(lhs),InterpolationValue::String(rhs)) => Ok(InterpolationValue::String(Rc::from(lhs.as_ref().to_owned() + rhs))),
            (InterpolationValue::Dice(lhs,sign),InterpolationValue::Dice(rhs,_)) => Ok(InterpolationValue::Dice(lhs.add_dice(rhs),*sign)),
            (InterpolationValue::Object(_),_) | (_,InterpolationValue::Object(_)) => Err(InterpolationError::CantAddObjects),
            (InterpolationValue::String(_),_) | (_,InterpolationValue::String(_)) => Err(InterpolationError::CantConcatenateNonStrings),

        }
    }

    fn subtract(&self,rhs: &InterpolationValue) -> Result<InterpolationValue,InterpolationError> {
        match (self,rhs) {
            (InterpolationValue::Number(num,sign),InterpolationValue::Number(rhs,_)) => Ok(InterpolationValue::Number(num-rhs,*sign)),
            (InterpolationValue::Dice(dice,sign),InterpolationValue::Number(rhs,_)) => Ok(InterpolationValue::Dice(dice.subtract(rhs),*sign)),
            (InterpolationValue::Number(num,sign),InterpolationValue::Dice(dice,_)) => Ok(InterpolationValue::Dice(dice.multiply(&-1).add(num),*sign)),
            (InterpolationValue::Dice(lhs,sign),InterpolationValue::Dice(rhs,_)) => Ok(InterpolationValue::Dice(lhs.subtract_dice(rhs),*sign)),
            (InterpolationValue::Object(_),_) | (_,InterpolationValue::Object(_)) => Err(InterpolationError::CantSubtractObjects),
            (InterpolationValue::String(_),_) | (_,InterpolationValue::String(_)) => Err(InterpolationError::CantSubtractStrings),

        }
    }

}

impl std::fmt::Display for InterpolationValue {

    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(),std::fmt::Error> {
        match self {
            Self::Dice(dice,sign) => if *sign {
                if dice.is_negative() {
                    write!(f,"{}",dice)
                } else {
                    write!(f,"+{}",dice)
                }
            } else {
                write!(f,"{}",dice)
            }
            Self::Number(num,sign) => if *sign {
                write!(f,"{:+}",num)
            } else {
                write!(f,"{}",num)
            }
            Self::String(str) => write!(f,"{}",str),
            Self::Object(_) => write!(f,"<object>")
        }
   }
}

#[derive(Debug)]
enum InterpolationOperation {
    // creates a string value and puts it on the stack
    CreateString(Rc<str>),
    // creates a number value and puts it on the stack
    CreateNumber(isize),
    // creates a dice value and puts it on the stack
    CreateDice(Dice),
    // gets the specified variable by name and puts it on the stack
    GetVariable(Rc<str>),
    // replaces the value on the top of the stack with  the result of calling 'get_index' on it
    GetIndex(usize),
    // replaces the value at the top of the stack with the result of calling 'get_property' on it
    GetProperty(Rc<str>),
    // replaces the value at the top of the stack with the result of calling 'negate' on it.
    Negate,
    // replaces the value at the top of the stack with the result of calling 'stringify' on it.
    Stringify,
    // replaces the value at the top of the stack with the result of calling 'sign' on it
    Sign,
    // takes two values off the stack and replaces with the result of calling multiply on them.
    Multiply,
    // takes two values off the stack and replaces with the result of calling floor_divide on them.
    FloorDivide,
    // takes two values off the stack and replaces with the result of calling ceiling_divide on them.
    CeilingDivide,
    // takes two values off the stack and replaces with the result of calling add on them.
    Add,
    // takes two values off the stack and replaces with the result of calling subtract on them.
    Subtract,
    // takes value off the stack, stringifies it if necessary, and appends it to the current string
    Append,
    
    // structured text operations
    // if italic mode is on, then throw an error, otherwise:
    // - put the current string into a span according to the current italic and bold modes, add to current span list, and clear current span
    // - set the italic mode to on
    StartItalic,
    // if bold mode is on, then throw an error, otherwise:
    // - put the current string into a span according to the current italic and bold modes, add to current span list, and clear current span
    // - set the bold mode to on
    StartBold,
    // if italic mode is off, then throw an error, otherwise:
    // - put the current string into a span according to the current italic and bold modes, add to current span list, and clear current span
    // - set the italic mode to off
    EndItalic,
    // if bold mode is off, then throw an error, otherwise:
    // - put the current string into a span according to the current italic and bold modes, add to current span list, and clear current span
    // - set the bold mode to off
    EndBold,

    // - put the current string into a span according to the current italic and bold modes, add to current span list, and clear current span
    // - add a block using the current heading and current span list, according to the current mode
    // - set current mode to paragraph
    StartParagraph,
    // - put the current string into a span according to the current italic and bold modes, add to current span list, and clear current span
    // - add a block using the current heading and current span list, according to the current mode
    // - set current mode to list item
    StartListItem,
    // - put the current string into a span according to the current italic and bold modes, add to current span list, and clear current span
    // - set the current span list as the current heading, and clear the current span list
    EndBlockStart,


}



/*
document = expression+ 
*/
struct Document {
    operations: Vec<(InterpolationOperation,PositionRange)>
}

impl Document {

/*
variable_expression = identifier ('.' identifier)*
*/
    fn parse_variable_expression<Source: Iterator<Item=char>>(tokenizer: &mut InterpolationTokenizer<Source>, operations: &mut Vec<(InterpolationOperation,PositionRange)>, identifier: Rc<str>, position: PositionRange) -> Result<(),InterpolationErrorDetails> {
        operations.push((InterpolationOperation::GetVariable(identifier.clone()),position.clone()));
        tokenizer.next_ok()?;
        while let Some(Ok((token,_))) = &tokenizer.current {
            match token {
                Token::Dot => {
                    match tokenizer.next_ok()? {
                        Some((Token::Identifier(str), position)) => 
                            operations.push((InterpolationOperation::GetProperty(str.clone()),position.clone())), 
                        Some((Token::Number(num), position)) => 
                            operations.push((InterpolationOperation::GetIndex(num as usize),position.clone())), 
                        _ => 
                            Err(InterpolationError::ExpectedIdentifier(format!("{:?}",tokenizer.current)).details(&tokenizer.source_name,&tokenizer.position))?
                    }
                },
                _ => break
            }

        }

        Ok(())
                
    }



/*
term = string_literal | number_literal | dice_literal | variable_reference | '(' expression ')'
*/
    fn parse_term<Source: Iterator<Item=char>>(tokenizer: &mut InterpolationTokenizer<Source>, operations: &mut Vec<(InterpolationOperation,PositionRange)>) -> Result<(),InterpolationErrorDetails> {

        match &tokenizer.current {
            Some(Ok((token, position))) => Ok(match &token {
                Token::String(str) => {
                    operations.push((InterpolationOperation::CreateString(str.clone()),position.clone()));
                    tokenizer.next_ok()?;
                }
                Token::Number(num) => {
                    operations.push((InterpolationOperation::CreateNumber(*num),position.clone()));
                    tokenizer.next_ok()?;
                },
                Token::Dice(dice) => {
                    operations.push((InterpolationOperation::CreateDice(dice.clone()),position.clone()));
                    tokenizer.next_ok()?;
                },
                Token::OpenParenthesis => {
                    tokenizer.next_ok()?;
                    Self::parse_expression(tokenizer,operations)?;
                    if let Some((Token::CloseParenthesis, ..)) = tokenizer.next_ok()? {
                    } else {
                        Err(InterpolationError::ExpectedCloseParen(format!("{:?}",tokenizer.current)).details(&tokenizer.source_name,&tokenizer.position))?
                    }
                },
                Token::Identifier(str) => {
                    let str = str.clone();
                    let position = position.clone();
                    Self::parse_variable_expression(tokenizer,operations,str,position)?
                },
                _ => Err(InterpolationError::ExpectedExpression(format!("{:?}",token)).details(&tokenizer.source_name,&tokenizer.position))?
            }),
            Some(Err(token)) => Err(token.into()),
            None => Err(InterpolationError::ExpectedExpression("end of file".to_owned()).details(&tokenizer.source_name,&tokenizer.position))
        }

    }




/*
unary_expression = ('-')* term
*/
    fn parse_unary<Source: Iterator<Item=char>>(tokenizer: &mut InterpolationTokenizer<Source>, operations: &mut Vec<(InterpolationOperation,PositionRange)>) -> Result<(),InterpolationErrorDetails> {
        let mut unary_operations = vec![];

        while let Some(Ok((token,position))) = &tokenizer.current {
            match token {
                Token::Minus => {
                    unary_operations.push((InterpolationOperation::Negate,position.clone()));
                    tokenizer.next_ok()?;
                },
                Token::Dollar => {
                    unary_operations.push((InterpolationOperation::Stringify,position.clone()));
                    tokenizer.next_ok()?;
                },
                Token::Plus => {
                    unary_operations.push((InterpolationOperation::Sign,position.clone()));
                    tokenizer.next_ok()?;
                }
                _ => break
            }

        }


        Self::parse_term(tokenizer,operations)?;
        
        // reverse through the things to push from inner to outer
        for op in unary_operations.into_iter().rev() {
            operations.push(op)
        };
        Ok(())

    }


/*
mul_expression = unary (('*' | '/<' '/>') unary)*
*/
    fn parse_mul<Source: Iterator<Item=char>>(tokenizer: &mut InterpolationTokenizer<Source>, operations: &mut Vec<(InterpolationOperation,PositionRange)>) -> Result<(),InterpolationErrorDetails> {
        Self::parse_unary(tokenizer,operations)?;

        while let Some(Ok((token,position))) = &tokenizer.current {
            let position = position.clone();
            match token {
                Token::Asterisk => {
                    Self::parse_unary(tokenizer,operations)?;
                    operations.push((InterpolationOperation::Multiply,position));
                },
                Token::SlashGreaterThan => {
                    Self::parse_unary(tokenizer,operations)?;
                    operations.push((InterpolationOperation::CeilingDivide,position));
                },
                Token::SlashLessThan => {
                    Self::parse_unary(tokenizer,operations)?;
                    operations.push((InterpolationOperation::FloorDivide,position));
                },
                _ => break
            }

        }

        Ok(())
    }


/*
add_expression = mul_expression (('+' | '-') mul_expression)*
*/
    fn parse_add<Source: Iterator<Item=char>>(tokenizer: &mut InterpolationTokenizer<Source>, operations: &mut Vec<(InterpolationOperation,PositionRange)>) -> Result<(),InterpolationErrorDetails> {
        Self::parse_mul(tokenizer,operations)?;

        while let Some(Ok((token,position))) = &tokenizer.current {
            let position = position.clone();
            match token {
                Token::Plus => {
                    Self::parse_mul(tokenizer,operations)?;
                    operations.push((InterpolationOperation::Add,position));
                },
                Token::Minus => {
                    Self::parse_mul(tokenizer,operations)?;
                    operations.push((InterpolationOperation::Subtract,position));
                },
                _ => break
            }

        }

        Ok(())
    }
    
    fn parse_command_arguments<Source: Iterator<Item=char>>(tokenizer: &mut InterpolationTokenizer<Source>, operations: &mut Vec<(InterpolationOperation,PositionRange)>) -> Result<PositionRange,InterpolationErrorDetails> {
        if let Some((Token::OpenParenthesis,_)) = tokenizer.next_ok()? {
            tokenizer.next_ok()?;
            loop {
                if let Some(Ok((Token::CloseParenthesis,position))) = &tokenizer.current {
                    let position = position.clone();
                    tokenizer.next_ok()?;
                    break Ok(position);
                }
                
                Self::parse_expression(tokenizer, operations)?;
                operations.push((InterpolationOperation::Append,tokenizer.position.clone()));
            }
        } else {
            Ok(tokenizer.position.clone())
        }

    }
    
    fn parse_command<Source: Iterator<Item=char>>(tokenizer: &mut InterpolationTokenizer<Source>, operations: &mut Vec<(InterpolationOperation,PositionRange)>, keyword: StructureKeyword, position: PositionRange) -> Result<(),InterpolationErrorDetails> {
        match keyword {
            StructureKeyword::Bold => {
                operations.push((InterpolationOperation::StartBold,position.clone()));
                let position = Self::parse_command_arguments(tokenizer, operations)?;
                operations.push((InterpolationOperation::EndBold,position.clone()));
            },
            StructureKeyword::Italic => {
                operations.push((InterpolationOperation::StartItalic,position.clone()));
                let position = Self::parse_command_arguments(tokenizer, operations)?;
                operations.push((InterpolationOperation::EndItalic,position.clone()));
            },
            StructureKeyword::Sub => {
                operations.push((InterpolationOperation::StartListItem,position.clone()));
                let position = Self::parse_command_arguments(tokenizer, operations)?;
                operations.push((InterpolationOperation::EndBlockStart,position.clone()));
            },
            StructureKeyword::Par => {
                operations.push((InterpolationOperation::StartParagraph,position.clone()));
                let position = Self::parse_command_arguments(tokenizer, operations)?;
                operations.push((InterpolationOperation::EndBlockStart,position.clone()));
            }
        }
        Ok(())
    }


/*
expression = text | add_expression
*/
    fn parse_expression<Source: Iterator<Item=char>>(tokenizer: &mut InterpolationTokenizer<Source>, operations: &mut Vec<(InterpolationOperation,PositionRange)>) -> Result<(),InterpolationErrorDetails> {
        match &tokenizer.current {
            Some(Ok((Token::Text(str), position))) => {
                operations.push((InterpolationOperation::CreateString(str.clone()),position.clone()));
                tokenizer.next_ok()?;
                Ok(())
            },
            _ => Ok(Self::parse_add(tokenizer,operations)?)
        }


    }

/*
document = (command | expression)*
*/
    fn parse<Source: Iterator<Item=char>>(tokenizer: &mut InterpolationTokenizer<Source>) -> Result<Self,InterpolationErrorDetails> {
        let mut operations = vec![];
        tokenizer.next_ok()?;

        while let Some(_) = tokenizer.current {
            if let Some(Ok((Token::Keyword(keyword),position))) = &tokenizer.current {
                let keyword = keyword.clone();
                let position = position.clone();
                Self::parse_command(tokenizer,&mut operations,keyword.clone(),position.clone())?;
            } else {
                Self::parse_expression(tokenizer,&mut operations)?;
                operations.push((InterpolationOperation::Append,tokenizer.position.clone()));
            }
        }


        
    
        Ok(Self {
            operations
        })
    }

    fn parse_str(source: &str, source_name: &str, mode: InterpolationMode) -> Result<Document,InterpolationErrorDetails> {
        let mut tokenizer = InterpolationTokenizer::new(source_name, source.chars(), mode);
        let result = Document::parse(&mut tokenizer);
        result
    }

    fn interpolate<Data: InterpolationObject>(&self, source_name: &str, data: &Data) -> Result<Vec<TextBlock>,InterpolationErrorDetails> {


        let mut result = Vec::new();
        let mut current_string = String::new();
        let mut current_spans = Vec::new();
        let mut stack = Vec::new();
        let mut italic_mode = false;
        let mut bold_mode = false;
        let mut list_mode = false;
        let mut current_heading = None;

        macro_rules! end_span {
            () => {
                if !current_string.is_empty() {
                    let span = std::mem::take(&mut current_string);
                    current_spans.push(match (italic_mode,bold_mode) {
                        (false,false) => TextSpan::Normal(span),
                        (true,false) => TextSpan::Italic(span),
                        (false,true) => TextSpan::Bold(span),
                        (true,true) => TextSpan::BoldItalic(span)
                    });
                }
            };
        }

        macro_rules! reset_span {
            () => {
                italic_mode = false;
                bold_mode = false;                    
            };
        }

        macro_rules! end_block {
            () => {
                if (current_heading.is_some()) || (current_spans.len() > 0) {
                    let heading = std::mem::take(&mut current_heading);
                    let body = std::mem::take(&mut current_spans);
                    result.push(if list_mode {
                        TextBlock::SubParagraph {
                            heading, body
                        }
                     } else {
                        TextBlock::Paragraph {
                            heading, body
                        }
                     });
                }
            
            };
        }
        
        for (operation,position) in &self.operations {

            macro_rules! pop {
                () => {
                    if let Some(value) = stack.pop() {
                        value
                    } else {
                        Err(InterpolationErrorDetails {
                            error: InterpolationError::EmptyStack(format!("{:?}",operation)),
                            source_name: source_name.to_owned(),
                            position: position.clone(),
                            full_text: None
                        })?
                    }
                };
            }

            macro_rules! error {
                ($error: ident) => {
                    Err(InterpolationErrorDetails {
                        error: InterpolationError::$error,
                        source_name: source_name.to_owned(),
                        position: position.clone(),
                        full_text: None
                    })?                    
                };
            }

            macro_rules! map_err {
                ($expr: expr) => {
                    $expr.map_err(|error| InterpolationErrorDetails {
                        error,
                        source_name: source_name.to_owned(),
                        position: position.clone(),
                        full_text: None
                    })?                    
                };
            }

    
            match operation {
                InterpolationOperation::CreateString(str) => {
                    stack.push(InterpolationValue::String(str.clone()));
                },
                InterpolationOperation::CreateNumber(num) => {
                    stack.push(InterpolationValue::Number(*num,false));      
                },
                InterpolationOperation::CreateDice(dice) => {
                    stack.push(InterpolationValue::Dice(dice.clone().into(),false));
                },
                InterpolationOperation::GetVariable(name) => {
                    if let Some(value) = data.get_property(name) {
                        stack.push(value)
                    } else {
                        error!(UnknownVariable)
                    }
                },
                InterpolationOperation::GetIndex(index) => {
                    let value = pop!();
                    if let Some(value) = value.get_index(index) {
                        stack.push(value)
                    } else {
                        error!(InvalidIndex)
                    }
                    
                },
                InterpolationOperation::GetProperty(name) => {
                    let value = pop!();
                    if let Some(value) = value.get_property(name) {
                        stack.push(value);
                    } else {
                        error!(UnknownProperty)
                    }                    
                },
                InterpolationOperation::Negate => {
                    let value = pop!();
                    stack.push(map_err!(value.negate()))
                },
                InterpolationOperation::Stringify => {
                    let value = pop!();
                    stack.push(map_err!(value.stringify()))  
                },
                InterpolationOperation::Sign => {
                    let value = pop!();
                    stack.push(map_err!(value.signed()))
                    
                },
                InterpolationOperation::Multiply => {
                    let rhs = pop!();
                    let lhs = pop!();
                    stack.push(map_err!(lhs.multiply(&rhs)))
                },
                InterpolationOperation::FloorDivide => {
                    let rhs = pop!();
                    let lhs = pop!();
                    stack.push(map_err!(lhs.divide_floor(&rhs)))
                    
                },
                InterpolationOperation::CeilingDivide => {
                    let rhs = pop!();
                    let lhs = pop!();
                    stack.push(map_err!(lhs.divide_ceiling(&rhs)))
                    
                },
                InterpolationOperation::Add => {
                    let rhs = pop!();
                    let lhs = pop!();
                    stack.push(map_err!(lhs.add(&rhs)))
                    
                },
                InterpolationOperation::Subtract => {
                    let rhs = pop!();
                    let lhs = pop!();
                    stack.push(map_err!(lhs.subtract(&rhs)))
                },
                InterpolationOperation::Append => {
                    let value = pop!();
                    current_string.push_str(&value.to_string());                    
                },

                InterpolationOperation::StartItalic => {
                    if italic_mode {
                        error!(TextIsAlreadyItalic)
                    }
                    end_span!();
                    italic_mode = true;
                },
                InterpolationOperation::StartBold => {
                    if bold_mode {
                        error!(TextIsAlreadyBold)
                    }
                    end_span!();
                    bold_mode = true;
                },
                InterpolationOperation::EndItalic => {
                    if !italic_mode {
                        error!(TextIsNotItalic)
                    }
                    end_span!();
                    italic_mode = false;
                },
                InterpolationOperation::EndBold => {
                    if !bold_mode {
                        error!(TextIsNotBold)
                    }
                    end_span!();
                    bold_mode = false;
                },
                InterpolationOperation::EndBlockStart => {
                    end_span!();
                    if current_spans.len() > 0 {
                        current_heading = Some(current_spans);
                        current_spans = Vec::new();
                    }
                    reset_span!();
                },
                InterpolationOperation::StartParagraph => {
                    end_span!();
                    end_block!();
                    reset_span!();
                    list_mode = false;
                },
                InterpolationOperation::StartListItem => {
                    end_span!();
                    end_block!();
                    reset_span!();
                    list_mode = true;
                },


            }
        }

        end_span!();
        end_block!();

        Ok(result)
    }


}

pub fn interpolate_str_for_deserialization<Data: InterpolationObject>(source: &str, source_name: &str, data: &Data, show_text_in_error: bool) -> Result<String,InterpolationErrorDetails> {
    match Document::parse_str(source, source_name, InterpolationMode::DeserializeCreatureCommands).and_then(|a| a.interpolate(source_name, data)) {
        Ok(value) => {
            // DeserializeCreatureCommands mode doesn't allow the commands that would create structured text
            if value.len() == 1 {
                if let Some(TextBlock::Paragraph{ heading: None, body: spans}) = value.last() {
                    if spans.len() == 1 {
                        if let Some(TextSpan::Normal(text)) = spans.last() {
                            return Ok(text.clone());
                        }
                    }
                }
            };
            Err(InterpolationErrorDetails {
                error: InterpolationError::UnexpectedStructuredText,
                source_name: source_name.to_owned(),
                position: PositionRange {
                    start: Position {
                        line: 0,
                        column: 0
                    },
                    end: Position {
                        line: 0,
                        column: 0
                    }
                },
                full_text: if show_text_in_error {
                    Some(source.to_owned())
                } else {
                    None
                }
            })
        },
        Err(err) => if show_text_in_error {
            Err(err.with_full_text(source))
        } else {
            Err(err)
        }
    }
}

pub fn interpolate_str_for_statblock<Data: InterpolationObject>(source: &str, source_name: &str, data: &Data, show_text_in_error: bool) -> Result<Vec<TextBlock>,InterpolationErrorDetails> {
    match Document::parse_str(source, source_name, InterpolationMode::CalculateStatBlock).and_then(|a| a.interpolate(source_name,data)) {
        Ok(value) => Ok(value),
        Err(err) => if show_text_in_error {
            Err(err.with_full_text(source))
        } else {
            Err(err)
        }
    }
}


pub fn interpolate_simple_markdown_naively(heading_source: &str, source: &str, source_name: &str, use_sub_block: bool, show_text_in_error: bool) -> Result<Vec<TextBlock>,InterpolationErrorDetails> {

    // simple parsing
    let mut current_string = String::new();
    let mut operations = Vec::new();

    let mut chars = source.chars().enumerate().peekable();
    let mut start = 0;
    let mut end = 0;
    let mut bold = false;
    let mut italic = false;

    fn get_position(start: usize, end: usize) -> PositionRange {
        PositionRange {
            start: Position {
                line: 1,
                column: start
            },
            end: Position {
                line: 1,
                column: end
            }

        }
    }

    // push the name on as the heading start
    if use_sub_block {
        operations.push((InterpolationOperation::StartListItem,get_position(start, end)));
    } else {
        operations.push((InterpolationOperation::StartParagraph,get_position(start, end)));
    }
    operations.push((InterpolationOperation::CreateString(Rc::from(heading_source)),get_position(0, 0)));
    operations.push((InterpolationOperation::Append,get_position(0,0)));
    operations.push((InterpolationOperation::EndBlockStart,get_position(start, end)));

    while let Some((position,char)) = chars.next() {
        end = position;
        match char {
            '\n' => {
                let use_sub = if let Some((_,'•')) = chars.peek() {
                    chars.next();
                    true
                } else {
                    false
                };
                let new_string = std::mem::replace(&mut current_string, String::new());
                operations.push((InterpolationOperation::CreateString(Rc::from(new_string)),get_position(start, end)));
                start = end;
                operations.push((InterpolationOperation::Append,get_position(start, end)));
                if use_sub {
                    operations.push((InterpolationOperation::StartListItem,get_position(start, end)));
                } else {

                    operations.push((InterpolationOperation::StartParagraph,get_position(start, end)));
                }
                operations.push((InterpolationOperation::EndBlockStart,get_position(start, end)));
            },
            '*' => {
                if let Some((_,'*')) = chars.peek() {
                    chars.next();
                    let new_string = std::mem::replace(&mut current_string, String::new());
                    operations.push((InterpolationOperation::CreateString(Rc::from(new_string)),get_position(start,end)));
                    start = end;
                    operations.push((InterpolationOperation::Append,get_position(start,end)));
                    if bold {
                        operations.push((InterpolationOperation::EndBold,get_position(start,end)));
                        bold = false;
                    } else {
                        operations.push((InterpolationOperation::StartBold,get_position(start,end)));
                        bold = true;
                    }
                } else {
                    current_string.push('*');
                }
            },
            '_' => {
                if let Some((_,'_')) = chars.peek() {
                    chars.next();
                }
                let new_string = std::mem::replace(&mut current_string, String::new());
                operations.push((InterpolationOperation::CreateString(Rc::from(new_string)),get_position(start,end)));
                start = end;
                operations.push((InterpolationOperation::Append,get_position(start,end)));
                if italic {
                    operations.push((InterpolationOperation::EndItalic,get_position(start,end)));
                    italic = false;
                } else {
                    operations.push((InterpolationOperation::StartItalic,get_position(start,end)));
                    italic = true;
                }
            },
            c => {
                current_string.push(c);
            }
        }
    };

    operations.push((InterpolationOperation::CreateString(Rc::from(current_string)),get_position(start, end)));
    start = end;
    operations.push((InterpolationOperation::Append,get_position(start, end)));

    let document = Document {
        operations
    };

    match document.interpolate(source_name,&()) {
        Ok(value) => Ok(value),
        Err(err) => if show_text_in_error {
            Err(err.with_full_text(source))
        } else {
            Err(err)
        }
    }



}