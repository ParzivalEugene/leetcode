# Solutions for leetcode problems

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
                rustfmt $(pwd)/${line:3}
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
|9. [Palindrome Number](https://lcid.cc/9) | [Solution](../rust/src/solutions/p0009.rs) | Easy |
|11. [Container With Most Water](https://lcid.cc/11) | [Solution](../rust/src/solutions/p0011.rs) | Medium |
|13. [Roman to Integer](https://lcid.cc/13) | [Solution](../rust/src/solutions/p0013.rs) | Easy |
|21. [Merge Two Sorted Lists](https://lcid.cc/21) | [Solution](../rust/src/solutions/p0021.rs) | Easy |
|26. [Remove Duplicates from Sorted Array](https://lcid.cc/26) | [Solution](../rust/src/solutions/p0026.rs) | Easy |
|27. [Remove Element](https://lcid.cc/27) | [Solution](../rust/src/solutions/p0027.rs) | Easy |
|55. [Jump Game](https://lcid.cc/55) | [Solution](../rust/src/solutions/p0055.rs) | Medium |
|58. [Length of Last Word](https://lcid.cc/58) | [Solution](../rust/src/solutions/p0058.rs) | Easy |
|66. [Plus One](https://lcid.cc/66) | [Solution](../rust/src/solutions/p0066.rs) | Easy |
|80. [Remove Duplicates from Sorted Array II](https://lcid.cc/80) | [Solution](../rust/src/solutions/p0080.rs) | Medium |
|88. [Merge Sorted Array](https://lcid.cc/88) | [Solution](../rust/src/solutions/p0088.rs) | Easy |
|121. [Best Time to Buy and Sell Stock](https://lcid.cc/121) | [Solution](../rust/src/solutions/p0121.rs) | Easy |
|122. [Best Time to Buy and Sell Stock II](https://lcid.cc/122) | [Solution](../rust/src/solutions/p0122.rs) | Medium |
|125. [Valid Palindrome](https://lcid.cc/125) | [Solution](../rust/src/solutions/p0125.rs) | Easy |
|167. [Two Sum II - Input Array Is Sorted](https://lcid.cc/167) | [Solution](../rust/src/solutions/p0167.rs) | Medium |
|169. [Majority Element](https://lcid.cc/169) | [Solution](../rust/src/solutions/p0169.rs) | Easy |
|172. [Factorial Trailing Zeroes](https://lcid.cc/172) | [Solution](../rust/src/solutions/p0172.rs) | Medium |
|189. [Rotate Array](https://lcid.cc/189) | [Solution](../rust/src/solutions/p0189.rs) | Medium |
|205. [Isomorphic Strings](https://lcid.cc/205) | [Solution](../rust/src/solutions/p0205.rs) | Easy |
|383. [Ransom Note](https://lcid.cc/383) | [Solution](../rust/src/solutions/p0383.rs) | Easy |
|392. [Is Subsequence](https://lcid.cc/392) | [Solution](../rust/src/solutions/p0392.rs) | Easy |
|1550. [Three Consecutive Odds](https://lcid.cc/1550) | [Solution](../rust/src/solutions/p1550.rs) | Easy |
<!-- table end -->

