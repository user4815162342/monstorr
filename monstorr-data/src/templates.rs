use std::collections::HashMap;
/*
 * Copyright © 2022 Neil M. Sheldon
 * 
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

pub trait TemplateSourceResolver {

    fn get_template(&self, name: &str) -> Option<String>;
    
}

impl TemplateSourceResolver for () {

    fn get_template(&self, _name: &str) -> Option<String> {
        None
    }
}

macro_rules! template {
    ($class: literal, $template: literal) => {
        ($template,include_str!(concat!("../data/templates/",$class,"/",$template)))   
    };
}

pub const STORED_TEMPLATES: [(&'static str, &'static str); 8] = [
    template!("html","html-full-page-template.html"),
    template!("html","html-stat-block-template.html"),
    template!("html","html-styles-fragment.html"),
    template!("html","html-stat-block-template.html"),
    template!("html","blocks-template.html"),
    template!("html","spans-template.html"),
    template!("html","feature-template.html"),
    template!("html","tapered-rule.html")
];

pub struct TemplateOptions {
    html: Option<usize> // if set, the html template is supposed to be two-columns, and the value is the height of the div in pixels
}

impl TemplateOptions {

    pub fn html(html: Option<usize>) -> Option<Self> {
        Some(Self {
            html
        })
    }
}

pub struct StoredTemplates {
    options: Option<TemplateOptions>,
    map: HashMap<&'static str, &'static str>
}

impl StoredTemplates {

    pub fn instance(options: Option<TemplateOptions>) -> Self {
        Self {
            options,
            map: STORED_TEMPLATES.into_iter().collect()
        }
    }

}

impl TemplateSourceResolver for StoredTemplates {

    fn get_template(&self, name: &str) -> Option<String> {
        if let Some(template) = self.map.get(name) {
            Some((*template).to_owned())
        } else {
            match name {
                "html-two-column" => if let Some(TemplateOptions{ html: Some(stat_block_height), ..}) = self.options {
                    Some(format!("data-two-column=\"\" style=\"--data-content-height: {}px;\"",stat_block_height))
                } else {
                    Some(String::new())
                },
                _ => None
            }
        }
    }


}


pub const FULL_HTML_TEMPLATE: (&'static str,&'static str) = template!("html","html-full-page-template.html");
pub const STAT_BLOCK_HTML_TEMPLATE: (&'static str,&'static str) = template!("html","html-stat-block-template.html");

// list is incomplete, another template is generated automatically in html_template_includes
pub const ADDITIONAL_FULL_HTML_TEMPLATE_INCLUDES: [&'static str;2] = 
    ["html-styles-fragment.html",
     "html-stat-block-template.html"];

// list is incomplete, another template is generated automatically in html_template_includes
pub const STANDARD_HTML_TEMPLATE_INCLUDES: [&'static str;5] = 
     ["blocks-template.html",
      "spans-template.html",
      "feature-template.html",
      "tapered-rule.html",
      "html-two-column"];
 
fn html_template_includes(two_column: Option<usize>,addl_includes: &[&'static str]) -> Vec<(String,String)> {
    let resolver = StoredTemplates::instance(TemplateOptions::html(two_column));
    let mut result = Vec::new();
    for name in STANDARD_HTML_TEMPLATE_INCLUDES.iter().chain(addl_includes) {
        if let Some(template) = resolver.get_template(name) {
            result.push(((*name).to_owned(),template.to_owned()))
        }
    }
    result

}

pub fn full_html_template_includes(two_column: Option<usize>) -> Vec<(String,String)> {
    html_template_includes(two_column, &ADDITIONAL_FULL_HTML_TEMPLATE_INCLUDES)
}

pub fn stat_block_html_template_includes(two_column: Option<usize>) -> Vec<(String,String)> {
    html_template_includes(two_column, &[])

}