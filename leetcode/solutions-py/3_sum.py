class Solution(object):
    def threeSum(self, nums):
        """
        :type nums: List[int]
        :rtype: List[List[int]]
        """
        nums.sort()
        res = []
        for i in range(len(nums)):
            # in sorted array, first number should be lower that 0 if we want to have total 0
            # skip the same to remove duplicates
            if nums[i] > 0 or i > 0 and nums[i] == nums[i-1]: continue
            l, r = i+1, len(nums)-1
            while l < r:
                total = nums[i] + nums[l] + nums[r]
                if total == 0:
                    res.append([nums[i], nums[l], nums[r]])
                    l, r = l+1, r-1

                    # skip the same to remove duplicates
                    while l < len(nums) -1 and nums[l] == nums[l-1]:
                        l += 1
                # in sorted, to make it smaller take smaller number from right, to make it bigger take bigger number from left 
                elif total > 0:
                    r -= 1
                else:
                    l += 1
        return res
