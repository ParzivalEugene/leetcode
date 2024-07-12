# Solutions for leetcode problems

<a align=center href=https://humanmademark.com/>
    <img src="https://humanmademark.com/white-logo.svg">
</a>

## Pre-commit

> build pre-commit cargo before !

```bash
cd pre-commit
./target/release/pre-commit
cd -
git add README.md

cd rust
IFS=$'\n'
for line in $(git status -s)
    do
        if [[ $line == A* || $line == M* ]]
        then
            if [[ $line == *.rs ]]
            then
                cargo fmt
                git add $(pwd)/${line:3}
            fi
        fi
    done
cd -
```

<!-- table start -->
| Problem | Solution | Difficulty |
|---|---|---|
|1. [Two Sum](https://lcid.cc/1) | [Solution](../rust/src/solutions/p0001.rs) | Easy |
|6. [Zigzag Conversion](https://lcid.cc/6) | [Solution](../rust/src/solutions/p0006.rs) | Medium |
|9. [Palindrome Number](https://lcid.cc/9) | [Solution](../rust/src/solutions/p0009.rs) | Easy |
|11. [Container With Most Water](https://lcid.cc/11) | [Solution](../rust/src/solutions/p0011.rs) | Medium |
|13. [Roman to Integer](https://lcid.cc/13) | [Solution](../rust/src/solutions/p0013.rs) | Easy |
|14. [Longest Common Prefix](https://lcid.cc/14) | [Solution](../rust/src/solutions/p0014.rs) | Easy |
|20. [Valid Parentheses](https://lcid.cc/20) | [Solution](../rust/src/solutions/p0020.rs) | Easy |
|21. [Merge Two Sorted Lists](https://lcid.cc/21) | [Solution](../rust/src/solutions/p0021.rs) | Easy |
|26. [Remove Duplicates from Sorted Array](https://lcid.cc/26) | [Solution](../rust/src/solutions/p0026.rs) | Easy |
|27. [Remove Element](https://lcid.cc/27) | [Solution](../rust/src/solutions/p0027.rs) | Easy |
|28. [Find the Index of the First Occurrence in a String](https://lcid.cc/28) | [Solution](../rust/src/solutions/p0028.rs) | Easy |
|36. [Valid Sudoku](https://lcid.cc/36) | [Solution](../rust/src/solutions/p0036.rs) | Medium |
|49. [Group Anagrams](https://lcid.cc/49) | [Solution](../rust/src/solutions/p0049.rs) | Medium |
|54. [Spiral Matrix](https://lcid.cc/54) | [Solution](../rust/src/solutions/p0054.rs) | Medium |
|55. [Jump Game](https://lcid.cc/55) | [Solution](../rust/src/solutions/p0055.rs) | Medium |
|58. [Length of Last Word](https://lcid.cc/58) | [Solution](../rust/src/solutions/p0058.rs) | Easy |
|66. [Plus One](https://lcid.cc/66) | [Solution](../rust/src/solutions/p0066.rs) | Easy |
|70. [Climbing Stairs](https://lcid.cc/70) | [Solution](../rust/src/solutions/p0070.rs) | Easy |
|71. [Simplify Path](https://lcid.cc/71) | [Solution](../rust/src/solutions/p0071.rs) | Medium |
|80. [Remove Duplicates from Sorted Array II](https://lcid.cc/80) | [Solution](../rust/src/solutions/p0080.rs) | Medium |
|88. [Merge Sorted Array](https://lcid.cc/88) | [Solution](../rust/src/solutions/p0088.rs) | Easy |
|104. [Maximum Depth of Binary Tree](https://lcid.cc/104) | [Solution](../rust/src/solutions/p0104.rs) | Easy |
|121. [Best Time to Buy and Sell Stock](https://lcid.cc/121) | [Solution](../rust/src/solutions/p0121.rs) | Easy |
|122. [Best Time to Buy and Sell Stock II](https://lcid.cc/122) | [Solution](../rust/src/solutions/p0122.rs) | Medium |
|125. [Valid Palindrome](https://lcid.cc/125) | [Solution](../rust/src/solutions/p0125.rs) | Easy |
|151. [Reverse Words in a String](https://lcid.cc/151) | [Solution](../rust/src/solutions/p0151.rs) | Medium |
|167. [Two Sum II - Input Array Is Sorted](https://lcid.cc/167) | [Solution](../rust/src/solutions/p0167.rs) | Medium |
|169. [Majority Element](https://lcid.cc/169) | [Solution](../rust/src/solutions/p0169.rs) | Easy |
|172. [Factorial Trailing Zeroes](https://lcid.cc/172) | [Solution](../rust/src/solutions/p0172.rs) | Medium |
|189. [Rotate Array](https://lcid.cc/189) | [Solution](../rust/src/solutions/p0189.rs) | Medium |
|202. [Happy Number](https://lcid.cc/202) | [Solution](../rust/src/solutions/p0202.rs) | Easy |
|205. [Isomorphic Strings](https://lcid.cc/205) | [Solution](../rust/src/solutions/p0205.rs) | Easy |
|217. [Contains Duplicate](https://lcid.cc/217) | [Solution](../rust/src/solutions/p0217.rs) | Easy |
|219. [Contains Duplicate II](https://lcid.cc/219) | [Solution](../rust/src/solutions/p0219.rs) | Easy |
|228. [Summary Ranges](https://lcid.cc/228) | [Solution](../rust/src/solutions/p0228.rs) | Easy |
|242. [Valid Anagram](https://lcid.cc/242) | [Solution](../rust/src/solutions/p0242.rs) | Easy |
|274. [H-Index](https://lcid.cc/274) | [Solution](../rust/src/solutions/p0274.rs) | Medium |
|290. [Word Pattern](https://lcid.cc/290) | [Solution](../rust/src/solutions/p0290.rs) | Easy |
|334. [Increasing Triplet Subsequence](https://lcid.cc/334) | [Solution](../rust/src/solutions/p0334.rs) | Medium |
|383. [Ransom Note](https://lcid.cc/383) | [Solution](../rust/src/solutions/p0383.rs) | Easy |
|392. [Is Subsequence](https://lcid.cc/392) | [Solution](../rust/src/solutions/p0392.rs) | Easy |
|443. [String Compression](https://lcid.cc/443) | [Solution](../rust/src/solutions/p0443.rs) | Medium |
|724. [Find Pivot Index](https://lcid.cc/724) | [Solution](../rust/src/solutions/p0724.rs) | Easy |
|1071. [Greatest Common Divisor of Strings](https://lcid.cc/1071) | [Solution](../rust/src/solutions/p1071.rs) | Easy |
|1550. [Three Consecutive Odds](https://lcid.cc/1550) | [Solution](../rust/src/solutions/p1550.rs) | Easy |
|1679. [Max Number of K-Sum Pairs](https://lcid.cc/1679) | [Solution](../rust/src/solutions/p1679.rs) | Medium |
<!-- table end -->
