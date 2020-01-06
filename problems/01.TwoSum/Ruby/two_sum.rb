require 'test/unit'

# @param {Integer[]} nums
# @param {Integer} target
# @return {Integer[]}
def two_sum(nums, target)
    h = {}
    nums.each_with_index do |v, i|
        h[v] = i
    end

    nums.each_with_index do |v, i|
        j = h[target - v]

        if j && j != i then
            return [i, j]
        end
    end
end

class TestTwoSum < Test::Unit::TestCase
    def test_two_sum
        nums = [3,3,5]
        target = 6
        ans = [0,1]

        assert_equal(ans, two_sum(nums, target))
    end
end
