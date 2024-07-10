pub struct Solution;

impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut dir_stack: Vec<&str> = vec![];
        for dir in path.split('/') {
            match dir {
                "" | "." => continue,
                ".." => {dir_stack.pop();},
                _ => dir_stack.push(dir)    
            }
            
        }
        format!("/{}", dir_stack.join("/"))
    }
}
