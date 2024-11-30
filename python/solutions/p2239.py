class Solution:
    def findClosestNumber(self, nums: list[int]) -> int:
        closest = 100001
        for num in nums:
            if abs(num) == abs(closest):
                closest = max(num, closest)
            elif abs(num) < abs(closest):
                closest = num
        return closest