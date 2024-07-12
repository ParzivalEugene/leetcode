pub struct Solution;

impl Solution {
    pub fn predict_party_victory(mut senate: String) -> String {
        while senate.len() != 1 {
            let senator = senate.remove(0);
            match senator {
                'R' => {
                    senate.remove(match senate.find('D') {
                        Some(value) => value,
                        None => return String::from("Radiant"),
                    });
                }
                _ => {
                    senate.remove(match senate.find('R') {
                        Some(value) => value,
                        None => return String::from("Dire"),
                    });
                }
            }
            senate.push(senator);
        }
        String::from(match senate.as_str() {
            "R" => "Radiant",
            _ => "Dire",
        })
    }
}
