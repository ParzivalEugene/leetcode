# Solutions for leetcode problems

<a align=center href=https://brainmade.org//>
    <img src="https://brainmade.org/white-logo.svg">
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
<!-- counters start -->
![all](https://img.shields.io/badge/solved-118-blue?style=for-the-badge) ![python](https://img.shields.io/badge/python-5-3776AB?logo=python&style=for-the-badge) ![rust](https://img.shields.io/badge/rust-104-000000?logo=rust&style=for-the-badge) ![sql](https://img.shields.io/badge/sql-12-4479A1?logo=mysql&style=for-the-badge)
<!-- counters end -->
<!-- table start -->
| Problem | Solution | Difficulty |
|---|---|---|
| 1. [Two Sum](https://lcid.cc/1) | [rust](rust/src/solutions/p0001.rs) | Easy |
| 4. [Median of Two Sorted Arrays](https://lcid.cc/4) | [rust](rust/src/solutions/p0004.rs) | Hard |
| 6. [Zigzag Conversion](https://lcid.cc/6) | [rust](rust/src/solutions/p0006.rs) | Medium |
| 9. [Palindrome Number](https://lcid.cc/9) | [rust](rust/src/solutions/p0009.rs) | Easy |
| 11. [Container With Most Water](https://lcid.cc/11) | [rust](rust/src/solutions/p0011.rs) | Medium |
| 13. [Roman to Integer](https://lcid.cc/13) | [rust](rust/src/solutions/p0013.rs) | Easy |
| 14. [Longest Common Prefix](https://lcid.cc/14) | [rust](rust/src/solutions/p0014.rs), [python](python/solutions/p0014.py) | Easy |
| 17. [Letter Combinations of a Phone Number](https://lcid.cc/17) | [rust](rust/src/solutions/p0017.rs) | Medium |
| 20. [Valid Parentheses](https://lcid.cc/20) | [rust](rust/src/solutions/p0020.rs) | Easy |
| 21. [Merge Two Sorted Lists](https://lcid.cc/21) | [rust](rust/src/solutions/p0021.rs) | Easy |
| 26. [Remove Duplicates from Sorted Array](https://lcid.cc/26) | [rust](rust/src/solutions/p0026.rs) | Easy |
| 27. [Remove Element](https://lcid.cc/27) | [rust](rust/src/solutions/p0027.rs) | Easy |
| 28. [Find the Index of the First Occurrence in a String](https://lcid.cc/28) | [rust](rust/src/solutions/p0028.rs) | Easy |
| 33. [Search in Rotated Sorted Array](https://lcid.cc/33) | [rust](rust/src/solutions/p0033.rs) | Medium |
| 34. [Find First and Last Position of Element in Sorted Array](https://lcid.cc/34) | [rust](rust/src/solutions/p0034.rs) | Medium |
| 36. [Valid Sudoku](https://lcid.cc/36) | [rust](rust/src/solutions/p0036.rs) | Medium |
| 38. [Count and Say](https://lcid.cc/38) | [rust](rust/src/solutions/p0038.rs) | Medium |
| 43. [Multiply Strings](https://lcid.cc/43) | [rust](rust/src/solutions/p0043.rs) | Medium |
| 48. [Rotate Image](https://lcid.cc/48) | [rust](rust/src/solutions/p0048.rs) | Medium |
| 49. [Group Anagrams](https://lcid.cc/49) | [rust](rust/src/solutions/p0049.rs) | Medium |
| 54. [Spiral Matrix](https://lcid.cc/54) | [rust](rust/src/solutions/p0054.rs) | Medium |
| 55. [Jump Game](https://lcid.cc/55) | [rust](rust/src/solutions/p0055.rs) | Medium |
| 56. [Merge Intervals](https://lcid.cc/56) | [rust](rust/src/solutions/p0056.rs) | Medium |
| 58. [Length of Last Word](https://lcid.cc/58) | [rust](rust/src/solutions/p0058.rs) | Easy |
| 59. [Spiral Matrix II](https://lcid.cc/59) | [rust](rust/src/solutions/p0059.rs) | Medium |
| 66. [Plus One](https://lcid.cc/66) | [rust](rust/src/solutions/p0066.rs) | Easy |
| 70. [Climbing Stairs](https://lcid.cc/70) | [rust](rust/src/solutions/p0070.rs) | Easy |
| 71. [Simplify Path](https://lcid.cc/71) | [rust](rust/src/solutions/p0071.rs) | Medium |
| 80. [Remove Duplicates from Sorted Array II](https://lcid.cc/80) | [rust](rust/src/solutions/p0080.rs) | Medium |
| 88. [Merge Sorted Array](https://lcid.cc/88) | [rust](rust/src/solutions/p0088.rs) | Easy |
| 100. [Same Tree](https://lcid.cc/100) | [rust](rust/src/solutions/p0100.rs) | Easy |
| 104. [Maximum Depth of Binary Tree](https://lcid.cc/104) | [rust](rust/src/solutions/p0104.rs) | Easy |
| 121. [Best Time to Buy and Sell Stock](https://lcid.cc/121) | [rust](rust/src/solutions/p0121.rs) | Easy |
| 122. [Best Time to Buy and Sell Stock II](https://lcid.cc/122) | [rust](rust/src/solutions/p0122.rs) | Medium |
| 125. [Valid Palindrome](https://lcid.cc/125) | [rust](rust/src/solutions/p0125.rs) | Easy |
| 136. [Single Number](https://lcid.cc/136) | [rust](rust/src/solutions/p0136.rs) | Easy |
| 151. [Reverse Words in a String](https://lcid.cc/151) | [rust](rust/src/solutions/p0151.rs) | Medium |
| 167. [Two Sum II - Input Array Is Sorted](https://lcid.cc/167) | [rust](rust/src/solutions/p0167.rs) | Medium |
| 169. [Majority Element](https://lcid.cc/169) | [rust](rust/src/solutions/p0169.rs) | Easy |
| 172. [Factorial Trailing Zeroes](https://lcid.cc/172) | [rust](rust/src/solutions/p0172.rs) | Medium |
| 189. [Rotate Array](https://lcid.cc/189) | [rust](rust/src/solutions/p0189.rs) | Medium |
| 197. [Rising Temperature](https://lcid.cc/197) | [sql](sql/solutions/p0197.sql) | Easy |
| 198. [House Robber](https://lcid.cc/198) | [rust](rust/src/solutions/p0198.rs) | Medium |
| 202. [Happy Number](https://lcid.cc/202) | [rust](rust/src/solutions/p0202.rs) | Easy |
| 203. [Remove Linked List Elements](https://lcid.cc/203) | [rust](rust/src/solutions/p0203.rs) | Easy |
| 205. [Isomorphic Strings](https://lcid.cc/205) | [rust](rust/src/solutions/p0205.rs) | Easy |
| 215. [Kth Largest Element in an Array](https://lcid.cc/215) | [rust](rust/src/solutions/p0215.rs) | Medium |
| 217. [Contains Duplicate](https://lcid.cc/217) | [rust](rust/src/solutions/p0217.rs) | Easy |
| 219. [Contains Duplicate II](https://lcid.cc/219) | [rust](rust/src/solutions/p0219.rs) | Easy |
| 226. [Invert Binary Tree](https://lcid.cc/226) | [rust](rust/src/solutions/p0226.rs) | Easy |
| 228. [Summary Ranges](https://lcid.cc/228) | [rust](rust/src/solutions/p0228.rs), [python](python/solutions/p0228.py) | Easy |
| 242. [Valid Anagram](https://lcid.cc/242) | [rust](rust/src/solutions/p0242.rs) | Easy |
| 274. [H-Index](https://lcid.cc/274) | [rust](rust/src/solutions/p0274.rs) | Medium |
| 290. [Word Pattern](https://lcid.cc/290) | [rust](rust/src/solutions/p0290.rs) | Easy |
| 328. [Odd Even Linked List](https://lcid.cc/328) | [rust](rust/src/solutions/p0328.rs) | Medium |
| 334. [Increasing Triplet Subsequence](https://lcid.cc/334) | [rust](rust/src/solutions/p0334.rs) | Medium |
| 338. [Counting Bits](https://lcid.cc/338) | [rust](rust/src/solutions/p0338.rs) | Easy |
| 383. [Ransom Note](https://lcid.cc/383) | [rust](rust/src/solutions/p0383.rs) | Easy |
| 392. [Is Subsequence](https://lcid.cc/392) | [rust](rust/src/solutions/p0392.rs), [python](python/solutions/p0392.py) | Easy |
| 394. [Decode String](https://lcid.cc/394) | [rust](rust/src/solutions/p0394.rs) | Medium |
| 443. [String Compression](https://lcid.cc/443) | [rust](rust/src/solutions/p0443.rs) | Medium |
| 485. [Max Consecutive Ones](https://lcid.cc/485) | [rust](rust/src/solutions/p0485.rs) | Easy |
| 535. [Encode and Decode TinyURL](https://lcid.cc/535) | [rust](rust/src/solutions/p0535.rs) | Medium |
| 577. [Employee Bonus](https://lcid.cc/577) | [sql](sql/solutions/p0577.sql) | Easy |
| 584. [Find Customer Referee](https://lcid.cc/584) | [sql](sql/solutions/p0584.sql) | Easy |
| 595. [Big Countries](https://lcid.cc/595) | [sql](sql/solutions/p0595.sql) | Easy |
| 649. [Dota2 Senate](https://lcid.cc/649) | [rust](rust/src/solutions/p0649.rs) | Medium |
| 700. [Search in a Binary Search Tree](https://lcid.cc/700) | [rust](rust/src/solutions/p0700.rs) | Easy |
| 724. [Find Pivot Index](https://lcid.cc/724) | [rust](rust/src/solutions/p0724.rs) | Easy |
| 735. [Asteroid Collision](https://lcid.cc/735) | [rust](rust/src/solutions/p0735.rs) | Medium |
| 746. [Min Cost Climbing Stairs](https://lcid.cc/746) | [rust](rust/src/solutions/p0746.rs) | Easy |
| 771. [Jewels and Stones](https://lcid.cc/771) | [rust](rust/src/solutions/p0771.rs) | Easy |
| 840. [Magic Squares In Grid](https://lcid.cc/840) | [rust](rust/src/solutions/p0840.rs) | Medium |
| 872. [Leaf-Similar Trees](https://lcid.cc/872) | [rust](rust/src/solutions/p0872.rs) | Easy |
| 912. [Sort an Array](https://lcid.cc/912) | [rust](rust/src/solutions/p0912.rs) | Medium |
| 1004. [Max Consecutive Ones III](https://lcid.cc/1004) | [rust](rust/src/solutions/p1004.rs) | Medium |
| 1068. [Product Sales Analysis I](https://lcid.cc/1068) | [sql](sql/solutions/p1068.sql) | Easy |
| 1071. [Greatest Common Divisor of Strings](https://lcid.cc/1071) | [rust](rust/src/solutions/p1071.rs) | Easy |
| 1137. [N-th Tribonacci Number](https://lcid.cc/1137) | [rust](rust/src/solutions/p1137.rs) | Easy |
| 1148. [Article Views I](https://lcid.cc/1148) | [sql](sql/solutions/p1148.sql) | Easy |
| 1280. [Students and Examinations](https://lcid.cc/1280) | [sql](sql/solutions/p1280.sql) | Easy |
| 1282. [Group the People Given the Group Size They Belong To](https://lcid.cc/1282) | [rust](rust/src/solutions/p1282.rs) | Medium |
| 1365. [How Many Numbers Are Smaller Than the Current Number](https://lcid.cc/1365) | [rust](rust/src/solutions/p1365.rs) | Easy |
| 1378. [Replace Employee ID With The Unique Identifier](https://lcid.cc/1378) | [sql](sql/solutions/p1378.sql) | Easy |
| 1395. [Count Number of Teams](https://lcid.cc/1395) | [rust](rust/src/solutions/p1395.rs) | Medium |
| 1442. [Count Triplets That Can Form Two Arrays of Equal XOR](https://lcid.cc/1442) | [rust](rust/src/solutions/p1442.rs) | Medium |
| 1448. [Count Good Nodes in Binary Tree](https://lcid.cc/1448) | [rust](rust/src/solutions/p1448.rs) | Medium |
| 1456. [Maximum Number of Vowels in a Substring of Given Length](https://lcid.cc/1456) | [rust](rust/src/solutions/p1456.rs) | Medium |
| 1460. [Make Two Arrays Equal by Reversing Subarrays](https://lcid.cc/1460) | [rust](rust/src/solutions/p1460.rs) | Easy |
| 1493. [Longest Subarray of 1's After Deleting One Element](https://lcid.cc/1493) | [rust](rust/src/solutions/p1493.rs) | Medium |
| 1508. [Range Sum of Sorted Subarray Sums](https://lcid.cc/1508) | [rust](rust/src/solutions/p1508.rs) | Medium |
| 1512. [Number of Good Pairs](https://lcid.cc/1512) | [rust](rust/src/solutions/p1512.rs) | Easy |
| 1550. [Three Consecutive Odds](https://lcid.cc/1550) | [rust](rust/src/solutions/p1550.rs) | Easy |
| 1581. [Customer Who Visited but Did Not Make Any Transactions](https://lcid.cc/1581) | [sql](sql/solutions/p1581.sql) | Easy |
| 1630. [Arithmetic Subarrays](https://lcid.cc/1630) | [rust](rust/src/solutions/p1630.rs) | Medium |
| 1636. [Sort Array by Increasing Frequency](https://lcid.cc/1636) | [rust](rust/src/solutions/p1636.rs) | Easy |
| 1657. [Determine if Two Strings Are Close](https://lcid.cc/1657) | [rust](rust/src/solutions/p1657.rs) | Medium |
| 1661. [Average Time of Process per Machine](https://lcid.cc/1661) | [sql](sql/solutions/p1661.sql) | Easy |
| 1679. [Max Number of K-Sum Pairs](https://lcid.cc/1679) | [rust](rust/src/solutions/p1679.rs) | Medium |
| 1683. [Invalid Tweets](https://lcid.cc/1683) | [sql](sql/solutions/p1683.sql) | Easy |
| 1689. [Partitioning Into Minimum Number Of Deci-Binary Numbers](https://lcid.cc/1689) | [rust](rust/src/solutions/p1689.rs) | Medium |
| 1757. [Recyclable and Low Fat Products](https://lcid.cc/1757) | [sql](sql/solutions/p1757.sql) | Easy |
| 1768. [Merge Strings Alternately](https://lcid.cc/1768) | [python](python/solutions/p1768.py) | Easy |
| 2095. [Delete the Middle Node of a Linked List](https://lcid.cc/2095) | [rust](rust/src/solutions/p2095.rs) | Medium |
| 2130. [Maximum Twin Sum of a Linked List](https://lcid.cc/2130) | [rust](rust/src/solutions/p2130.rs) | Medium |
| 2181. [Merge Nodes in Between Zeros](https://lcid.cc/2181) | [rust](rust/src/solutions/p2181.rs) | Medium |
| 2239. [Find Closest Number to Zero](https://lcid.cc/2239) | [python](python/solutions/p2239.py) | Easy |
| 2352. [Equal Row and Column Pairs](https://lcid.cc/2352) | [rust](rust/src/solutions/p2352.rs) | Medium |
| 2390. [Removing Stars From a String](https://lcid.cc/2390) | [rust](rust/src/solutions/p2390.rs) | Medium |
| 2418. [Sort the People](https://lcid.cc/2418) | [rust](rust/src/solutions/p2418.rs) | Easy |
| 2610. [Convert an Array Into a 2D Array With Conditions](https://lcid.cc/2610) | [rust](rust/src/solutions/p2610.rs) | Medium |
| 2678. [Number of Senior Citizens](https://lcid.cc/2678) | [rust](rust/src/solutions/p2678.rs) | Easy |
| 2769. [Find the Maximum Achievable Number](https://lcid.cc/2769) | [rust](rust/src/solutions/p2769.rs) | Easy |
| 2807. [Insert Greatest Common Divisors in Linked List](https://lcid.cc/2807) | [rust](rust/src/solutions/p2807.rs) | Medium |
| 3110. [Score of a String](https://lcid.cc/3110) | [rust](rust/src/solutions/p3110.rs) | Easy |
| 3146. [Permutation Difference between Two Strings](https://lcid.cc/3146) | [rust](rust/src/solutions/p3146.rs) | Easy |
| 3206. [Alternating Groups I](https://lcid.cc/3206) | [rust](rust/src/solutions/p3206.rs) | Easy |
| 3208. [Alternating Groups II](https://lcid.cc/3208) | [rust](rust/src/solutions/p3208.rs) | Medium |
<!-- table end -->
