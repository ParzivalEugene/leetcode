use crate::data::Problem;

pub fn problem_info(id: u16) -> Problem {
    let response = match reqwest::blocking::get(format!("https://lcid.cc/info/{}", id)) {
        Ok(response) => response,
        Err(_) => {
            return Problem {
                id,
                title: "ERROR".to_string(),
                difficulty: "ERROR".to_string(),
                url: format!("https://lcid.cc/{}", id),
            }
        }
    };

    let json: serde_json::Value = response.json().unwrap();

    Problem {
        id,
        title: json["title"].as_str().unwrap().to_string(),
        difficulty: json["difficulty"].as_str().unwrap().to_string(),
        url: format!("https://lcid.cc/{}", id),
    }
}
