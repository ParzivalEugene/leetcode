pub struct Solution;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return vec![];
        }
        let table = std::collections::HashMap::from([
            ('2', "abc"),
            ('3', "def"),
            ('4', "ghi"),
            ('5', "jkl"),
            ('6', "mno"),
            ('7', "pqrs"),
            ('8', "tuv"),
            ('9', "wxyz"),
        ]);

        fn backtrack(
            table: &std::collections::HashMap<char, &str>,
            digits: &str,
            current: String,
            combinations: &mut Vec<String>,
        ) {
            if digits.is_empty() {
                combinations.push(current);
                return;
            }

            let digit = digits.chars().next().unwrap();
            if let Some(letters) = table.get(&digit) {
                for letter in letters.chars() {
                    backtrack(
                        table,
                        &digits[1..],
                        current.clone() + &letter.to_string(),
                        combinations,
                    )
                }
            }
        }

        let mut combinations = Vec::new();
        backtrack(&table, &digits, String::new(), &mut combinations);
        combinations
    }
}
