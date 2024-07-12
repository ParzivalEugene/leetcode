#[cfg(test)]
mod test {
    use crate::solutions::p0746::Solution;

    #[test]
    fn leetcode_case_1() {
        assert_eq!(Solution::min_cost_climbing_stairs(vec![10, 15, 20]), 15);
    }
}
