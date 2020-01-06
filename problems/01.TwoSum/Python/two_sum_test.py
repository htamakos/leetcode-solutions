import unittest
from two_sum import Solution

class TwoSumTest(unittest.TestCase):
    def test_two_sum1(self):
        nums = [1, 2, 7, 9]
        target = 9
        ans = [1,2]

        self.assertEqual(Solution().twoSum1(nums, target), ans)
        self.assertEqual(Solution().twoSum2(nums, target), ans)

    def test_two_sum2(self):
        nums = [3,3]
        target = 6
        ans = [0,1]

        self.assertEqual(Solution().twoSum1(nums, target), ans)
        self.assertEqual(Solution().twoSum2(nums, target), ans)


if __name__ == "__main__":
    unittest.main()

