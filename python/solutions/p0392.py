class Solution:
    def isSubsequence(self, s: str, t: str) -> bool:
        if not s:
            return True
        i, end = 0, len(s)
        for c in t:
            if s[i] == c:
                i += 1
            if i == end:
                return True
        return False
