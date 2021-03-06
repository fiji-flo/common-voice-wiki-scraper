use regex::Regex;

use crate::character_map::CHARACTER_MAP;

static PUNCTUATIONS: [char; 3] = ['。', '？', '！'];

pub struct SentenceExtractor {
    text: String,
}

impl SentenceExtractor {
    pub fn new(text: &str) -> SentenceExtractor {
        let lines: Vec<&str> = text.lines().collect();
        SentenceExtractor {
            text: if lines.len() > 1 {
                // skip disambiguation pages
                if lines.first().unwrap().contains("消歧義") {
                    String::default()
                } else {
                    // skip title
                    lines[1..].join("")
                }
            } else {
                text.to_string()
            },
        }
    }
}

fn is_invalid(s: &str) -> bool {
    !s.chars().next().unwrap_or_default().is_alphabetic()
        || s.chars().any(|c| c.is_ascii())
        || s.chars().all(|c| !c.is_alphabetic())
}

lazy_static! {
    static ref PARANS: Regex = Regex::new("（.*）").unwrap();
}

impl Iterator for SentenceExtractor {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        loop {
            if self.text.len() == 0 {
                return None;
            }

            let chars = self.text.chars().collect::<Vec<_>>();
            let end_index = chars
                .iter()
                .position(|&c| PUNCTUATIONS.contains(&c) || c == '\n');
            let index = end_index.unwrap_or(chars.len());
            let mut next_item = chars
                .iter()
                .take(index)
                .collect::<String>()
                .trim()
                .to_string();
            self.text = chars
                .iter()
                .skip(index + (if end_index.is_some() { 1 } else { 0 }))
                .collect::<String>();

            next_item = PARANS.replace(&next_item, "").to_string();

            let count = next_item.chars().count();
            if count < 3 || count > 38 || is_invalid(&next_item) {
                continue;
            }

            next_item = next_item
                .chars()
                .map(|c| CHARACTER_MAP.get(&c).unwrap_or(&c).clone())
                .collect();

            if let Some(i) = end_index {
                next_item.push(*chars.get(i).unwrap());
            }
            return Some(next_item.trim().to_string());
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_split_after() {
        let value = "唐王國。鹰号称铜。高等";
        let mut iter = SentenceExtractor::new(value);
        assert_eq!(iter.next().unwrap(), "鹰号称铜。");
        assert_eq!(iter.next().unwrap(), "高等");
        assert!(iter.next().is_none());
    }
}
