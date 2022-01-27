use std::hash::Hash;

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Language {
    pub extensions: Vec<String>,
    pub name: String,
}

impl Language {
    pub fn new(name: &str, extensions: Vec<&str>) -> Language {
        Language {
            extensions: extensions.iter().map(|e| e.to_string()).collect(),
            name: name.to_string()
        }
    }
}

const TYPESCRIPT: Language = Language::new("Typescript", vec!["ts", "tsx"]);
const JAVASCRIPT: Language = Language::new("Javascript", vec!["js", "jsx"]);
const JSON: Language = Language::new("JSON", vec!["json"]);

pub const LANGUAGES: Vec<Language> = vec![TYPESCRIPT, JAVASCRIPT, JSON];
