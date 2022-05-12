/*
 * Copyright © 2022 Neil M. Sheldon
 * 
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::collections::HashMap;

macro_rules! template {
    ($class: literal, $template: expr) => {
        ($template,($class,include_str!(concat!("../data/templates/",$class,"/",$template))))   
    };
}

macro_rules! html_full_page_template {
    () => {
        "html-full-page-template.html"
    };
}

macro_rules! html_stat_block_template {
    () => {
        "html-stat-block-template.html"
    };
}

macro_rules! latex_main_template {
    () => {
        "latex-stat-block-template.tex"
    };
}

pub const FULL_HTML_TEMPLATE: &'static str = html_full_page_template!();
pub const STAT_BLOCK_HTML_TEMPLATE: &'static str = html_stat_block_template!();
pub const HTML_TWO_COLUMN_TEMPLATE: &'static str = "html-two-column";
pub const LATEX_TEMPLATE: &'static str = latex_main_template!();

pub const STORED_TEMPLATES: [(&'static str, (&'static str, &'static str)); 12] = [
    template!("html",html_full_page_template!()),
    template!("html",html_stat_block_template!()),
    template!("html","html-styles-fragment.html"),
    template!("html","html-stat-block-template.html"),
    template!("html","blocks-template.html"),
    template!("html","spans-template.html"),
    template!("html","feature-template.html"),
    template!("html","tapered-rule.html"),
    template!("latex",latex_main_template!()),
    template!("latex","feature-template.tex"),
    template!("latex","blocks-template.tex"),
    template!("latex","spans-template.tex")
];

// TODO: I'm repeating these next to string constants


#[derive(Default)]
pub struct TemplateOptions {
    html: Option<usize>, // if set, the html template is supposed to be two-columns, and the value is the height of the div in pixels
}

impl TemplateOptions {

    pub fn html(html: Option<usize>) -> Option<Self> {
        Some(Self {
            html
        })
    }

    pub fn latex() -> Option<Self> {
        Some(Self {
            html: None
        })
    }

}

pub struct StoredTemplates {
    options: TemplateOptions,
    map: HashMap<&'static str, (&'static str, &'static str)>
}

impl StoredTemplates {

    pub fn instance(options: Option<TemplateOptions>) -> Self {
        let options = options.unwrap_or_default();
        Self {
            options,
            map: STORED_TEMPLATES.into_iter().collect()
        }
    }

    pub fn get(&self, name: &str) -> Option<String> {
        if let Some((_,template)) = self.map.get(name) {
            Some((*template).to_owned())
        } else {
            match name {
                HTML_TWO_COLUMN_TEMPLATE => Some(if let Some(stat_block_height) = self.options.html {
                    format!("data-two-column=\"\" style=\"--data-content-height: {}px;\"",stat_block_height)
                } else {
                    String::new()
                }),
                _ => None
            }
        }

    }

    pub fn list(&self, class: Option<&str>) -> Vec<String> {
        // TODO: Need to filter by class if specified
        if let Some(class) = &class {
            self.map.iter().filter_map(|(i,(c,_))| if c == class {
                Some((*i).to_owned())
            } else {
                None
            }).collect()
        } else {
            self.map.keys().map(|i| (*i).to_owned()).collect()
        }
    }

}

