/*
 * Copyright © 2022 Neil M. Sheldon
 * 
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::collections::HashMap;

use minijinja::Environment;
use minijinja::meta::find_referenced_templates;
use monstorr_data::templates::StoredTemplates;

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

pub trait TemplateSourceResolver {

    fn get_template(&self, name: &str) -> Result<Option<String>,String>;
    
}

impl TemplateSourceResolver for () {

    fn get_template(&self, _name: &str) -> Result<Option<String>,String> {
        Ok(None)
    }
}

impl TemplateSourceResolver for StoredTemplates {

    fn get_template(&self, name: &str) -> Result<Option<String>,String> {
        Ok(self.get(name))
    }


}





fn resolve_templates<Resolver: TemplateSourceResolver>(resolver: &Resolver, resolved: &mut HashMap<String,String>, name: &str) -> Result<(),String> {
    // if this one is already resolved, I don't need to do anything
    if let Some(_) = resolved.get(name) {
        Ok(())
    } else if let Some(source) = resolver.get_template(name)? {
        // add it to resolved now, even though it's not quite resolved yet, to avoid infinite recursion
        resolved.insert(name.to_owned(),source.to_owned());

        let other_templates = find_referenced_templates(&source).map_err(|e| format!("Error resolving included templates: '{}'",e))?;

        for template in other_templates {
            resolve_templates(resolver, resolved, &template)?
        }
        Ok(())  
    } else {
        Err(format!("Could not resolve template '{}'",name))?
    }

}


pub fn process_template<Resolver: TemplateSourceResolver>(resolver: &Resolver, template: &str, includes: &Vec<String>, stat_block: &CreatureStatBlock) -> Result<String,String> {

    let mut resolved_templates = HashMap::new();
    resolve_templates(resolver, &mut resolved_templates, &template)?;

    // resolve additional included templates
    for include in includes {
        resolve_templates(resolver, &mut resolved_templates, include)?;
    }

    let mut env = Environment::new();
        
    //let resolved_templates = resolved_templates.into_iter().collect::<Vec<(String,String)>>();

    // I need to keep the iterator in scope, or I get lifetime errors when adding the template...
    let resolved_templates = resolved_templates.iter();
    for (name,source) in resolved_templates {
        env.add_template(&name, &source).map_err(|e| format!("Error parsing template '{}': {}",name,e))?
    }

    let template = env.get_template(&template).map_err(|e| format!("Error parsing template '{}': {}",template,e))?;
    template.render(stat_block).map_err(|e| format!("Template error: {}",e))

}