pub struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let matcher = std::collections::HashMap::from([(')', '('), (']', '['), ('}', '{')]);
        let mut opened_stack: Vec<char> = vec![];
        for i in s.chars() {
            if "({[".contains(i) {
                opened_stack.push(i);
            } else {
                let latest_opened = match opened_stack.pop() {
                    Some(value) => value,
                    _ => return false,
                };
                if matcher.get(&i).unwrap() != &latest_opened {
                    return false;
                }
            }
        }
        opened_stack.is_empty()
    }
}
