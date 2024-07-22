# Solutions for leetcode problems

<a align=center href=https://humanmademark.com/>
    <img src="https://humanmademark.com/white-logo.svg">
</a>

## Pre-commit

> build pre-commit cargo before !

```bash
#!/bin/bash
cd pre-commit
./target/release/pre-commit
cd -
git add README.md

cd rust
cargo fmt
IFS=$'\n'
for line in $(git status -s)
    do
        if [[ $line == A* || $line == M* ]]
        then
            if [[ $line == *.rs ]]
            then
                git add $(pwd)/${line:3}
            fi
        fi
    done
cd -
```

### Problems

![count](https://img.shields.io/badge/Solved-84-blue?style=for-the-badge)

<!-- table start -->
| Problem | Solution | Difficulty |
|---|---|---|
|1. [Two Sum](https://lcid.cc/1) | [Solution](../rust/src/solutions/p0001.rs) | Easy |
|4. [Median of Two Sorted Arrays](https://lcid.cc/4) | [Solution](../rust/src/solutions/p0004.rs) | Hard |
|6. [Zigzag Conversion](https://lcid.cc/6) | [Solution](../rust/src/solutions/p0006.rs) | Medium |
|9. [Palindrome Number](https://lcid.cc/9) | [Solution](../rust/src/solutions/p0009.rs) | Easy |
|11. [Container With Most Water](https://lcid.cc/11) | [Solution](../rust/src/solutions/p0011.rs) | Medium |
|13. [Roman to Integer](https://lcid.cc/13) | [Solution](../rust/src/solutions/p0013.rs) | Easy |
|14. [Longest Common Prefix](https://lcid.cc/14) | [Solution](../rust/src/solutions/p0014.rs) | Easy |
|17. [Letter Combinations of a Phone Number](https://lcid.cc/17) | [Solution](../rust/src/solutions/p0017.rs) | Medium |
|20. [Valid Parentheses](https://lcid.cc/20) | [Solution](../rust/src/solutions/p0020.rs) | Easy |
|21. [Merge Two Sorted Lists](https://lcid.cc/21) | [Solution](../rust/src/solutions/p0021.rs) | Easy |
|26. [Remove Duplicates from Sorted Array](https://lcid.cc/26) | [Solution](../rust/src/solutions/p0026.rs) | Easy |
|27. [Remove Element](https://lcid.cc/27) | [Solution](../rust/src/solutions/p0027.rs) | Easy |
|28. [Find the Index of the First Occurrence in a String](https://lcid.cc/28) | [Solution](../rust/src/solutions/p0028.rs) | Easy |
|33. [Search in Rotated Sorted Array](https://lcid.cc/33) | [Solution](../rust/src/solutions/p0033.rs) | Medium |
|34. [Find First and Last Position of Element in Sorted Array](https://lcid.cc/34) | [Solution](../rust/src/solutions/p0034.rs) | Medium |
|36. [Valid Sudoku](https://lcid.cc/36) | [Solution](../rust/src/solutions/p0036.rs) | Medium |
|38. [Count and Say](https://lcid.cc/38) | [Solution](../rust/src/solutions/p0038.rs) | Medium |
|43. [Multiply Strings](https://lcid.cc/43) | [Solution](../rust/src/solutions/p0043.rs) | Medium |
|48. [Rotate Image](https://lcid.cc/48) | [Solution](../rust/src/solutions/p0048.rs) | Medium |
|49. [Group Anagrams](https://lcid.cc/49) | [Solution](../rust/src/solutions/p0049.rs) | Medium |
|54. [Spiral Matrix](https://lcid.cc/54) | [Solution](../rust/src/solutions/p0054.rs) | Medium |
|55. [Jump Game](https://lcid.cc/55) | [Solution](../rust/src/solutions/p0055.rs) | Medium |
|56. [Merge Intervals](https://lcid.cc/56) | [Solution](../rust/src/solutions/p0056.rs) | Medium |
|58. [Length of Last Word](https://lcid.cc/58) | [Solution](../rust/src/solutions/p0058.rs) | Easy |
|59. [Spiral Matrix II](https://lcid.cc/59) | [Solution](../rust/src/solutions/p0059.rs) | Medium |
|66. [Plus One](https://lcid.cc/66) | [Solution](../rust/src/solutions/p0066.rs) | Easy |
|70. [Climbing Stairs](https://lcid.cc/70) | [Solution](../rust/src/solutions/p0070.rs) | Easy |
|71. [Simplify Path](https://lcid.cc/71) | [Solution](../rust/src/solutions/p0071.rs) | Medium |
|80. [Remove Duplicates from Sorted Array II](https://lcid.cc/80) | [Solution](../rust/src/solutions/p0080.rs) | Medium |
|88. [Merge Sorted Array](https://lcid.cc/88) | [Solution](../rust/src/solutions/p0088.rs) | Easy |
|100. [Same Tree](https://lcid.cc/100) | [Solution](../rust/src/solutions/p0100.rs) | Easy |
|104. [Maximum Depth of Binary Tree](https://lcid.cc/104) | [Solution](../rust/src/solutions/p0104.rs) | Easy |
|121. [Best Time to Buy and Sell Stock](https://lcid.cc/121) | [Solution](../rust/src/solutions/p0121.rs) | Easy |
|122. [Best Time to Buy and Sell Stock II](https://lcid.cc/122) | [Solution](../rust/src/solutions/p0122.rs) | Medium |
|125. [Valid Palindrome](https://lcid.cc/125) | [Solution](../rust/src/solutions/p0125.rs) | Easy |
|136. [Single Number](https://lcid.cc/136) | [Solution](../rust/src/solutions/p0136.rs) | Easy |
|151. [Reverse Words in a String](https://lcid.cc/151) | [Solution](../rust/src/solutions/p0151.rs) | Medium |
|167. [Two Sum II - Input Array Is Sorted](https://lcid.cc/167) | [Solution](../rust/src/solutions/p0167.rs) | Medium |
|169. [Majority Element](https://lcid.cc/169) | [Solution](../rust/src/solutions/p0169.rs) | Easy |
|172. [Factorial Trailing Zeroes](https://lcid.cc/172) | [Solution](../rust/src/solutions/p0172.rs) | Medium |
|189. [Rotate Array](https://lcid.cc/189) | [Solution](../rust/src/solutions/p0189.rs) | Medium |
|198. [House Robber](https://lcid.cc/198) | [Solution](../rust/src/solutions/p0198.rs) | Medium |
|202. [Happy Number](https://lcid.cc/202) | [Solution](../rust/src/solutions/p0202.rs) | Easy |
|203. [Remove Linked List Elements](https://lcid.cc/203) | [Solution](../rust/src/solutions/p0203.rs) | Easy |
|205. [Isomorphic Strings](https://lcid.cc/205) | [Solution](../rust/src/solutions/p0205.rs) | Easy |
|215. [Kth Largest Element in an Array](https://lcid.cc/215) | [Solution](../rust/src/solutions/p0215.rs) | Medium |
|217. [Contains Duplicate](https://lcid.cc/217) | [Solution](../rust/src/solutions/p0217.rs) | Easy |
|219. [Contains Duplicate II](https://lcid.cc/219) | [Solution](../rust/src/solutions/p0219.rs) | Easy |
|226. [Invert Binary Tree](https://lcid.cc/226) | [Solution](../rust/src/solutions/p0226.rs) | Easy |
|228. [Summary Ranges](https://lcid.cc/228) | [Solution](../rust/src/solutions/p0228.rs) | Easy |
|242. [Valid Anagram](https://lcid.cc/242) | [Solution](../rust/src/solutions/p0242.rs) | Easy |
|274. [H-Index](https://lcid.cc/274) | [Solution](../rust/src/solutions/p0274.rs) | Medium |
|290. [Word Pattern](https://lcid.cc/290) | [Solution](../rust/src/solutions/p0290.rs) | Easy |
|328. [Odd Even Linked List](https://lcid.cc/328) | [Solution](../rust/src/solutions/p0328.rs) | Medium |
|334. [Increasing Triplet Subsequence](https://lcid.cc/334) | [Solution](../rust/src/solutions/p0334.rs) | Medium |
|338. [Counting Bits](https://lcid.cc/338) | [Solution](../rust/src/solutions/p0338.rs) | Easy |
|383. [Ransom Note](https://lcid.cc/383) | [Solution](../rust/src/solutions/p0383.rs) | Easy |
|392. [Is Subsequence](https://lcid.cc/392) | [Solution](../rust/src/solutions/p0392.rs) | Easy |
|394. [Decode String](https://lcid.cc/394) | [Solution](../rust/src/solutions/p0394.rs) | Medium |
|443. [String Compression](https://lcid.cc/443) | [Solution](../rust/src/solutions/p0443.rs) | Medium |
|485. [Max Consecutive Ones](https://lcid.cc/485) | [Solution](../rust/src/solutions/p0485.rs) | Easy |
|649. [Dota2 Senate](https://lcid.cc/649) | [Solution](../rust/src/solutions/p0649.rs) | Medium |
|700. [Search in a Binary Search Tree](https://lcid.cc/700) | [Solution](../rust/src/solutions/p0700.rs) | Easy |
|724. [Find Pivot Index](https://lcid.cc/724) | [Solution](../rust/src/solutions/p0724.rs) | Easy |
|735. [Asteroid Collision](https://lcid.cc/735) | [Solution](../rust/src/solutions/p0735.rs) | Medium |
|746. [Min Cost Climbing Stairs](https://lcid.cc/746) | [Solution](../rust/src/solutions/p0746.rs) | Easy |
|872. [Leaf-Similar Trees](https://lcid.cc/872) | [Solution](../rust/src/solutions/p0872.rs) | Easy |
|1004. [Max Consecutive Ones III](https://lcid.cc/1004) | [Solution](../rust/src/solutions/p1004.rs) | Medium |
|1071. [Greatest Common Divisor of Strings](https://lcid.cc/1071) | [Solution](../rust/src/solutions/p1071.rs) | Easy |
|1137. [N-th Tribonacci Number](https://lcid.cc/1137) | [Solution](../rust/src/solutions/p1137.rs) | Easy |
|1448. [Count Good Nodes in Binary Tree](https://lcid.cc/1448) | [Solution](../rust/src/solutions/p1448.rs) | Medium |
|1456. [Maximum Number of Vowels in a Substring of Given Length](https://lcid.cc/1456) | [Solution](../rust/src/solutions/p1456.rs) | Medium |
|1493. [Longest Subarray of 1's After Deleting One Element](https://lcid.cc/1493) | [Solution](../rust/src/solutions/p1493.rs) | Medium |
|1550. [Three Consecutive Odds](https://lcid.cc/1550) | [Solution](../rust/src/solutions/p1550.rs) | Easy |
|1657. [Determine if Two Strings Are Close](https://lcid.cc/1657) | [Solution](../rust/src/solutions/p1657.rs) | Medium |
|1679. [Max Number of K-Sum Pairs](https://lcid.cc/1679) | [Solution](../rust/src/solutions/p1679.rs) | Medium |
|2095. [Delete the Middle Node of a Linked List](https://lcid.cc/2095) | [Solution](../rust/src/solutions/p2095.rs) | Medium |
|2130. [Maximum Twin Sum of a Linked List](https://lcid.cc/2130) | [Solution](../rust/src/solutions/p2130.rs) | Medium |
|2181. [Merge Nodes in Between Zeros](https://lcid.cc/2181) | [Solution](../rust/src/solutions/p2181.rs) | Medium |
|2352. [Equal Row and Column Pairs](https://lcid.cc/2352) | [Solution](../rust/src/solutions/p2352.rs) | Medium |
|2390. [Removing Stars From a String](https://lcid.cc/2390) | [Solution](../rust/src/solutions/p2390.rs) | Medium |
|2418. [Sort the People](https://lcid.cc/2418) | [Solution](../rust/src/solutions/p2418.rs) | Easy |
|2769. [Find the Maximum Achievable Number](https://lcid.cc/2769) | [Solution](../rust/src/solutions/p2769.rs) | Easy |
|3110. [Score of a String](https://lcid.cc/3110) | [Solution](../rust/src/solutions/p3110.rs) | Easy |
<!-- table end -->
