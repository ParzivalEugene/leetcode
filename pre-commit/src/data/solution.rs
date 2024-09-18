use crate::enums::Language;

pub struct Solution {
    pub id: u16,
    pub languages: Vec<Language>,
}

impl Solution {
    pub fn new(id: u16, languages: Vec<Language>) -> Self {
        Solution { id, languages }
    }
}
