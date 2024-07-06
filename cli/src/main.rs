use clap::{Parser, Subcommand};

const RUST_PATH: &str = "/home/parzival/projects/personal/leetcode/rust/src";

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    commmand: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(arg_required_else_help = true)]
    Add { id: u32 },
}

fn exists(id: &str) -> bool {
    std::fs::metadata(format!("{}/solutions/p{}.rs", RUST_PATH, id)).is_ok()
}

fn add_solution(raw_id: u32) {
    let id = format!("{:0>4}", raw_id);

    if exists(&id) {
        eprintln!("Solution for problem {} already exists", id);
        return;
    }

    let path = format!("{}/solutions/p{}.rs", RUST_PATH, id);
    let test_path = format!("{}/tests/p{}.rs", RUST_PATH, id);

    std::fs::write(&path, "pub struct Solution;").unwrap();
    std::fs::write(
        &test_path,
        format!(
            "#[cfg(test)]
mod test {{
    use crate::solutions::p{}::Solution;
    
    #[test]
    fn leetcode_case_1() {{
        
    }}
}}",
            id
        ),
    )
    .unwrap();

    let mut solutions = std::fs::read_to_string(format!("{}/solutions/mod.rs", RUST_PATH)).unwrap();
    solutions.push_str(&format!("pub mod p{};\n", id));
    std::fs::write(format!("{}/solutions/mod.rs", RUST_PATH), solutions).unwrap();

    let mut tests = std::fs::read_to_string(format!("{}/tests/mod.rs", RUST_PATH)).unwrap();
    tests.push_str(&format!("pub mod p{};\n", id));
    std::fs::write(format!("{}/tests/mod.rs", RUST_PATH), tests).unwrap();

    std::process::Command::new("code")
        .arg(&path)
        .arg(&test_path)
        .spawn()
        .unwrap();
}

fn main() {
    let args = Args::parse();

    match args.commmand {
        Commands::Add { id } => {
            add_solution(id);
        }
    }
}
