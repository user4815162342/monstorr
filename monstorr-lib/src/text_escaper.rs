use std::collections::HashMap;
use crate::structured_text::TextBlock;
use crate::structured_text::TextSpan;
use crate::stat_block::StatBlockFeature;
use crate::stat_block::StatBlockLairActions;
use crate::stat_block::StatBlockLegendary;
use crate::stat_block::StatBlockRegionalEffects;
use crate::stat_block::CreatureStatBlock;

pub trait TextEscaper {

    fn escape_text(&self, source: &str) -> String;
}

impl TextEscaper for HashMap<char,String> {

    fn escape_text(&self, source: &str) -> String {
        // FUTURE: Is there a more efficient way to do this?
        let mut result = String::new();

        for ch in source.chars() {
            if let Some(replace) = self.get(&ch) {
                result.push_str(replace)
            } else {
                result.push(ch)
            }
        }

        result
    }
}

impl<Function: Fn(char) -> Option<&'static str>> TextEscaper for Function {

    fn escape_text(&self, source: &str) -> String {
        // FUTURE: Is there a more efficient way to do this?
        let mut result = String::new();

        for ch in source.chars() {
            if let Some(escaped) = self(ch) {
                result.push_str(escaped)
            } else {
                result.push(ch)
            }
        }

        result
        
    }

}

pub trait Escapable {

    fn escape<Escaper: TextEscaper>(&self, escaper: &Escaper) -> Self;

}

impl Escapable for String {

    fn escape<Escaper: TextEscaper>(&self, escaper: &Escaper) -> Self {
        escaper.escape_text(self)
    }

}

impl<Item: Escapable> Escapable for Vec<Item> {

    fn escape<Escaper: TextEscaper>(&self, escaper: &Escaper) -> Self {
        self.iter().map(|a| a.escape(escaper)).collect()
    }

}

impl<Item: Escapable> Escapable for Option<Item> {

    fn escape<Escaper: TextEscaper>(&self, escaper: &Escaper) -> Self {
        if let Some(me) = self {
            Some(me.escape(escaper))
        } else {
            None
        }
    }

}

impl Escapable for TextSpan {

    fn escape<Escaper: TextEscaper>(&self, escaper: &Escaper) -> Self {
        match self {
            TextSpan::Bold(text) => TextSpan::Bold(text.escape(escaper)),
            TextSpan::Normal(text) => TextSpan::Normal(text.escape(escaper)),
            TextSpan::Italic(text) => TextSpan::Italic(text.escape(escaper)),
            TextSpan::BoldItalic(text) => TextSpan::BoldItalic(text.escape(escaper))
        }
    }

}

impl Escapable for TextBlock {

    fn escape<Escaper: TextEscaper>(&self, escaper: &Escaper) -> Self {
        match self {
            TextBlock::Paragraph{ heading, body } => TextBlock::Paragraph{
                heading: heading.escape(escaper),
                body: body.escape(escaper)
            },
            TextBlock::SubParagraph{ heading, body } => TextBlock::SubParagraph{
                heading: heading.escape(escaper),
                body: body.escape(escaper)
            }
        }
    }

}

impl Escapable for StatBlockFeature {

    fn escape<Escaper: TextEscaper>(&self, escaper: &Escaper) -> Self {
        Self {
            text: self.text.escape(escaper)
        }
    }

}

impl Escapable for StatBlockLairActions {

    fn escape<Escaper: TextEscaper>(&self, escaper: &Escaper) -> Self {
        Self {
            actions: self.actions.escape(escaper),
            afterword: self.afterword.escape(escaper),
            foreword: self.foreword.escape(escaper)
        }
    }

}

impl Escapable for StatBlockLegendary {

    fn escape<Escaper: TextEscaper>(&self, escaper: &Escaper) -> Self {
        Self {
            actions: self.actions.escape(escaper),
            description: self.description.escape(escaper)
        }
    }
}

impl Escapable for StatBlockRegionalEffects {

    fn escape<Escaper: TextEscaper>(&self, escaper: &Escaper) -> Self {
        Self {
            afterword: self.afterword.escape(escaper),
            effects: self.effects.escape(escaper),
            foreword: self.foreword.escape(escaper)
        }
    }

}

impl Escapable for CreatureStatBlock {

    fn escape<Escaper: TextEscaper>(&self, escaper: &Escaper) -> Self {

        Self {
            name: self.name.escape(escaper),
            actions: self.actions.escape(escaper),
            alignment: self.alignment.escape(escaper),
            armor: self.armor.escape(escaper),
            challenge_rating: self.challenge_rating.escape(escaper),
            charisma: self.charisma.escape(escaper),
            condition_immunities: self.condition_immunities.escape(escaper),
            constitution: self.constitution.escape(escaper),
            damage_immunities: self.damage_immunities.escape(escaper),
            damage_resistances: self.damage_resistances.escape(escaper),
            damage_vulnerabilities: self.damage_vulnerabilities.escape(escaper),
            dexterity: self.dexterity.escape(escaper),
            group: self.group.escape(escaper),
            hit_points: self.hit_points.escape(escaper),
            intelligence: self.intelligence.escape(escaper),
            lair_actions: self.lair_actions.escape(escaper),
            languages: self.languages.escape(escaper),
            legendary_actions: self.legendary_actions.escape(escaper),
            reactions: self.reactions.escape(escaper),
            regional_effects: self.regional_effects.escape(escaper),
            saving_throws: self.saving_throws.escape(escaper),
            senses: self.senses.escape(escaper),
            size: self.size.escape(escaper),
            skills: self.skills.escape(escaper),
            source: self.source.escape(escaper),
            special_abilities: self.special_abilities.escape(escaper),
            speed: self.speed.escape(escaper),
            strength: self.strength.escape(escaper),
            subtype: self.subtype.escape(escaper),
            type_: self.type_.escape(escaper),
            wisdom: self.wisdom.escape(escaper),
        }
    }


}

pub fn escape_latex(ch: char) -> Option<&'static str> {
    match ch {
        '&' => Some("\\&"),
        '%' => Some("\\%"),
        '$' => Some("\\$"),
        '#' => Some("\\#"),
        '_' => Some("\\_"),
        '{' => Some("\\{"),
        '}' => Some("\\}"),
        '~' => Some("\textasciitilde"),
        '^' => Some("\textasciicircum"),
        '\\' => Some("\textbackslash"),
        _ => None    
    }

}
