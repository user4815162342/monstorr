/*
 * Copyright © 2022 Neil M. Sheldon
 * 
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use minijinja::Environment;

use crate::stat_block::CreatureStatBlock;


/*
FUTURE: My own template parser, which uses the same tokenizer as everything else here.
- benefits:
  - it looks like the rest of the syntax
  - it has macros, unlike minininja (but is still just as easy to use)
  - it can be designed to resolve the external includes as needed, instead of having to add them at the beginning. (if I even need those, given macros)
  - the syntax is more expressive because I don't think of it as tags, but more like interpolation like JavaScript template strings
  - this also allows me better control over whitespace. If I think of text as expressions starting with '}' and ending with '${' then it's easier
    to lay out the statements in line below

template = statement*

statement = <text> | control | expression-statement

control = if-statement | switch-statement | for-statement | macro-statement | extend | placeholder | include | with | filter

if-statement = 'if' expression 'then' statement* ('elif' statement*)* ('else' sstatement*)? 'end'

match-statement = 'switch' expression ('case' expression 'then' statement*) ('else' statement*)? 'end'

for-statement = 'for' <identifier> (('in' expression)|('from' expression 'to' expression ('step' expression)?)) 'do' statement* ('else' statement*)? 'end'

macro-statement = 'macro' <identifier> ':' statement* 'end'
-- a macro is just an external template written inside another, so it creates a new template with the specified name.
-- macros can be called with include or extends

extend = 'extend' expression ('replace' <identifier> 'with' statement*)* 'end'

placeholder = 'placeholder' <identifier> ':' statement* 'end'
- in the context of a macro or template, this specifies placeholders and their default content

include = 'include' expression 'with' expression
- the first expression is either a template name or a macro identifier
- the second expression is the context data to pass to the template

with = 'with' expression 'do' statement* 'end'
- the expression becomes the new context for this block

filter = 'apply' <identifier> 'to' statement* 'end'
- applies the specified filter to every string appended to the result for the duration. A filter is basically a method that can be applied to a string.

expression-statement = expression ';'?

expression = <text> | conditional-expression
-- note that text is treated as a string in an expression instead of automatically appending. A bare expression-statement will never see this variant, since it will be caught as a statement, but in parenthesis or other situation, it will happen

conditional-expression = logical-or ('then' expression)? ('else' expression)?
-- if the initial expression is truthy, calls the then expression, otherwise the else expression
-- if there is no then expression, then returns the value of the subject
-- if there is no else expression, still it returns the value of the subject

logical-or = logical-and ('or' logical-and)*

logical-and = equality ('and' equality)*

equality = comparison (('=' | '<>' | '!=') comparison)

comparison = term = ((('>' | '>=' | '<' | '<=') term) | ('is' 'not'? term) | ('in' term) | ('not' 'in' term))

term = factor (('-' | '+') factor)*

factor = unary (('/' | '//' | '/<' | '/>' | '%' | '*' | '**') unary)*

unary = (('not' | '-' | '$' | '+' ) unary) | call

call = primary (('(' arguments? ')' ) | ('[' expression ']' ) | (('.' | '?.' ) identifier))*
-- the ?. returns none if the property doesn't exist on the previous.

arguments = expression (',' expression)*

primary = 'true' | 'false' | 'none' | <number> | <string> | <identifier> | list | '(' expression ')'

list = '[' arguments? ']'


*/

pub struct TemplateData {
    name: String,
    content: String,
}

impl From<&(String,String)> for TemplateData {

    fn from(data: &(String,String)) -> Self {
        TemplateData {
            name: data.0.clone(),
            content: data.1.clone()
        }
    }
}

impl From<(String,String)> for TemplateData {

    fn from(data: (String,String)) -> Self {
        TemplateData {
            name: data.0,
            content: data.1
        }
    }
}

impl From<&(&str,&str)> for TemplateData {

    fn from(data: &(&str,&str)) -> Self {
        TemplateData {
            name: data.0.to_owned(),
            content: data.1.to_owned()
        }
    }

}

impl From<(&str,&str)> for TemplateData {

    fn from(data: (&str,&str)) -> Self {
        TemplateData {
            name: data.0.to_owned(),
            content: data.1.to_owned()
        }
    }

}

pub fn process_template(template: TemplateData, includes: Vec<TemplateData>, stat_block: &CreatureStatBlock) -> Result<String,String> {

    let mut env = Environment::new();
    env.add_template(&template.name, &template.content).map_err(|e| format!("Error parsing template '{}': {}",template.name,e))?;
    for i in 0..includes.len() {
        env.add_template(&includes[i].name, &includes[i].content).map_err(|e| format!("Error parsing template '{}': {}",includes[i].name,e))?
    }

    let template = env.get_template(&template.name).map_err(|e| format!("Error parsing template '{}': {}",template.name,e))?;
    template.render(stat_block).map_err(|e| format!("Template error: {}",e))

}