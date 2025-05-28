class Solution(object):
    def minSwaps(self, data):
        """
        :type data: List[int]
        :rtype: int
        """
        # count 1s, it will be fixed sliding window size
        count1 = data.count(1)

        # number of 1s in window
        total = 0
        for i in range(count1): total += data[i]

        # trick: number of missing 1s in window means number of swaps, no more checks needed
        swaps = count1-total

        # move windows by one and count
        for r in range(count1, len(data)):
            total += data[r]
            total -= data[r-count1]
            swaps = min(swaps, count1-total)
        return swaps
        
