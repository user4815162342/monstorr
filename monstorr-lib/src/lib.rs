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
use monstorr_data::creatures::CreatureSummary;
use monstorr_data::templates::StoredTemplates;
use monstorr_data::templates::TemplateOptions;

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
use crate::utils::to_kebab_case;
use crate::template::TemplateSourceResolver;

pub use creature_commands::MONSTORR_VERSION;

pub enum InputFormat {
    Creature,
    Open5e,
    Open5eList(String),
    Stored(String)
}

pub enum ListInputFormat {
    Open5eList,
    Stored
}

impl Default for InputFormat {

    fn default() -> Self {
        Self::Creature
    }
}


pub enum OutputFormat {
    JSON(bool), // whether to print ugly
    MiniJinjaTemplate(String,Vec<String>), // path to template, paths to templates to be included
    HTML(Option<usize>,bool), // an optional usize indicating that they want a two-column stat-block instead of one-column, a bool indicating that only a fragment should be output
    LaTeX()
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

fn resolve_existing_dir(working_dir: &PathBuf, file: &str) -> Result<PathBuf,String> {
    let result = resolve_file(working_dir, file);
    if !result.is_dir() {
        Err(format!("Path {} does not exist or is not a directory",file))
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
            if let Some(entry) = monstorr_data::creatures::STORED_CREATURES.iter().find(|(creature,_)| (creature.slug == creature_name) || (creature.name == creature_name)) {
                entry.1.to_owned()
            } else {
                Err("Couldn't find creature in list.".to_owned())?
            }
        },
        _ => Err("Input must be a creature file.".to_owned())?
    };

    // deserialize the commands
    let creator = CreatureCreator::load_from_str(&source).map_err(|e| format!("Error loading creature commands: {}",e))?;
    let output = creator.save_to_string().map_err(|e| format!("Error writing creature to string: {}",e))?;


    write_target(target_file, &output)

    
}

impl TemplateSourceResolver for PathBuf {

    fn get_template(&self, name: &str) -> Result<Option<String>,String> {
        let mut result = self.clone();
        result.push(name);
        if result.is_file() {
            Ok(Some(read_source(Some(&result))?))
        } else {
            Ok(None)
        }        
    }

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
            let source = if let Some(entry) = monstorr_data::creatures::STORED_CREATURES.iter().find(|(creature,_)| (creature.slug == creature_name) || (creature.name == creature_name)) {
                entry.1
            } else {
                Err("Couldn't find creature in list.".to_owned())?
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
            let template_name = if let Some(file_name) = template_file.file_name() {
                if let Some(file_name) = file_name.to_str() {
                    file_name.to_owned()
                } else {
                    template
                }
            } else {
                template
            };
            
            let mut includes = Vec::new();
            let mut template_dir = template_file.clone();
            template_dir.pop();

            // resolve any include files as relative to main directory
            // (command line processing might have already resolved them to longer paths)
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
                includes.push(name);
            }
            
            process_template(&template_dir,&template_name,&includes,&stat_block).map_err(|e| format!("Error processing template: {}",e))?
        },
        OutputFormat::HTML(two_column_height,fragment) => {
            let main_template = if fragment {
                monstorr_data::templates::STAT_BLOCK_HTML_TEMPLATE
            } else {
                monstorr_data::templates::FULL_HTML_TEMPLATE
            };
            process_template(&StoredTemplates::instance(TemplateOptions::html(two_column_height)), main_template, &Vec::new(), &stat_block).map_err(|e| format!("Error producing HTML: {}",e))?
        },
        OutputFormat::LaTeX() => {
            let main_template = monstorr_data::templates::LATEX_TEMPLATE;
            process_template(&StoredTemplates::instance(TemplateOptions::latex()), main_template, &Vec::new(), &stat_block).map_err(|e| format!("Error producing LaTeX: {}",e))?
        }
    };

    write_target(target_file, &output)

    
}

pub fn list_template_names(class: Option<&str>) -> Vec<String> {
    StoredTemplates::instance(None).list(class)
}

pub fn print_template(name: &str) -> Result<(),String> {
    let resolver = StoredTemplates::instance(None);
    if let Some(template) = resolver.get_template(name)? {
        println!("{}",template);
        Ok(())
    } else {
        Err(format!("This program does not contain a template named '{}'",name))
    }
}


pub fn list_creatures(input_file: Option<&str>, input_format: ListInputFormat, type_: Option<String>, subtype: Option<String>, size: Option<String>,alignment: Option<String>,max_cr: Option<String>,min_cr: Option<String>) -> Result<Vec<CreatureSummary<String>>,String> {


    let type_ = type_.map(|a| a.trim().to_lowercase());
    let size = size.map(|a| a.trim().to_lowercase());
    let alignment = alignment.map(|a| a.trim().to_lowercase());
    let max_cr = max_cr.map(|a| a.parse::<ChallengeRating>()).transpose().map_err(|e| format!("Could not parse max_cr: {}",e))?;
    let min_cr = min_cr.map(|a| a.parse::<ChallengeRating>()).transpose().map_err(|e| format!("Could not parse min_cr: {}",e))?;

    macro_rules! match_filter {
        ($creature: ident) => {{
            if let Some(type_) = &type_ {
                if &$creature.type_.to_lowercase() != type_ {
                    continue;
                }
            }
        
            if let Some(subtype) = &subtype {
                if let Some(creature_subtype) = &$creature.subtype {
                    if &creature_subtype.to_lowercase() != subtype {
                        continue;
                    }
                } else {
                    continue;
                }
                
            }
            if let Some(size) = &size {
                if &$creature.size.to_lowercase() != size {
                    continue;
                }
            }
            if let Some(alignment) = &alignment {
                if &$creature.alignment.to_lowercase() != alignment {
                    continue;
                }
            }
            
            if let Some(max_cr) = &max_cr {
                if &$creature.challenge_rating.parse::<ChallengeRating>().
                map_err(|e| format!("Could not parse cr on creature '{}': {}",$creature.name,e))? > 
                max_cr {
                    continue;
                }
            }
            if let Some(min_cr) = &min_cr {
                if &$creature.challenge_rating.parse::<ChallengeRating>().
                        map_err(|e| format!("Could not parse cr on creature '{}': {}",$creature.name,e))? < 
                        min_cr {
                    continue;
                }
            }
        }};
    }

    match input_format {
        ListInputFormat::Open5eList => {
            let working_dir = get_default_working_dir()?;

            let source_file = if let Some(input_file) = input_file {
                Some(resolve_existing_file(&working_dir, input_file)?)
            } else {
                None
            };
        
            // get the data from the file
            let source = read_source(source_file.as_ref())?;

            // deserialize the stat block
            let list = Open5eMonsterList::load_from_str(&source).map_err(|e| format!("Error loading open5e monster: {}",e))?;
            let mut result = Vec::new();
            for creature in list.results {

                match_filter!(creature);

                result.push(CreatureSummary {
                    name: creature.name.clone(),
                    slug: creature.slug.clone(),
                    type_: creature.type_.clone(),
                    subtype: creature.subtype.clone(),
                    size: creature.size.clone(),
                    alignment: creature.alignment.clone(),
                    challenge_rating: creature.challenge_rating.clone()
                });
            };
            Ok(result)
        },
        ListInputFormat::Stored => {
            // we have summary data already.
            let mut result = Vec::new();
            for (creature,_) in monstorr_data::creatures::STORED_CREATURES {
                
                match_filter!(creature);

                result.push(CreatureSummary {
                    name: creature.name.to_owned(),
                    slug: creature.slug.to_owned(),
                    type_: creature.type_.to_owned(),
                    subtype: creature.subtype.map(|a| a.to_owned()),
                    size: creature.size.to_owned(),
                    alignment: creature.alignment.to_owned(),
                    challenge_rating: creature.challenge_rating.to_owned()
                });
            };
            Ok(result)
        }        
    }
    
}


pub fn generate_creatures_as_rust_array(search_directory: &str) -> Result<(),String> {
    let working_dir = get_default_working_dir()?;

    let search_directory = resolve_existing_dir(&working_dir, search_directory)?;

    let mut target_file = search_directory.clone();
    target_file.push("creature_database.rs.inc");
    let target_file = target_file;

    let mut files = Vec::new();

    // I'm looping twice because I want to collect the files *now* before someone adds a new one or something...
    for file in fs::read_dir(&search_directory).map_err(|e| format!("{}",e))? {
        let file = file.map_err(|e| format!("{}",e))?;
        let path: PathBuf = file.path();
        if let Some(extension) = path.extension() {
            if let Some("creature") = extension.to_str() {
                files.push(path);
            }
        }
    }

    let mut output = String::new();

    output.push_str(&format!("// This file was automatically generated, do not edit. Use `build-data.sh` to regenerate\n"));

    // wrap in a struct so the compiler can warn me if I'm missing something I'll need in list_creatures
    output.push_str(&format!("pub const STORED_CREATURES: [(CreatureSummary<&'static str>, &'static str); {}] = [",files.len()));
/*
const TEST: CreatureSummary<&str> = CreatureSummary {
    name: "test",
    slug: "test",
    type_: "type",
    subtype: Some("test"),
    size: "test",
    alignment: "test",
    challenge_rating: "test"
};
*/

    for file in files {
        let include_filename = if let Some(name) = path_relative_from(&file, &search_directory) {
            if let Some(name) = name.to_str() {
                name.to_owned()
            } else {
                file.display().to_string()
            }
        } else {
            file.display().to_string()
        };
        println!("Processing creature {}",include_filename);
        // get the data from the file
        let source = read_source(Some(&file))?;
        // the final working directory should be the directory in which the source file is located.
        let working_dir = get_working_dir_relative_to_source_or_default(&Some(file), &working_dir);
        // deserialize the commands
        let creator = CreatureCreator::load_from_str(&source).map_err(|e| format!("Error loading creature commands: {}",e))?;
        let creature = creator.create_creature(&working_dir).map_err(|e| format!("{}",e))?;
        let slug = to_kebab_case(&creature.name);
        // wrap in a struct so the compiler can warn me if I'm missing something I'll need in list_creatures
        let summary = CreatureSummary {
            name: creature.name,
            slug,
            type_: creature.type_.to_string(),
            subtype: creature.subtype,
            size: creature.size.to_string(),
            alignment: creature.alignment.to_string(),
            challenge_rating: creature.challenge_rating.to_string()
        };

        output.push_str("\n        (");
        output.push_str("\n            CreatureSummary {");
        output.push_str(&format!("\n                name: {:?},",summary.name));
        output.push_str(&format!("\n                slug: {:?},",summary.slug));
        output.push_str(&format!("\n                type_: {:?},",summary.type_));
        output.push_str(&format!("\n                subtype: {:?},",summary.subtype));
        output.push_str(&format!("\n                size: {:?},",summary.size));
        output.push_str(&format!("\n                alignment: {:?},",summary.alignment));
        output.push_str(&format!("\n                challenge_rating: {:?},",summary.challenge_rating));
        output.push_str("\n             },");
        output.push_str(&format!("\n             include_str!({:?})",include_filename));
        output.push_str("\n        ),");
    }

    output.push_str("\n    ];");


    write_target(Some(target_file), &output)

}