class Solution(object):
    def lengthOfLongestSubstringTwoDistinct(self, s):
        """
        :type s: str
        :rtype: int
        """
        last_occurrence = dict()
        max_len, l = 0, 0
        for r in range(len(s)):
            # count last occurence of character within window
            last_occurrence[s[r]] = r
            r += 1

            # if new charracter arrived, remove old one
            if len(last_occurrence) == 3:

                # instead of skipping one by one to new character just go directly,
                # minimum value is the caracter you are looking for
                l = min(last_occurrence.values())+1

                # remove character from window
                del last_occurrence[s[l-1]]
            max_len = max(max_len, r - l)
        return max_len
