/*
 * Copyright © 2022 Neil M. Sheldon
 * 
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
#![allow(rustdoc::private_intra_doc_links)]
/*!
Monstorr is a simple tool for generating and validating D&D Fifth Edition monster stats, for use in homebrew and published game content. 

This is the main run-time crate for Monstorr. For more information about the tool, see its usage, which is also generated from doc comments inside this module. Starting at [`Arguments`].

For more detailed documentation, including information on how to build a creature file, see [`monstorr_lib`].

*/

use std::process;

use clap::Parser;
use clap::ArgEnum;
use clap::Subcommand;
use clap::Args;

use monstorr_lib::create_stat_block;
use monstorr_lib::list_html_template_names;
use monstorr_lib::print_template;
use monstorr_lib::list_creatures;
use monstorr_lib::validate_creature;
use monstorr_lib::MONSTORR_VERSION;
use monstorr_lib::InputFormat as MonstorrInputFormat;
use monstorr_lib::OutputFormat as MonstorrOutputFormat;
use monstorr_lib::ListInputFormat as MonstorrListInputFormat;


// this is different from the one in monstorr-lib because its redefined to work as an ArgEnum
// which can't support data inside the variants and can't be derived from another crate
#[derive(ArgEnum,Clone)]
/// Represents in input format argument for commands that require input files
enum InputFormat {
    /// A file containing creature commands for building a creature stat-block with automatic calculations
    Creature,
    /// A single-creature JSON file in the format used by Open5e
    Open5e,
    /// A JSON list in the format used by Open5e
    Open5eList,
    /// One of a few creatures stored in this program
    Stored
}

#[derive(ArgEnum,Clone)]
/// Represents a list input format argument for commands that require only list input files
enum ListInputFormat {
    /// A JSON list in the format used by Open5e
    Open5eList
}

// this is different from the one in monstorr-lib because its redefined to work as an ArgEnum
// which can't support data inside the variants and can't be derived from another crate
#[derive(ArgEnum,Clone)]
/// Represents a file output format argument for commands which produce statblocks
enum OutputFormat {
    JSON,
    /// Run the output through a minijinja template (specified with the template option)
    MiniJinja,
    /// Run the output through a default HTML templates
    HTML,
}

#[derive(Args)]
/// A central structure for input data, since the same by so many commands. Note that output format is not specified, as that's part of the command.
struct InputOutputData {
    #[clap(short,long,arg_enum,default_value_t=InputFormat::Creature)]
    /// format of the input file (see main help)
    format: InputFormat,
    #[clap(short,long,value_name="STRING")]
    // FUTURE: Is there some way I can make this required only when InputFormat is a list thingie?
    /// if the input file is a list, such as open5e-list, this specifies what creature to pick
    creature: Option<String>,
    #[clap(value_name="FILENAME")]
    /// input file, if not specified will read from stdin
    input: Option<String>,
    #[clap(value_name="FILENAME")]
    /// output file, if not specified will write to stdout
    output: Option<String>,

}

impl InputOutputData {

    /// Retrieves a [`monstorr_lib::InputFormat`] from the I/O arguments.
    fn get_monstorr_input(&self) -> Result<MonstorrInputFormat,String> {
        Ok(match self.format {
            InputFormat::Creature => MonstorrInputFormat::Creature,
            InputFormat::Open5e => MonstorrInputFormat::Open5e,
            InputFormat::Open5eList => if let Some(creature) = &self.creature {
                MonstorrInputFormat::Open5eList(creature.clone())
            } else {
                Err("Please specify a creature name to process.")?
            },
            InputFormat::Stored => if let Some(creature) = &self.creature {
                MonstorrInputFormat::Stored(creature.clone())
            } else {
                Err("Please specify a creature name to process.")?
            }

        })

    }
}

#[derive(ArgEnum,Clone)]
/// Represents an argument requiring a class of templates for the ListTemplates command.
enum TemplateClass {
    /// Templates (in MiniJinja syntax) used in producing HTML
    HTML
}

#[derive(Subcommand)]
/// Represents a command to be called by the main function
enum Command {

    #[clap(author="N. M. Sheldon", version, about, long_about = None)]
    /** 
    Generate a stat block in JSON format.

    The structure for the stat-block JSON is documented in this tool's code documentation. I hope to have a better link to this later.
    */
    JSON {
    
        #[clap(long)]
        /// turns off pretty-printing of JSON text.
        ugly: bool,
    
        #[clap(flatten)]
        input_output: InputOutputData
    },
    
    #[clap(author="N. M. Sheldon", version, about, long_about = None)]
    /**
    Run a stat block through a mini-jinja template to create almost any format you want.

    Mini-Jinja is a template format related to another template format called Jinja. For more information on the syntax, see [MiniJinja](https://docs.rs/minijinja/0.13.0/minijinja/syntax/index.html).

    Currently, the code used to render these templates does not support auto-discovery of included template files. If your template needs these, you will need to include these using the 'include' option, below. Every attempt has been made to make sure that you can reference these files using relative paths in the include statement and still get your include to work correctly.
    */
    MiniJinja {
    
        #[clap(long,value_name="FILENAME")]
        /// a template file
        template: String,
    
        #[clap(short,long,value_name="FILENAME")]
        /// additional template file required by the main template (may be specified multiple times)
        include: Vec<String>,
    
        #[clap(flatten)]
        input_output: InputOutputData

    },
    
    #[clap(author="N. M. Sheldon", version, about, long_about = None)]
    /**
    Generate a stat block in html using some nice styles that resembles the official books.

    This command utilizes built-in MiniJinja templates (see the `mini-jinja` command) to generate the HTML. It supports one or two-column formats, although the two-column format requires a height for the output box in `px` units. It can also generate just the `div` tag and its contents instead of the full HTML document.

    If you wish to modify the output, retrieve the styles for embedding multiple `div` fragments in a page, or just reference them for how to write a template, use the `list-templates html` command to retrieve them. One simple template, for specifying the two-column mode, is generated at run-time, but a comment in the template explains how to add this yourself.

    The HTML templates were based on styles used in [statblock5e](https://valloric.github.io/statblock5e/). That code was converted from "web components" into plain HTML, so it can support older browsers and not require JavaScript.
    */
    HTML {
 
        #[clap(flatten)]
        input_output: InputOutputData,
 
        #[clap(long,value_name="INTEGER")]
        /// a two-column stat-block is produced, with the specified height in pixels. The stat-block is one column if not specified.
        two_column: Option<usize>,
 
        #[clap(long)]
        /// only the stat-block div is produced, you would then import it into your own HTML page (see list-templates command to get the default styles)
        fragment: bool
    },

    /**
    Produce creature files unprocessed.

    This can be used for two purposes: to retrieve the text of creatures from the 'stored' input, and to validate creature files without building. The latter has few practical uses, but the former is useful if you wish to see examples of real creature documents.

    Attempting to use this on non-creature files will cause an error.
    */
    Validate {
        #[clap(flatten)]
        input_output: InputOutputData,
    },

    #[clap(author="N. M. Sheldon", version, about, long_about = None)]
    /// List built-in template files by template class, so you can modify or reference them.
    ListTemplates {
    
        #[clap(arg_enum)]
        /// The class of templates you want to list
        template_class: TemplateClass
    },

    #[clap(author="N. M. Sheldon", version, about, long_about = None)]
    /// Retrieve the source for a template so you can save and customize it
    GetTemplate {
        /// The name of the template you want to extract
        name: String
    },

    #[clap(author="N. M. Sheldon", version, about, long_about = None)]
    /**
    List monsters in a file, filtering for specific data.

    This is a simple filtering tool to help you find creatures in a list. It currently only supports `open5e-list` files. See the information on input files in the main help for more information on this format.
    */
    ListCreatures {

        #[clap(short,long,arg_enum)]
        /// format of the input file (stored creatures can not be listed yet)
        format: ListInputFormat,

        #[clap(value_name="FILENAME")]
        /// input file, if not specified will read from stdin
        input: Option<String>,

        #[clap(long)]
        /// type (humanoid, undead, monstrosity, etc.) of creatures to show 
        type_: Option<String>,

        #[clap(long)]
        /// subtype (goblinoid, shapechanger, etc.) of creatures to show 
        subtype: Option<String>,

        #[clap(long)]
        /// size of creatures to show
        size: Option<String>,

        #[clap(long)]
        /// alignment of creatures to show
        alignment: Option<String>,

        #[clap(long)]
        /// max challenge rating to show
        max_cr: Option<String>,

        #[clap(long)]
        /// minimum challenge rating to show
        min_cr: Option<String>
    },

    #[clap(author="N. M. Sheldon", version, about, long_about = None)]
    /** 
    Display the creature format version used by this tool.

    This version is used in the `Monstorr` command in creature files to make sure you're processing your creature in the right version of Monstorr.
    */
    MonstorrVersion
}

#[derive(Parser)]
#[clap(author="N. M. Sheldon", version)]
/**
A tool for building D&D 5e creature stat blocks and other neat tricks
 
Monstorr's primary function is to build creature stat blocks while automatically calculating anything that can be calculated. For example, specify that the creature has a Longbow, and monstorr will calculate the appropriate attack and hit rolls based on the creatures dexterity and proficiency, and produce an appropriate attack action. There are a number of things it can calculate.

Monstorr can produce plain stat-block data in JSON format, using the `json` command, or it can automatically template that data into another text format using the `mini-jinja` command. It also has some built-in templates it uses to produce html text with the `html` command. The built-in templates can be reviewed with the `list-templates` and `get-templates` commands.

This tool was originally thought of as a creature database tool, and it still retains one command from that idea. The `list-creatures` command can be used to query a list file.

# Input Formats

Monstorr recognizes a few formats for the input data for generating stat blocks. These are used by all stat-block creation and templating commands.

* `creature`: This is essentially a list of commands for designing the creature, assuming defaults for everything not added. The syntax for this file format is documented in this tool's code documentation. I hope to have a better link to this later.

* `open5e-list`: This is the closest thing I could find to a standard format. This is a JSON format returned by queries to the monster database at [Open5e.com](https://open5e.com/monsters/monster-list). When generating stat-blocks from this format, a creature name is required. This format can also be queried using `list-creatures`. The stat-blocks generated from this list will not be formatted as nicely as with the `creature` format. Monstorr currently does not parse the Markdown text used in feature descriptions, calculations are not validated, and there are typos and errors in some of the creatures from that database.

* `open5e`: This is simply a single creature block extracted from an `open5e-list` file, as a stand-alone JSON file.

* `stored`: Monstorr contains a few pre-built creatures converted from D&D Fifth Edition System Reference Document* content. Currently, only four creatures are contained in the executable, as conversion of the creatures is a busy work task that would take some time. It is a dream that the entire SRD list of monsters be available in Monstorr, but it is likely that this will not happen before a hypothetical sixth edition happens.

(I apologize for the unusual formatting of this text, the tools used to generate it do not yet support structured and formatted text in this part, so it is written in raw Markdown format.)

*/
struct Arguments {
    #[clap(subcommand)]
    command: Command
}


/**
Runs the tool with the given arguments.
*/
fn run(args: Arguments) -> Result<(),String> {

    match args.command {
        Command::MonstorrVersion => {
            println!("monstorr creature commands version: {}",MONSTORR_VERSION);
            process::exit(0);
        },
        Command::JSON{ugly, input_output} => {
            let output_format = MonstorrOutputFormat::JSON(ugly);
            let input_format = input_output.get_monstorr_input()?;
            create_stat_block(input_output.input.as_deref(), input_format, input_output.output.as_deref(), output_format)
        },
        Command::HTML{input_output, two_column, fragment} => {
            let output_format = MonstorrOutputFormat::HTML(two_column,fragment);
            let input_format = input_output.get_monstorr_input()?;
            create_stat_block(input_output.input.as_deref(), input_format, input_output.output.as_deref(), output_format)
        },
        Command::MiniJinja{template,include,input_output} => {
            let output_format = MonstorrOutputFormat::MiniJinjaTemplate(template,include);
            let input_format = input_output.get_monstorr_input()?;
            create_stat_block(input_output.input.as_deref(), input_format, input_output.output.as_deref(), output_format)
        },
        Command::Validate{input_output} => {
            let input_format = input_output.get_monstorr_input()?;
            validate_creature(input_output.input.as_deref(), input_format, input_output.output.as_deref())
        },
        Command::ListTemplates{template_class} => {
            match template_class {
                TemplateClass::HTML => {
                    println!("{}",list_html_template_names().join("\n"));
                    Ok(())
                }
            }
        },
        Command::ListCreatures{format,input,type_,subtype,size,alignment,max_cr,min_cr} => {
            let format = match format {
                ListInputFormat::Open5eList => MonstorrListInputFormat::Open5eList
            };
            for creature in list_creatures(input.as_deref(),format,type_,subtype,size,alignment,max_cr,min_cr)? {
                println!("{} [{}] Challenge {}: {} {}{}, {} ",
                          creature.name,
                          creature.slug,
                          creature.alignment,
                          creature.size,
                          creature.type_,
                          if let Some(subtype) = creature.subtype {
                              format!(" ({})",subtype)
                          } else {
                              String::new()
                          },
                          creature.challenge_rating);
            }
            Ok(())
        }

        Command::GetTemplate{name} => {
            print_template(&name)
        }
    }


}

/**
Parses the command line arguments, runs the tool, and prints out any error messages.
*/
fn main() {

    if let Err(e) = run(Arguments::parse()) {
        eprintln!("{}",e);
        process::exit(1);
    }


}
