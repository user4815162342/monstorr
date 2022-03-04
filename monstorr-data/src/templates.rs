/*
 * Copyright © 2022 Neil M. Sheldon
 * 
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

 macro_rules! template {
    ($template: literal) => {
        ($template,include_str!(concat!("../data/",$template)))   
    };
}

pub const FULL_HTML_TEMPLATE: (&'static str, &'static str) = template!("html-full-page-template.html");
pub const STAT_BLOCK_HTML_TEMPLATE: (&'static str, &'static str) = template!("html-stat-block-template.html");

// list is incomplete, another template is generated automatically in html_template_includes
pub const FULL_HTML_TEMPLATE_INCLUDES: [(&'static str, &'static str);6] = 
    [template!("blocks-template.html"),
     template!("spans-template.html"),
     template!("feature-template.html"),
     template!("tapered-rule.html"),
     template!("html-styles-fragment.html"),
     template!("html-stat-block-template.html")];

// list is incomplete, another template is generated automatically in html_template_includes
pub const STAT_BLOCK_HTML_TEMPLATE_INCLUDES: [(&'static str, &'static str);4] = 
     [template!("blocks-template.html"),
      template!("spans-template.html"),
      template!("feature-template.html"),
      template!("tapered-rule.html")];
 
fn html_template_includes(two_column: Option<usize>,includes: &[(&'static str, &'static str)]) -> Vec<(String,String)> {
    let mut result: Vec<_> = includes.iter().map(|a| (a.0.to_owned(),a.1.to_owned())).collect();
    result.push(if let Some(stat_block_height) = two_column {
        ("two-column".to_owned(),format!("data-two-column=\"\" style=\"--data-content-height: {}px;\"",stat_block_height))
    } else {
        ("two-column".to_owned(),String::new())
    });
    result

}

pub fn full_html_template_includes(two_column: Option<usize>) -> Vec<(String,String)> {
    html_template_includes(two_column, &FULL_HTML_TEMPLATE_INCLUDES)
}

pub fn stat_block_html_template_includes(two_column: Option<usize>) -> Vec<(String,String)> {
    html_template_includes(two_column, &STAT_BLOCK_HTML_TEMPLATE_INCLUDES)

}