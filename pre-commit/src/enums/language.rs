use crate::constants;

#[derive(Hash, Eq, PartialEq, Clone)]
#[allow(clippy::upper_case_acronyms)]
pub enum Language {
    Python,
    Rust,
    SQL,
}

impl From<&str> for Language {
    fn from(s: &str) -> Self {
        match s {
            "rust" => Language::Rust,
            "sql" => Language::SQL,
            _ => panic!("Invalid language"),
        }
    }
}

impl From<String> for Language {
    fn from(s: String) -> Self {
        Self::from(s.as_str())
    }
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Language::Rust => write!(f, "rust"),
            Language::SQL => write!(f, "sql"),
            Language::Python => write!(f, "python"),
        }
    }
}

impl Language {
    pub fn from_extension(extension: &str) -> Self {
        match extension {
            "rs" => Language::Rust,
            "sql" => Language::SQL,
            "py" => Language::Python,
            _ => panic!("Invalid extension {}", extension),
        }
    }

    pub fn to_readme(&self, id: &u16) -> String {
        match self {
            Language::Rust => format!("[rust]({}p{:0>4}.rs)", &constants::RUST_PATH[3..], id),
            Language::SQL => format!("[sql]({}p{:0>4}.sql)", &constants::SQL_PATH[3..], id),
            Language::Python => format!("[python]({}p{:0>4}.py)", &constants::PYTHON_PATH[3..], id),
        }
    }

    pub fn to_badge(&self, count: u32) -> String {
        match self {
            Language::Rust => format!("![rust](https://img.shields.io/badge/rust-{}-000000?logo=rust&style=for-the-badge)", count),
            Language::SQL => format!("![sql](https://img.shields.io/badge/sql-{}-4479A1?logo=mysql&style=for-the-badge)", count),
            Language::Python => format!("![python](https://img.shields.io/badge/python-{}-3776AB?logo=python&style=for-the-badge)", count),
        }
    }
}
