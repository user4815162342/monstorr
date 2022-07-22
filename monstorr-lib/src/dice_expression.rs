/*
 * Copyright © 2022 Neil M. Sheldon
 * 
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::iter::Peekable;

use serde::Deserialize;
use serde::Serialize;
use serde::Serializer;

use crate::tokenizer::Token;
use crate::errors::TokenError;
use crate::tokenizer::Tokenizer;
use crate::dice::Die;
use crate::dice::Dice;
use crate::digit;
use crate::utils::NextOk;
use crate::utils::CeilingDiv;
use crate::utils::FloorDiv;

/*
Structures for a basic dice expression formula, since we can't simplify dice to a single number. These dice expressions allow for:
- add dice together (D+D)
- subtract dice from another dice (D-D)
- add dice and numbers (D+x)
- subtract numbers from dice (D-x)
- subtract dice from numbers (x-D = -D + x)
- multiply dice and numbers (D*x)
- divide dice by numbers (D/x = D * 1/x)

All of these operations can be reduced to the following rough formula:
(dice(1) * factor) + (dice(2) * factor) ... (dice(n) * factor) + addend

Operations between numbers can be computed and reduced to a single number terms.

The following are not available, as they make life way too complicated, and can't be reduced to the formula mentioned above
- multiply dice together (D*D)
- divide dice from dice or numbers (D/D, x/D)

For more information on how this will work, see notes on proofs at the bottom of this module. If I'm incorrect about any of that, please let me know.
*/



#[derive(PartialEq,Clone,Debug)]
struct FactoredDice {
    dice: Dice, 
    factor: isize
}

impl FactoredDice {

    fn serialize_to_string(&self,as_predicate: bool) -> String {
        if as_predicate {
            match self.factor {
                -1 => format!(" - {}",self.dice),
                1 => format!(" + {}",self.dice),
                factor => format!(" + ({} × {})",self.dice,factor)
            }
        } else {
            match self.factor {
                -1 => format!("-{}",self.dice),
                1 => self.dice.to_string(),
                factor => format!("({} × {})",self.dice,factor)
            }
        }

    }

    fn average(&self) -> f64 {
        let average = self.dice.average();
        (average as f64) * (self.factor as f64)
    }

    fn multiply(&self, factor: &isize) -> Self {
        FactoredDice {
            dice: self.dice.clone(),
            factor: self.factor * factor
        }
    }

    fn div_ceiling(&self, factor: &isize) -> Self {
        FactoredDice {
            dice: self.dice.clone(),
            factor: self.factor.div_ceiling(factor)
        }
    }

    fn div_floor(&self, factor: &isize) -> Self {
        FactoredDice {
            dice: self.dice.clone(),
            factor: self.factor.nms_div_floor(factor)
        }
    }

    fn is_negative(&self) -> bool {
        self.factor < 0
    }
}


pub enum ParseDiceExpressionError {
    ScanError(TokenError),
    ExpectedNumberAfterMinus,
    ExpectedNumberAfterMultiply,
    ExpectedMultiplySymbolAfterDiceInParens,
    ExpectedDiceAfterOpenParen,
    ExpectedCloseParen,
    ExpectedDiceAfterFirstMinus,
    ExpectedMinusParenOrDiceAsFirstToken,
    UnexpectedContentAfterAddend,
    ExpectedNumberOrDiceAfterMinus,
    ExpectedNumberDiceOrParenAfterPlus,
    ExpectedPlusOrMinusAfterTerm,
}

impl From<TokenError> for ParseDiceExpressionError {

    fn from(error: TokenError) -> Self {
        ParseDiceExpressionError::ScanError(error)
    }
}

impl std::fmt::Display for ParseDiceExpressionError {

    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(),std::fmt::Error> {
        match self {
            ParseDiceExpressionError::ScanError(err) => write!(f,"Tokenizing error: {}",err),
            ParseDiceExpressionError::ExpectedNumberAfterMinus => write!(f,"Expected number after minus in factor"),
            ParseDiceExpressionError::ExpectedNumberAfterMultiply => write!(f,"Expected number after multiply symbol in factor"),
            ParseDiceExpressionError::ExpectedMultiplySymbolAfterDiceInParens => write!(f,"Expected multiply symbol after dice in factored dice"),
            ParseDiceExpressionError::ExpectedDiceAfterOpenParen => write!(f,"Expected dice term after open parenthesis in factored dice"),
            ParseDiceExpressionError::ExpectedCloseParen => write!(f,"Expected closing parenthesis in factored dice"),
            ParseDiceExpressionError::ExpectedDiceAfterFirstMinus => write!(f,"Expected dice after initial minus"),
            ParseDiceExpressionError::ExpectedMinusParenOrDiceAsFirstToken => write!(f,"Expected minus, open parenthesis or dice term as first token"),
            ParseDiceExpressionError::UnexpectedContentAfterAddend => write!(f,"No content is allowed after the final addend"),
            ParseDiceExpressionError::ExpectedNumberOrDiceAfterMinus => write!(f,"Expected number or dice after minus symbol in term"),
            ParseDiceExpressionError::ExpectedNumberDiceOrParenAfterPlus => write!(f,"Expected number, dice or open parenthesis after plus symbol in term"),
            ParseDiceExpressionError::ExpectedPlusOrMinusAfterTerm => write!(f,"Only plus or minus operators are allowed after each term"),
        
        }
   }
}



#[derive(PartialEq,Clone,Debug)]
#[derive(Deserialize)]
#[serde(try_from = "String")]
pub struct DiceExpression {
    head: FactoredDice,
    medial: Vec<FactoredDice>,
    addend: isize
}

impl DiceExpression {

    pub fn from_dice(dice: Dice, addend: isize) -> Self {
        Self {
            head: FactoredDice {
                dice,
                factor: 1
            },
            medial: Vec::new(),
            addend
        }
    }

    pub fn serialize_to_string(&self) -> String {
        let mut result = String::new();
        result.push_str(&self.head.serialize_to_string(false));
        for factored in &self.medial {
            result.push_str(&factored.serialize_to_string(true));
        }
        if self.addend < 0 {
            result.push_str(&format!(" - {}",-self.addend))
        } else if self.addend > 0 {
            result.push_str(&format!(" + {}",self.addend))
        }
        result
    }

    pub fn average(&self) -> isize {
        let mut average = self.head.average();
        for factored in &self.medial {
            average += factored.average();
        }
        average += self.addend as f64;
        average.floor() as isize
    }

    pub fn display_with_alternate_average(&self, average: Option<isize>) -> String {
        format!("{} ({})",average.unwrap_or(self.average()),&self.serialize_to_string())
    }

    pub fn multiply(&self, factor: &isize) -> Self {
        // distribute and multiply against each thing...
        let head = self.head.multiply(factor);
        let medial = self.medial.iter().map(|a| a.multiply(factor)).collect();
        let addend = self.addend * factor;
        Self {
            head,
            medial,
            addend
        }
    }

    pub fn div_ceiling(&self, factor: &isize) -> Self {
        // distribute and multiply
        let head = self.head.div_ceiling(factor);
        let medial = self.medial.iter().map(|a| a.div_ceiling(factor)).collect();
        let addend = self.addend.div_ceiling(factor);
        Self {
            head,
            medial,
            addend
        }
    }

    pub fn div_floor(&self, factor: &isize) -> Self {
        // distribute and multiply
        let head = self.head.div_floor(factor);
        let medial = self.medial.iter().map(|a| a.div_floor(factor)).collect();
        let addend = self.addend.nms_div_floor(factor);
        Self {
            head,
            medial,
            addend
        }
    }

    pub fn add(&self, addend: &isize) -> Self {
        Self {
            head: self.head.clone(),
            medial: self.medial.clone(),
            addend: self.addend + addend
        }
    }

    fn add_dice_factor(&mut self, dice: &FactoredDice) {
        if (self.head.dice.die == dice.dice.die) &&
           (self.head.factor == dice.factor) {
            self.head.dice.coefficient += dice.dice.coefficient;
        } else {
            for factored in &mut self.medial {
                if (factored.dice.die == dice.dice.die) &&
                   (factored.factor == dice.factor) {
                    factored.dice.coefficient += dice.dice.coefficient;
                    return;
                }
            }
            self.medial.push(dice.clone());
        }
    }

    pub fn add_dice(&self, dice: &DiceExpression) -> Self {
        let mut result = self.clone();
        result.add_dice_factor(&dice.head);
        for factored in &dice.medial {
            result.add_dice_factor(&factored);
        }
        result.addend += dice.addend;
        result
    }

    // Used by the 'brute' feature. I'm not absolutely certain whether this operation should be distributed
    // among the medial factors as well. FUTURE: If we could get attacks to just use 'Dice', than it would be much
    // simpler, but then we have to have a separate 'magic' property on the Attacks. This might not be a bad
    // idea, since that can easily be optional if 0, but it makes things wordier.
    // Revisit this if I ever create an actual parser for the Monstor creature format instead of just using serde, 
    // which would allow me more control over skipping optional arguments in a tuple.
    pub fn coefficient_add(&self, count: u8) -> Self {
        let mut result = self.clone();
        let mut new_dice = self.head.clone();
        new_dice.dice.coefficient = count;
        result.add_dice_factor(&new_dice);
        result
    }

    pub fn subtract(&self, addend: &isize) -> Self {
        Self {
            head: self.head.clone(),
            medial: self.medial.clone(),
            addend: self.addend - addend
        }
    }

    pub fn subtract_dice(&self, dice: &DiceExpression) -> Self {
        let head = self.head.clone();
        let mut medial = self.medial.clone();
        medial.push(dice.head.clone().multiply(&-1));
        medial.append(&mut dice.medial.iter().map(|a| a.multiply(&-1)).collect());
        Self {
            head,
            medial,
            addend: self.addend - dice.addend
        }
    }

    pub fn is_negative(&self) -> bool {
        self.head.is_negative()
    }


}

impl From<Dice> for DiceExpression {

    fn from(dice: Dice) -> Self {
        Self {
            head: FactoredDice {
                dice,
                factor: 1
            },
            medial: vec![],
            addend: 0
        }
    }

}

impl From<Die> for DiceExpression {

    fn from(die: Die) -> Self {
        Self {
            head: FactoredDice {
                dice: die.into(),
                factor: 1
            },
            medial: vec![],
            addend: 0
        }
    }

}

struct DiceExpressionTokenizer<Source: Iterator<Item=char>>(Peekable<Source>);


impl<Source: Iterator<Item=char>> Tokenizer<()> for DiceExpressionTokenizer<Source> {

    fn resolve_keyword(&self, _: &str) -> Option<()> {
        None
    }


    fn next_char(&mut self) -> Option<char> {
        self.0.next()
    }

    fn peek_char(&mut self) -> Option<&char> {
        self.0.peek()
    }

} 

impl<Source: Iterator<Item=char>> DiceExpressionTokenizer<Source> {

    fn new(source: Source) -> Self {
        Self(source.peekable())
    }

    fn parse_factored_dice(&mut self) -> Result<FactoredDice,ParseDiceExpressionError> {
        // factored_dice = '(' dice '×' '-'? number ')'
        // we already have the '('
        let result = match self.next_ok()? {
            Some(Token::Dice(dice)) => {

                if let Some(Token::Asterisk) = self.next_ok()? {
                    let factor = match self.next_ok()? {
                        Some(Token::Minus) => match self.next_ok()? {
                            Some(Token::Number(num)) => -num, 
                            _ => Err(ParseDiceExpressionError::ExpectedNumberAfterMinus)?
                        },
                        Some(Token::Number(num)) => num, 
                        _ => Err(ParseDiceExpressionError::ExpectedNumberAfterMultiply)?
                    };
                    FactoredDice {
                        dice,
                        factor
                    }
    
                } else {
                    Err(ParseDiceExpressionError::ExpectedMultiplySymbolAfterDiceInParens)?
                }


            },
            _ => Err(ParseDiceExpressionError::ExpectedDiceAfterOpenParen)? 
        };

        if let Some(Token::CloseParenthesis) = self.next_ok()? {
            Ok(result)
        } else {
            Err(ParseDiceExpressionError::ExpectedCloseParen)
        }
    }

    fn parse(&mut self) -> Result<DiceExpression,ParseDiceExpressionError> {
        // dice_expression = head predicate?
        // head = ('-' dice) | dice | factored_dice
        // predicate = '-' number | (dice predicate?)
        //             '+' number | (dice predicate?) | (factored_dice predicate?)
        // factored_dice = '(' dice '×' '-'? number ')'


        // head = ('-' dice) | dice | factored_dice
        let head = match self.next_ok()? {
            Some(Token::Minus) => match self.next_ok()? {
                Some(Token::Dice(dice)) => FactoredDice {
                    dice,
                    factor: -1
                }, 
                _ => Err(ParseDiceExpressionError::ExpectedDiceAfterFirstMinus)?, 
            },
            Some(Token::Dice(dice)) => FactoredDice {
                dice,
                factor: 1
            }, 
            Some(Token::OpenParenthesis) => self.parse_factored_dice()?, 
            _ => Err(ParseDiceExpressionError::ExpectedMinusParenOrDiceAsFirstToken)? 
        };

        let mut medial = vec![];

        let addend = loop {
            // predicate = '-' number | (dice predicate?)
            //             '+' number | (dice predicate?) | (factored_dice predicate?)
            match self.next_ok()? {
                Some(Token::Minus) => match self.next_ok()? {
                    Some(Token::Number(num)) => match self.next_ok()? {
                        None => break -num, // return the addend
                        _ => Err(ParseDiceExpressionError::UnexpectedContentAfterAddend)?, 
                    },
                    Some(Token::Dice(dice)) => {
                        medial.push(FactoredDice {
                            dice,
                            factor: -1
                        })
                    }, // continue parsing more terms
                    _ => Err(ParseDiceExpressionError::ExpectedNumberOrDiceAfterMinus)?, 
                },
                Some(Token::Plus) => match self.next_ok()? {
                    Some(Token::Number(num)) => match self.next_ok()? {
                        None => break num, // return the addend
                        _ => Err(ParseDiceExpressionError::UnexpectedContentAfterAddend)?, 
                    },
                    Some(Token::Dice(dice)) => {
                        medial.push(FactoredDice {
                            dice,
                            factor: 1
                        })
                    }, // continue parsing more terms
                    Some(Token::OpenParenthesis) => {
                        medial.push(self.parse_factored_dice()?)
                        // but continue parsing more terms
                    },
                    _ => Err(ParseDiceExpressionError::ExpectedNumberDiceOrParenAfterPlus)?, 
                },
                None => break 0, // no further predicate found, so the addend must be 0
                _ => Err(ParseDiceExpressionError::ExpectedPlusOrMinusAfterTerm)?, 
            }
        };
        
        Ok(DiceExpression {
            head,
            medial,
            addend
        })

    }    

}

impl<Source: Iterator<Item=char>> Iterator for DiceExpressionTokenizer<Source> {

    type Item = Result<Token<()>,TokenError>;

    fn next(&mut self) -> Option<Self::Item> {

        self.skip_whitespace();

        match self.next_char() {
            Some(char) if matches!(char,digit!()) => Some(self.number_or_dice(char)),
            Some('+') => Some(Ok(Token::Plus)),
            Some('-') => Some(Ok(Token::Minus)),
            Some('×') => Some(Ok(Token::Asterisk)), // it's an alias so the display looks better
            Some('*') => Some(Ok(Token::Asterisk)),
            Some('(') => Some(Ok(Token::OpenParenthesis)),
            Some(')') => Some(Ok(Token::CloseParenthesis)),
            Some(_) => Some(Err(TokenError::UnexpectedCharacter)),
            None => None
        }


    }

}



impl std::fmt::Display for DiceExpression {

    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(),std::fmt::Error> {
        write!(f,"{} ({})",self.average(),&self.serialize_to_string())
   }
}


impl std::str::FromStr for DiceExpression {
    type Err = ParseDiceExpressionError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {

        DiceExpressionTokenizer::new(s.chars()).parse()

    }
}

impl std::convert::TryFrom<String> for DiceExpression {

    type Error = ParseDiceExpressionError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()

    }

}

impl Serialize for DiceExpression {

    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.serialize_to_string())
    }
}


/*
DICE MATH LAWS AND PROOFS:
(For how I came up with the algorithms and structures above)
Given:
1. A *die* represents a random number generator that can generate consecutive integral numbers from 1 up to and including a specified positive whole number. The notation 'D' <integer> is used to represent a die, where the integer is the highest number in the range. 
2. Each die of a given number is unique and won't be simplified for any purposes.
3. A *dice* represents a die rolled a certain number of times.  The notation <integer> <die> indicates a dice consisting of the specified die rolled the <integer> number of times.
4. Terms in a dice formula consist of integers and dice, as defined above.
4. Arbitrarily, two or more dice terms can not be multiplied together. This is an operation which is not expected to be used or useful, so will not be considered in our proofs, as it complicates the possibilities.
5. Similarly, an expression can not include a division by a dice term.
6. Otherwise, the four basic binary arithmetic operations, addition, subtraction, multiplication and division will be supported, as well as the unary operation negation.
Prove:
1. Any dice expression can be written in the form (<dice> * <number>) + (<dice> * <number>) + ... + <number>

Sort of Proof:
1. any operation consisting of two number terms can be reduced to a single number.
2. A negated number can be reduced to a single number.
3. The following possible combinations exist, where <dice-expression> represents any possible <dice-expression> which will be supported.
   a. <dice> + <number>
   b. <dice> - <number>
   c. <dice> * <number>
   d. <dice> / <number>
   e. <number> + <dice>
   f. <number> - <dice>
   g. <number> * <dice>
   h. <number> / <dice>
   i. <dice-expression> + <number>
   j. <dice-expression> - <number>
   k. <dice-expression> * <number>
   l. <dice-expression> / <number>
   m. <number> + <dice-expression>
   n. <number> - <dice-expression>
   o. <number> * <dice-expression>
   p. <number> / <dice-expression>
   q. <dice-expression> + <dice>
   r. <dice-expression> - <dice>
   s. <dice-expression> * <dice>
   t. <dice-expression> / <dice>
   u. <dice> + <dice-expression>
   v. <dice> - <dice-expression>
   w. <dice> * <dice-expression>
   x. <dice> / <dice-expression>
   y. <dice> + <dice>
   z. <dice> - <dice>
   aa. <dice> * <dice>
   ab. <dice> / <dice>
   ac. <dice-expression> + <dice-expression>
   ad. <dice-expression> - <dice-expression>
   ae. <dice-expression> * <dice-expression>
   af. <dice-expression> / <dice-expression>
4. Because we can not allow dividing by a dice term or multiplying two or more dice terms. The combinations (h), (t), (aa) and (ab) are not allowed. In addition, as a dice-expression by definition includes a dice term, the combinations (p), (s), (w), (x), (ae) and (af) do not exist. Meaning we can narrow the combinations down to the following:
   a. <dice> + <number>
   b. <dice> - <number>
   c. <dice> * <number>
   d. <dice> / <number>
   e. <number> + <dice>
   f. <number> - <dice>
   g. <number> * <dice>
   h. <dice-expression> + <number>
   i. <dice-expression> - <number>
   j. <dice-expression> * <number>
   k. <dice-expression> / <number>
   l. <number> + <dice-expression>
   m. <number> - <dice-expression>
   n. <number> * <dice-expression>
   o. <dice-expression> + <dice>
   p. <dice-expression> - <dice>
   q. <dice> + <dice-expression>
   r. <dice> - <dice-expression>
   s. <dice> + <dice>
   t. <dice> - <dice>
   u. <dice-expression> + <dice-expression>
   v. <dice-expression> - <dice-expression>
5. Any subtraction operation can be converted to a addition operation with the second term negated. Similarly, any division operation can be converted to a multiplication operation with the second term reciprocated. Therefore, the combinations become:
   a. <dice> + <number>
   b. <dice> + -<number>
   c. <dice> * <number>
   d. <dice> * 1/<number>
   e. <number> + <dice>
   f. <number> + -<dice>
   g. <number> * <dice>
   h. <dice-expression> + <number>
   i. <dice-expression> + -<number>
   j. <dice-expression> * <number>
   k. <dice-expression> * 1/<number>
   l. <number> + <dice-expression>
   m. <number> + -<dice-expression>
   n. <number> * <dice-expression>
   o. <dice-expression> + <dice>
   p. <dice-expression> + -<dice>
   q. <dice> + <dice-expression>
   r. <dice> + -<dice-expression>
   s. <dice> + <dice>
   t. <dice> + -<dice>
   u. <dice-expression> + <dice-expression>
   v. <dice-expression> + -<dice-expression>
6. A negative number is still a number, and the reciprocal of a number is still a number, therefore the following operations are duplicates (b), (d), (i), (k)
   a. <dice> + <number>
   b. <dice> * <number>
   c. <number> + <dice>
   d. <number> + -<dice>
   e. <number> * <dice>
   f. <dice-expression> + <number>
   g. <dice-expression> * <number>
   h. <number> + <dice-expression>
   i. <number> + -<dice-expression>
   j. <number> * <dice-expression>
   k. <dice-expression> + <dice>
   l. <dice-expression> + -<dice>
   m. <dice> + <dice-expression>
   n. <dice> + -<dice-expression>
   o. <dice> + <dice>
   p. <dice> + -<dice>
   q. <dice-expression> + <dice-expression>
   r. <dice-expression> + -<dice-expression>
7. Because addition and multiplication terms can be switched, some of the forms above are now duplicates.
   a. <dice> + <number>
   b. <dice> * <number>
   c. <number> + -<dice>
   d. <dice-expression> + <number>
   e. <dice-expression> * <number>
   f. <number> + -<dice-expression>
   g. <dice-expression> + <dice>
   h. <dice-expression> + -<dice>
   i. <dice> + -<dice-expression>
   j. <dice> + <dice>
   k. <dice> + -<dice>
   l. <dice-expression> + <dice-expression>
   m. <dice-expression> + -<dice-expression>
8. A negative <dice> term or <dice-expression> term can be represented as the same term multiplied by -1
   a. <dice> + <number>
   b. <dice> * <number>
   c. <number> + <dice> * -1
   d. <dice-expression> + <number>
   e. <dice-expression> * <number>
   f. <number> + <dice-expression> * -1
   g. <dice-expression> + <dice>
   h. <dice-expression> + <dice> * -1
   i. <dice> + <dice-expression> * -1
   j. <dice> + <dice>
   k. <dice> + <dice> * -1
   l. <dice-expression> + <dice-expression>
   m. <dice-expression> + <dice-expression> * -1
9. These -1's are just numbers, which simplifies those forms a bit more.
   a. <dice> + <number>
   b. <dice> * <number>
   c. <number> + <dice> * <number>
   d. <dice-expression> + <number>
   e. <dice-expression> * <number>
   f. <number> + <dice-expression> * <number>
   g. <dice-expression> + <dice>
   h. <dice-expression> + <dice> * <number>
   i. <dice> + <dice-expression> * <number>
   j. <dice> + <dice>
   k. <dice> + <dice> * <number>
   l. <dice-expression> + <dice-expression>
   m. <dice-expression> + <dice-expression> * <number>
10. Any number equals that same number plus 0, which allows us to rewrite some of the multiplicative forms above:
   a. <dice> + <number>
   b. <dice> * <number> + <number>
   c. <number> + <dice> * <number>
   d. <dice-expression> + <number>
   e. <dice-expression> * <number> + <number>
   f. <number> + <dice-expression> * <number>
   g. <dice-expression> + <dice>
   h. <dice-expression> + <dice> * <number>
   i. <dice> + <dice-expression> * <number>
   j. <dice> + <dice>
   k. <dice> + <dice> * <number>
   l. <dice-expression> + <dice-expression>
   m. <dice-expression> + <dice-expression> * <number>
11. Similarly, any number equals that same number times 1, allowing us to rewrite some of the additve forms:
   a. <dice> * <number> + <number>
   b. <dice> * <number> + <number>
   c. <number> + <dice> * <number>
   d. <dice-expression> * <number> + <number>
   e. <dice-expression> * <number> + <number>
   f. <number> + <dice-expression> * <number>
   g. <dice-expression> + <dice> * <number>
   h. <dice-expression> + <dice> * <number>
   i. <dice> + <dice-expression> * <number>
   j. <dice> + <dice> * <number>
   k. <dice> + <dice> * <number>
   l. <dice-expression> + <dice-expression> * <number>
   m. <dice-expression> + <dice-expression> * <number>
12. This allows us to remove a bunch of duplicates
   a. <dice> * <number> + <number>
   b. <dice-expression> * <number> + <number>
   c. <dice-expression> + <dice> * <number>
   d. <dice> + <dice-expression> * <number>
   e. <dice> + <dice> * <number>
   f. <dice-expression> + <dice-expression> * <number>
13. There are only two forms above that are simplified down to only dice and numbers, these become the basis of a simple dice expression
   a. <dice> * <number> + <number>
   e. <dice> + <dice> * <number>
14. We can add identity factors and addends to these.
   a. <dice> * <number> + <number>
   e. <dice> * 1 + 0 + <dice> * <number> + 0
15. Which creates the following:
   a. <dice> * <number> + <number>
   e. <dice> * <number> + <number> + <dice> * <number> + <number>
16. Further reduction:
   a. <dice> * <number> + <number>
   e. <dice> * <number> + <dice> * <number> + <number>
17. Let's take the first one as the most basic expression. Whenever we add a number to that, it can be collapsed into the number term.
   <dice> * <number> + <number> + <number> -> <dice> * <number> + <number>
18. Similarly, whenever we multiple the term by a number, after distribution, it can be collapsed into the other number terms.
   (<dice> * <number> + <number>) * <number> -> <dice> * <number> * <number> + <number> * <number> -> <dice> * <number> + <number>
19. When we add two such dice expressions together, we can collapse the addend number, but not the multiplicative temrs
   (<dice> * <number> + <number>) + (<dice> * <number> + <number>) -> <dice> * <number> + <number> + <dice> * <number> + <number> -> <dice> * <number> + <dice> * <number> + <number>
20. As we continue to add more such expressions together, the addends will collapse, but the multiplications will extend, leading to the simple
    formula:
    <dice> * <number> + (<dice> + <number>)... + <number>
    -- Note, we could simplify that number at the end to <dice> * 0 + <number>, but that's just weird
21. Thus, if we revisit the combinations above, replacing dice-expression with that formula:
   a. (<dice> * <number> + (<dice> + <number>)... + <number>) * <number> + <number>
   b. (<dice> * <number> + (<dice> + <number>)... + <number>) + <dice> * <number>
   c. <dice> + (<dice> * <number> + (<dice> + <number>)... + <number>) * <number>
   d. (<dice> * <number> + (<dice> + <number>)... + <number>) + (<dice> * <number> + (<dice> + <number>)... + <number>) * <number>
22. Then distribute the factors and ungroup:
   a. <dice> * <number> * <number> * <number> + (<dice> * <number> + <number> * <number>)... + <number> * <number> + <number>
   b. <dice> * <number> + (<dice> + <number>)... + <number> + <dice> * <number>
   c. <dice> + <dice> * <number> * <number> * <number> + (<dice> * <number> + <number> * <number>)... + <number> * <number>
   d. <dice> * <number> + (<dice> + <number>)... + <number> + <dice> * <number> * <number> * <number> + (<dice> * <number> + <number> * <number>)... + <number> * <number>
23. And reduce the numeric terms:
   a. <dice> * <number> + (<dice> * <number>)... + <number>
   b. <dice> * <number> + (<dice> + <number>)... + <dice> * <number> + <number> 
   c. <dice> * 1 + <dice> * <number> + (<dice> * <number>)... + <number>
   d. <dice> * <number> + (<dice> + <number>)... + <dice> * <number> + (<dice> * <number>)... + <number>
24. And simplify
   a. <dice> * <number> + (<dice> * <number>)... + <number>
   b. <dice> * <number> + (<dice> + <number>)... + <number> 
   c. <dice> * <number> + (<dice> * <number>)... + <number>
   d. <dice> * <number> + (<dice> + <number>)... + <number>
25. We get the same formulat for each one
   <dice> * <number> + (<dice> + <number>)... + <number>
*/