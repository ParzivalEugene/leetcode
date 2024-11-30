class Solution:
    def longestCommonPrefix(self, strs: list[str]) -> str:
        prefix = ""
        for i in range(len(min(strs))):
            if len(set(word[i] for word in strs)) == 1:
                prefix += strs[0][i]
            else:
                return prefix
        return prefix
