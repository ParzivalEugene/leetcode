class Solution:
    def format(self, start: int, end: int) -> str:
        if start == end:
            return str(start)
        return f"{start}->{end}"

    def summaryRanges(self, nums: list[int]) -> list[str]:
        if not nums:
            return []
        output = list()
        start, end = nums[0], nums[0]
        for n in nums[1:]:
            if n - 1 == end:
                end += 1
            else:
                output.append(self.format(start, end))
                start, end = n, n
        output.append(self.format(start, end))

        return output
