/*
 * Copyright © 2022 Neil M. Sheldon
 * 
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

 macro_rules! template {
    ($class: literal, $template: literal) => {
        ($template,include_str!(concat!("../data/templates/",$class,"/",$template)))   
    };
}

pub const FULL_HTML_TEMPLATE: (&'static str, &'static str) = template!("html","html-full-page-template.html");
pub const STAT_BLOCK_HTML_TEMPLATE: (&'static str, &'static str) = template!("html","html-stat-block-template.html");

// list is incomplete, another template is generated automatically in html_template_includes
pub const ADDITIONAL_FULL_HTML_TEMPLATE_INCLUDES: [(&'static str, &'static str);2] = 
    [template!("html","html-styles-fragment.html"),
     template!("html","html-stat-block-template.html")];

// list is incomplete, another template is generated automatically in html_template_includes
pub const STANDARD_HTML_TEMPLATE_INCLUDES: [(&'static str, &'static str);4] = 
     [template!("html","blocks-template.html"),
      template!("html","spans-template.html"),
      template!("html","feature-template.html"),
      template!("html","tapered-rule.html")];
 
fn html_template_includes(two_column: Option<usize>,addl_includes: &[(&'static str, &'static str)]) -> Vec<(String,String)> {
    let mut result: Vec<_> = STANDARD_HTML_TEMPLATE_INCLUDES.iter().chain(addl_includes).map(|a| (a.0.to_owned(),a.1.to_owned())).collect();
    result.push(if let Some(stat_block_height) = two_column {
        ("two-column".to_owned(),format!("data-two-column=\"\" style=\"--data-content-height: {}px;\"",stat_block_height))
    } else {
        ("two-column".to_owned(),String::new())
    });
    result

}

pub fn full_html_template_includes(two_column: Option<usize>) -> Vec<(String,String)> {
    html_template_includes(two_column, &ADDITIONAL_FULL_HTML_TEMPLATE_INCLUDES)
}

pub fn stat_block_html_template_includes(two_column: Option<usize>) -> Vec<(String,String)> {
    html_template_includes(two_column, &[])

}