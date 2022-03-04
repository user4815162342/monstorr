/*
 * Copyright © 2022 Neil M. Sheldon
 * 
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

/*

Prior Work:
  - This online stat-block generator: https://tetra-cube.com/dnd/dnd-statblock.html, which has code shared on github: https://github.com/Tetra-cube/Tetra-cube.github.io 
  - https://www.rpgtinker.com/ - A NPC generator. While not obviously open source, when it creates a "challenge rating" the question mark provides a whole bunch of useful information about how that CR was generated, specific to the NPC 
  - https://iadndmn.neocities.org/CRcalc.html - Also not OS. 
  - https://open5e.com/ - Mostly open source, this provides the data for freely available monsters. Other monsters would have to be copied.
  - https://valloric.github.io/statblock5e/ - for HTML template

 */


// need this because some important documentation is in private code
#![allow(rustdoc::private_intra_doc_links)]
/* 
NOTE: I use block comments for documentation because they don't require an extra token at the beginning of every line. The extra token confuses my indents, are an extra bit of unnecessary typing, and frankly, look messy. I also don't use asterisks at the beginning of the doc comment lines for the same reasons, and I hate that my IDE insists that I do.
*/
/*!

This crate is the library for the functionality of the Monstorr program. It contains a chaotic good mess of structures and functionality for calculating D&D 5e creature stats and producing stat blocks in various formats. It's not currently intended as a public API, and any published functionality here is very unstable. However, some of the stuff in this crate is public API, therefore it needs documentation.

**Project Status.** This project is newly created and poorly tested. It works, but there may be weird issues, please report any that you find so I can fix them. While feature suggestions are also appreciated, they will probably not be addressed anytime soon. One great way to test this would be to convert the SRD monsters over to creature files, which could then be included in the monstorr-data crate.

# Points of Interest

There are a few things that you may need to know in order to simply use Monstorr.

* For the Monstorr command line tool usage, run `monstorr help`.
* For the creature file syntax, see [`crate::creature_commands`].
<!--* For templating information, see [`crate::template`], but for its syntax use [MiniJinja](https://docs.rs/minijinja/0.13.0/minijinja/syntax/index.html)
* For how to use this with the Open5e monster database, see [`crate::open5e_convertor`] as well as the separate `monstorr-open5e` crate.-->

# Development

**Organization and Coding Style.** I originally intended this as a creature database tool which would pull in Open5e-style JSON, verifying calculations, store them for later use, and spitting out LaTeX documents (for my own needs) as needed. Little thought was put into organization, formatting and keeping to the rusty ways of doing things. It was only later when I realized the true goal of this tool that I began to clean a few things up. I apologize for this confusion, you're welcome to fork this code and clean it up. Testing would be most appreciated.

 */
use std::fs;
use std::path::PathBuf;
use std::io::BufRead;
use std::io::Write;

use monstorr_open5e::Open5eMonster;
use monstorr_open5e::Open5eMonsterList;
use monstorr_data;

mod utils;
mod parse_position;
mod errors;
mod dice;
mod tokenizer;
mod dice_expression;
mod structured_text;
mod interpolation;
mod stats;
mod attacks;
mod actions;
mod reactions;
mod features;
mod spellcasting;
mod creature_commands;
mod stat_block;
mod creature;
mod open5e_convertor;
mod template;
#[cfg(test)] mod tests;


use crate::creature_commands::CreatureCreator;
use crate::stat_block::TryIntoStatBlock;
use crate::stats::ChallengeRating;
use crate::template::process_template;
use crate::utils::path_relative_from;

pub use creature_commands::MONSTORR_VERSION;

pub enum InputFormat {
    Creature,
    Open5e,
    Open5eList(String),
    Stored(String)
}

pub enum ListInputFormat {
    Open5eList
}

impl Default for InputFormat {

    fn default() -> Self {
        Self::Creature
    }
}


pub enum OutputFormat {
    JSON(bool), // whether to print ugly
    MiniJinjaTemplate(String,Vec<String>), // path to template, paths to templates to be included
    HTML(Option<usize>,bool) // an optional usize indicating that they want a two-column stat-block instead of one-column, a bool indicating that only a fragment should be output
}

impl Default for OutputFormat {

    fn default() -> Self {
        Self::JSON(false)
    }
}

fn resolve_file(working_dir: &PathBuf, file: &str) -> PathBuf {
    let mut result = working_dir.clone();
    result.push(file);
    result

}

fn resolve_existing_file(working_dir: &PathBuf, file: &str) -> Result<PathBuf,String> {
    let result = resolve_file(working_dir, file);
    if !result.is_file() {
        Err(format!("Path {} does not exist or is not a file",file))
    } else {
        Ok(result)
    }
}

fn get_default_working_dir() -> Result<PathBuf,String> {
    std::env::current_dir().map_err(|e| format!("Error getting current directory: {}",e))
}

fn get_working_dir_relative_to_source_or_default(source: &Option<PathBuf>, default_working_dir: &PathBuf) -> PathBuf {
    if let Some(source_file) = source {
        let mut working_dir = source_file.clone();
        working_dir.pop();
        working_dir
    } else {
        default_working_dir.clone()
    }
}

fn read_source(source_file: Option<&PathBuf>) -> Result<String,String> {
    if let Some(source_file) = source_file {
        fs::read_to_string(&source_file).map_err(|e| format!("Error reading input file: {}",e))
    } else {
        let mut buffer = String::new();
        let stdin = std::io::stdin();
        let stdin = stdin.lock();
        for line in stdin.lines() {
            buffer.push_str(&line.map_err(|e| format!("Error reading from stdin: {}",e))?)
        }
        Ok(buffer)
    }
}

fn write_target(target_file: Option<PathBuf>, content: &str) -> Result<(),String> {
    if let Some(target_file) = target_file {
        fs::write(target_file, content).map_err(|e| format!("Error writing file: {}",e))
    } else {
        std::io::stdout().write_all(content.as_bytes()).map_err(|e| format!("Error writing to stdout: {}",e))
    }
}

fn get_source_file(working_dir: &PathBuf, input_file: Option<&str>) -> Result<Option<PathBuf>,String> {
    Ok(if let Some(input_file) = input_file {
        Some(resolve_existing_file(&working_dir, input_file)?)
    } else {
        None
    })

}

pub fn validate_creature(input_file: Option<&str>, input_format: InputFormat, 
                         output_file: Option<&str>) -> Result<(),String> {
    let working_dir = get_default_working_dir()?;

    let target_file = if let Some(output_file) = output_file {
        Some(resolve_file(&working_dir, output_file))
    } else {
        None
    };

    let source = match input_format {
        InputFormat::Creature => {
            let source_file = get_source_file(&working_dir, input_file)?;
            // get the data from the file
            read_source(source_file.as_ref())?
        },
        InputFormat::Stored(creature_name) => {
            match creature_name.as_str() {
                "dragon" | "adult gold dragon" => monstorr_data::creatures::DRAGON_GOLD_ADULT,
                "goblin" => monstorr_data::creatures::GOBLIN,
                "bugbear" => monstorr_data::creatures::BUGBEAR,
                "efreeti" => monstorr_data::creatures::EFREETI,
                _ => Err("Couldn't find creature in list.".to_owned())?
            }.to_owned()
        },
        _ => Err("Input must be a creature file.".to_owned())?
    };

    // deserialize the commands
    let creator = CreatureCreator::load_from_str(&source).map_err(|e| format!("Error loading creature commands: {}",e))?;
    let output = creator.save_to_string().map_err(|e| format!("Error writing creature to string: {}",e))?;


    write_target(target_file, &output)

    
}

pub fn create_stat_block(input_file: Option<&str>, input_format: InputFormat, 
                         output_file: Option<&str>, output_format: OutputFormat) -> Result<(),String> {
    let working_dir = get_default_working_dir()?;

    let target_file = if let Some(output_file) = output_file {
        Some(resolve_file(&working_dir, output_file))
    } else {
        None
    };


    let stat_block = match input_format {
        InputFormat::Creature => {
            let source_file = get_source_file(&working_dir, input_file)?;
            // get the data from the file
            let source = read_source(source_file.as_ref())?;
            // the final working directory should be the directory in which the source file is located.
            let working_dir = get_working_dir_relative_to_source_or_default(&source_file, &working_dir);
            // deserialize the commands
            let creator = CreatureCreator::load_from_str(&source).map_err(|e| format!("Error loading creature commands: {}",e))?;
            let creature = creator.create_creature(&working_dir).map_err(|e| format!("{}",e))?;
            creature.try_into_stat_block().map_err(|e| format!("{}",e))?
        },
        InputFormat::Open5e => {
            let source_file = get_source_file(&working_dir, input_file)?;
            // get the data from the file
            let source = read_source(source_file.as_ref())?;
            // deserialize the stat block
            let creature = Open5eMonster::load_from_str(&source).map_err(|e| format!("Error loading open5e monster: {}",e))?;
            creature.try_into_stat_block().map_err(|e| format!("{}",e))?

        },
        InputFormat::Open5eList(creature_name) => {
            let source_file = get_source_file(&working_dir, input_file)?;
            // get the data from the file
            let source = read_source(source_file.as_ref())?;
            // deserialize the stat block
            let list = Open5eMonsterList::load_from_str(&source).map_err(|e| format!("Error loading open5e monster: {}",e))?;
            if let Some(creature) = list.results.into_iter().find(|creature| (creature.slug == creature_name) || (creature.name == creature_name)) {
                creature.try_into_stat_block().map_err(|e| format!("{}",e))?
            } else {
                Err("Couldn't find creature in list.".to_owned())?
            }
        },
        InputFormat::Stored(creature_name) => {
            let source = match creature_name.as_str() {
                "dragon" | "adult gold dragon" => monstorr_data::creatures::DRAGON_GOLD_ADULT,
                "goblin" => monstorr_data::creatures::GOBLIN,
                "bugbear" => monstorr_data::creatures::BUGBEAR,
                "efreeti" => monstorr_data::creatures::EFREETI,
                _ => Err("Couldn't find creature in list.".to_owned())?
            };
            // deserialize the commands
            let creator = CreatureCreator::load_from_str(source).map_err(|e| format!("Error loading creature commands: {}",e))?;
            let creature = creator.create_creature(&working_dir).map_err(|e| format!("{}",e))?;
            creature.try_into_stat_block().map_err(|e| format!("{}",e))?
        }
    };

    let output = match output_format {
        OutputFormat::JSON(ugly) => stat_block.write_to_string(ugly)?,
        OutputFormat::MiniJinjaTemplate(template,include_files) => {
            // use the default working dir instead of making it relative to the source.
            let template_file = resolve_existing_file(&working_dir, &template)?;
            let main_template = (template,read_source(Some(&template_file))?).into();
            
            let mut includes = Vec::new();
            let mut template_dir = template_file.clone();
            template_dir.pop();

            for include in include_files {
                let file = resolve_existing_file(&working_dir, &include)?;
                let name = if let Some(name) = path_relative_from(&file, &template_dir) {
                    if let Some(name) = name.to_str() {
                        name.to_owned()
                    } else {
                        include
                    }
                } else {
                    include
                };
                includes.push((name,read_source(Some(&file))?).into());
            }
            
            process_template(main_template,includes,&stat_block).map_err(|e| format!("Error processing template: {}",e))?
        },
        OutputFormat::HTML(two_column_height,fragment) => {
            let (main_template,includes) = if fragment {
                let main_template = monstorr_data::templates::STAT_BLOCK_HTML_TEMPLATE.into();
                let includes = monstorr_data::templates::stat_block_html_template_includes(two_column_height).iter().map(|a| a.into()).collect();
                (main_template,includes)

            } else {
                let main_template = monstorr_data::templates::FULL_HTML_TEMPLATE.into();
                let includes = monstorr_data::templates::full_html_template_includes(two_column_height).iter().map(|a| a.into()).collect();
                (main_template,includes)
    
            };
            process_template(main_template, includes, &stat_block).map_err(|e| format!("Error producing HTML: {}",e))?
        }
    };

    write_target(target_file, &output)

    
}

pub fn list_html_template_names() -> Vec<String> {
    let mut result = Vec::new();
    result.push(monstorr_data::templates::FULL_HTML_TEMPLATE.0.to_owned());
    for template in monstorr_data::templates::STANDARD_HTML_TEMPLATE_INCLUDES {
        result.push(template.0.to_owned())
    }
    for template in monstorr_data::templates::ADDITIONAL_FULL_HTML_TEMPLATE_INCLUDES {
        result.push(template.0.to_owned())
    }
    result

}

fn print_template_if_name_matches(name: &str,template: (&str,&str)) -> bool {
    if name == template.0 {
        println!("{}",template.1);
        true
    } else {
        false
    }

}

pub fn print_template(name: &str) -> Result<(),String> {
    if !print_template_if_name_matches(name, monstorr_data::templates::FULL_HTML_TEMPLATE) {
        for template in monstorr_data::templates::STANDARD_HTML_TEMPLATE_INCLUDES {
            if print_template_if_name_matches(name, template) {
                return Ok(());
            }
        }
        for template in monstorr_data::templates::ADDITIONAL_FULL_HTML_TEMPLATE_INCLUDES {
            if print_template_if_name_matches(name, template) {
                return Ok(());
            }
        }
    } else {
        return Ok(());
    }
    Err(format!("This program does not contain a template named '{}'",name))
    
}


pub struct CreatureSummary {
    pub name: String,
    pub slug: String,
    pub type_: String,
    pub subtype: Option<String>,
    pub size: String,
    pub alignment: String,
    pub challenge_rating: String
}

pub fn list_creatures(input_file: Option<&str>, input_format: ListInputFormat, type_: Option<String>, subtype: Option<String>, size: Option<String>,alignment: Option<String>,max_cr: Option<String>,min_cr: Option<String>) -> Result<Vec<CreatureSummary>,String> {

    let working_dir = get_default_working_dir()?;

    let source_file = if let Some(input_file) = input_file {
        Some(resolve_existing_file(&working_dir, input_file)?)
    } else {
        None
    };

    // get the data from the file
    let source = read_source(source_file.as_ref())?;

    let type_ = type_.map(|a| a.trim().to_lowercase());
    let size = size.map(|a| a.trim().to_lowercase());
    let alignment = alignment.map(|a| a.trim().to_lowercase());
    let max_cr = max_cr.map(|a| a.parse::<ChallengeRating>()).transpose().map_err(|e| format!("Could not parse max_cr: {}",e))?;
    let min_cr = min_cr.map(|a| a.parse::<ChallengeRating>()).transpose().map_err(|e| format!("Could not parse min_cr: {}",e))?;

    match input_format {
        ListInputFormat::Open5eList => {
            // deserialize the stat block
            let list = Open5eMonsterList::load_from_str(&source).map_err(|e| format!("Error loading open5e monster: {}",e))?;
            let mut result = Vec::new();
            for creature in list.results {
                if let Some(type_) = &type_ {
                    if &creature.type_.to_lowercase() != type_ {
                        continue;
                    }
                }
                if let Some(subtype) = &subtype {
                    if &creature.subtype.to_lowercase() != subtype {
                        continue;
                    }
                }
                if let Some(size) = &size {
                    if &creature.size.to_lowercase() != size {
                        continue;
                    }
                }
                if let Some(alignment) = &alignment {
                    if &creature.alignment.to_lowercase() != alignment {
                        continue;
                    }
                }
                
                if let Some(max_cr) = &max_cr {
                    if &creature.challenge_rating.parse::<ChallengeRating>().
                    map_err(|e| format!("Could not parse cr on creature '{}': {}",creature.name,e))? > 
                    max_cr {
                        continue;
                    }
                }
                if let Some(min_cr) = &min_cr {
                    if &creature.challenge_rating.parse::<ChallengeRating>().
                            map_err(|e| format!("Could not parse cr on creature '{}': {}",creature.name,e))? < 
                            min_cr {
                        continue;
                    }
                }

                let subtype = if creature.subtype == "" {
                    None
                } else {
                    Some(creature.subtype.clone())
                };

                result.push(CreatureSummary {
                    name: creature.name.clone(),
                    slug: creature.slug.clone(),
                    type_: creature.type_.clone(),
                    subtype,
                    size: creature.size.clone(),
                    alignment: creature.alignment.clone(),
                    challenge_rating: creature.challenge_rating.clone()
                });
            };
            Ok(result)
        }
    }
    
}

