use std::{hash::Hash, cmp::Ordering};

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

pub fn get_extensions() -> Vec<Language> {
    let typescript = Language::new("Typescript", vec!["ts", "tsx"]);
    let javascript = Language::new("Javascript", vec!["js", "jsx"]);
    let swift = Language::new("Swift", vec!["swift"]);
    let objective_c = Language::new("Objective-C", vec!["h", "m"]);
    let markdown = Language::new("Markdown", vec!["md"]);
    let shell = Language::new("Shell", vec!["sh", "zsh"]);
    let c_sharp = Language::new("C#", vec!["cs", "cake"]);
    let java = Language::new("Java", vec!["java"]);
    let kotlin = Language::new("Kotlin", vec!["kt"]);
    let rust = Language::new("Rust", vec!["rs"]);
    let go = Language::new("Go", vec!["go"]);
    let ruby = Language::new("Ruby", vec!["rb"]);
    let python = Language::new("Python", vec!["py"]);
    let c = Language::new("C", vec!["c"]);
    let html = Language::new("HTML", vec!["html", "htm", "xhtml"]);
    let css = Language::new("CSS", vec!["css", "sass", "scss"]);
    let sql = Language::new("SQL", vec!["sql"]);
    let cucumber = Language::new("Cucumber", vec!["feature"]);

    vec![
        typescript,
        javascript,
        swift,
        objective_c,
        markdown,
        shell,
        c_sharp,
        java,
        kotlin,
        rust,
        go,
        ruby,
        python,
        c,
        html,
        css,
        sql,
        cucumber,
    ]
}
