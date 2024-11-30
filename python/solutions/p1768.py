class Solution:
    def mergeAlternately(self, word1_: str, word2_: str) -> str:
        word1, word2 = list(word1_), list(word2_)
        word = ""
        for i in range(min(len(word1), len(word2))):
            word += word1.pop(0) + word2.pop(0)
        word += "".join(word1) + "".join(word2)
        return word
