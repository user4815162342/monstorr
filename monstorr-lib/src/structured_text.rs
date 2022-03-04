/*
 * Copyright © 2022 Neil M. Sheldon
 * 
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use serde::Deserialize;
use serde::Serialize;

#[derive(PartialEq,Debug)]
#[derive(Serialize,Deserialize)]
#[serde(rename_all = "lowercase")]
#[serde(tag="style",content="content")]
/**
The JSON form of TextSpan does not quite match the rust form. All text spans share a similar structure:

```text
TextSpan = {
    style: "normal" | "italic" | "bold" | "bold-italic",
    content: <string>
}
```
*/
pub enum TextSpan {
    Normal(String),
    Italic(String),
    Bold(String),
    BoldItalic(String)
}

#[derive(PartialEq,Debug)]
#[derive(Serialize,Deserialize)]
#[serde(rename_all = "lowercase")]
#[serde(tag="block")]
/**
The JSON form of TextBlock does not quite match the rust form. All text blocks share a similar structure:

```text
TextBlock = {
    block: "paragraph" | "sub-paragraph",
    heading?: list(<TextSpan>),
    body: list(<TextSpan>)
}
```
*/
pub enum TextBlock {
    Paragraph {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        heading: Option<Vec<TextSpan>>,
        body: Vec<TextSpan>
    },
    // In the SRD, these have non-italic bold headers and a hanging indent
    // NOTE: The SRD also has another paragraph type, which has a normal indent
    // and a header in non-bold italic. However, I can only find this on the Vampire's
    // "Vampire Weaknesses". Of course, The version here is found in metallic dragon's
    // double breath weapons, and in legendary actions. I suspect the vampire weaknesses
    // was a formatting mistake, even though I actually like that one better.
    SubParagraph {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        heading: Option<Vec<TextSpan>>,
        body: Vec<TextSpan>
    }
}