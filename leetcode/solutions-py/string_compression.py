class Solution(object):
    def compress(self, chars):
        """
        :type chars: List[str]
        :rtype: int
        """
        def compress_char(write, curr, counter):
            chars[write] = curr
            write += 1
            if counter == 1:      # does not append length
                return write
            length = str(counter) # convert length to string
            for c in length:
                chars[write] = c
                write += 1
            return write
        write, counter, curr = 0, 1, chars[0]
        for read in range(1, len(chars)):
            if chars[read] == curr:
                counter += 1
            else:
                write = compress_char(write, curr, counter)
                counter = 1
            curr = chars[read]
        write = compress_char(write, curr, counter)
        return write

