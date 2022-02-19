use std::{cmp::Ordering, hash::Hash};

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct Language {
    pub extensions: Vec<String>,
    pub name: String,
    pub color: String,
}

impl Language {
    pub fn new(name: &str, extensions: Vec<&str>, color: &str) -> Language {
        Language {
            extensions: extensions.iter().map(|e| e.to_string()).collect(),
            name: name.to_string(),
            color: String::from(color),
        }
    }
}

impl Ord for Language {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
    }
}

impl PartialOrd for Language {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Colours generated using https://mokole.com/palette.html
pub fn get_extensions() -> Vec<Language> {
    vec![
        Language::new("Typescript", vec!["ts", "tsx"], "#7f0000"),
        Language::new("Javascript", vec!["js", "jsx"], "#696969"),
        Language::new("Swift", vec!["swift"], "#808000"),
        Language::new("Objective-C", vec!["h", "m"], "#3cb371"),
        Language::new("Shell", vec!["sh", "zsh"], "#00008b"),
        Language::new("C#", vec!["cs", "cake"], "#ff0000"),
        Language::new("Java", vec!["java"], "#ff8c00"),
        Language::new("Kotlin", vec!["kt"], "#ffd700"),
        Language::new("Rust", vec!["rs"], "#ba55d3"),
        Language::new("Go", vec!["go"], "#00ff7f"),
        Language::new("Ruby", vec!["rb"], "#e9967a"),
        Language::new("Python", vec!["py"], "#00ffff"),
        Language::new("C", vec!["c"], "#eee8aa"),
        Language::new("HTML", vec!["html", "htm", "xhtml"], "#adff2f"),
        Language::new("CSS", vec!["css", "sass", "scss"], "#ff00ff"),
        Language::new("SQL", vec!["sql"], "#ff1493"),
    ]
}
