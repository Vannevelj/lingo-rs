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
    let javascript = Language::new("Javascript", vec!["js", "jsx"]);
    let json = Language::new("JSON", vec!["json"]);
    let yaml = Language::new("YAML", vec!["yml"]);
    let toml = Language::new("TOML", vec!["toml"]);
    let swift = Language::new("Swift", vec!["swift"]);
    let objective_c = Language::new("Objective-C", vec!["h", "m"]);
    let markdown = Language::new("Markdown", vec!["md"]);
    let shell = Language::new("Shell", vec!["sh", "zsh"]);
    let c_sharp = Language::new("C#", vec!["cs", "cake"]);
    let java = Language::new("Java", vec!["java"]);
    let kotlin = Language::new("Kotlin", vec!["kt"]);

    vec![
        typescript,
        javascript,
        json,
        swift,
        objective_c,
        markdown,
        yaml,
        toml,
        shell,
        c_sharp,
        java,
        kotlin,
    ]
}
