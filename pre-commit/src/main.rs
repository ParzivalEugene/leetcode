use std::{
    collections::{HashMap, HashSet},
    fs, u32,
};

#[derive(Hash, Eq, PartialEq)]
enum Languages {
    All,
    Rust,
}

#[derive(Debug, Clone)]
struct Problem {
    id: u32,
    title: String,
    difficulty: String,
    url: String,
}

impl std::fmt::Display for Problem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "|{}. [{}]({}) | [Solution](../rust/src/solutions/p{:0>4}.rs) | {} |",
            self.id, self.title, self.url, self.id, self.difficulty
        )
    }
}

const RUST_PATH: &str = "../rust/src/solutions/";
const README_PATH: &str = "../README.md";

fn get_rust_solutions() -> Vec<String> {
    let mut solutions = Vec::new();
    let paths = fs::read_dir(RUST_PATH).unwrap();
    for path in paths {
        let path = path.unwrap().path();
        if path.is_file() {
            let name = String::from(path.file_name().unwrap().to_str().unwrap());
            if name.starts_with("p") && name.ends_with(".rs") {
                solutions.push(name[1..name.len() - 3].trim_start_matches("0").to_string());
            }
        }
    }
    solutions
}

fn get_info_about_problem(id: u32) -> Problem {
    let response = match reqwest::blocking::get(&format!("https://lcid.cc/info/{}", id)) {
        Ok(response) => response,
        Err(_) => {
            return Problem {
                id: id,
                title: "ERROR".to_string(),
                difficulty: "ERROR".to_string(),
                url: format!("https://lcid.cc/{}", id),
            }
        }
    };

    let json: serde_json::Value = response.json().unwrap();

    Problem {
        id: id,
        title: json["title"].as_str().unwrap().to_string(),
        difficulty: json["difficulty"].as_str().unwrap().to_string(),
        url: format!("https://lcid.cc/{}", id),
    }
}

fn read_table() -> Vec<Problem> {
    let mut problems = Vec::new();

    let readme = fs::read_to_string(README_PATH)
        .expect("Unable to read file")
        .split("<!-- table start -->")
        .collect::<Vec<&str>>()[1]
        .split("<!-- table end -->")
        .collect::<Vec<&str>>()[0]
        .to_owned();

    let mut table = readme.split("\n").collect::<Vec<&str>>();
    table = table[3..table.len() - 1].to_vec();

    for line in table {
        let chunks = line.split('|').collect::<Vec<&str>>();
        let difficulty = chunks[chunks.len() - 2].trim();
        let (id_raw, block) = chunks[1].trim().split_once(". ").unwrap();
        let id = id_raw.parse::<u32>().unwrap();
        let (mut name, mut url) = block.split_once("(").unwrap();
        name = &name[1..name.len() - 1];
        url = &url[0..url.len() - 1];

        problems.push(Problem {
            id: id,
            title: name.to_string(),
            difficulty: difficulty.to_string(),
            url: url.to_string(),
        });
    }

    problems
}

fn get_solved_problems(problems: &Vec<Problem>) -> HashSet<u32> {
    let mut ids = HashSet::new();
    for problem in problems {
        ids.insert(problem.id);
    }
    ids
}

fn get_not_added_solutions(solutions: &Vec<String>, solved_problems: &HashSet<u32>) -> Vec<String> {
    let mut not_added_solutions = Vec::new();
    for solution in solutions {
        let id = solution.parse::<u32>().unwrap();
        if !solved_problems.contains(&id) {
            not_added_solutions.push(solution.clone());
        }
    }
    not_added_solutions
}

fn update_problems(problems: &mut Vec<Problem>, not_added_solutions: &Vec<String>) {
    for solution in not_added_solutions {
        let id = solution.parse::<u32>().unwrap();
        let problem = get_info_about_problem(id);
        problems.push(problem);
    }
}

fn update_table(problems: &mut Vec<Problem>) {
    problems.sort_by(|a, b| a.id.cmp(&b.id));

    let mut table = String::new();
    for problem in problems {
        table.push_str(&format!("{}\n", problem));
    }

    let readme = fs::read_to_string(README_PATH).expect("Unable to read file");
    let new_readme = format!(
        "{}<!-- table start -->\n| Problem | Solution | Difficulty |\n|---|---|---|\n{}<!-- table end -->{}",
        readme.split("<!-- table start -->").collect::<Vec<&str>>()[0],
        table,
        readme.split("<!-- table end -->").collect::<Vec<&str>>()[1]
    );

    fs::write(README_PATH, new_readme).expect("Unable to write file");
}

fn main() {
    let solutions: HashMap<Languages, Vec<String>> = HashMap::from([
        (Languages::All, get_rust_solutions()),
        (Languages::Rust, get_rust_solutions()),
    ]);
    let mut problems = read_table();

    let solved_problems = get_solved_problems(&problems);
    let not_added_solutions =
        get_not_added_solutions(&solutions[&Languages::Rust], &solved_problems);
    update_problems(&mut problems, &not_added_solutions);
    update_table(&mut problems);
}
