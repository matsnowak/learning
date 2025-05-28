class Solution(object):
    def numRescueBoats(self, people, limit):
        """
        :type people: List[int]
        :type limit: int
        :rtype: int
        """
        people.sort()
        l, r = 0 , len(people)-1
        boat = 0
        while l <= r:
            if people[l] + people[r] <= limit:
                l += 1
            boat += 1
            r -= 1
        return boat
        
