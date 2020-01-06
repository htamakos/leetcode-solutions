from typing import List

class Solution:
    # Worst Case Performance: O(n^2)
    # Space Complexity: O(1)
    def twoSum1(self, nums: List[int], target: int) -> List[int]:
        for i, v1 in enumerate(nums):
            for j, v2 in enumerate(nums[i+1:]):
                if (v1 + v2) == target:
                    return [i, (i+j+1)]

    # Worst Case Performance: O(n)
    # Space Complexity: O(n)
    def twoSum2(self, nums: List[int], target: int) -> List[int]:
        h = { v: i for i, v in enumerate(nums)}

        i = 0
        for v in nums:
            j = h.get(target - v)

            if j is not None and j != i:
                return [i, j]
            i += 1

