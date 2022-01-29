use std::hash::Hash;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct Language {
    pub extensions: Vec<String>,
    pub name: String,
}

impl Language {
    pub fn new(name: &str, extensions: Vec<&str>) -> Language {
        Language {
            extensions: extensions.iter().map(|e| e.to_string()).collect(),
            name: name.to_string(),
        }
    }
}

pub fn get_extensions() -> Vec<Language> {
    let typescript = Language::new("Typescript", vec!["ts", "tsx"]);
    let javascript: Language = Language::new("Javascript", vec!["js", "jsx"]);
    let json: Language = Language::new("JSON", vec!["json"]);

    vec![typescript, javascript, json]
}
