pub struct Solution;

struct Nesting {
    multiplier: usize,
    state: String,
}

impl Nesting {
    pub fn new<S: AsRef<str>>(multiplier: usize, state: S) -> Self {
        Self {
            multiplier,
            state: state.as_ref().to_string(),
        }
    }
}

impl Solution {
    pub fn decode_string(s: String) -> String {
        let mut stack: Vec<Nesting> = vec![];
        let (mut multiplier, mut buffer) = (0 as usize, "".to_string());
        for c in s.chars() {
            match c {
                '0'..='9' => multiplier = multiplier * 10 + c.to_digit(10).unwrap() as usize,
                '[' => {
                    stack.push(Nesting::new(multiplier, &buffer));
                    multiplier = 0;
                    buffer.clear();
                }
                ']' => {
                    if let Some(last) = stack.pop() {
                        buffer = last.state + buffer.repeat(last.multiplier).as_str();
                    }
                }
                _ => buffer.push(c),
            }
        }
        buffer
    }
}
