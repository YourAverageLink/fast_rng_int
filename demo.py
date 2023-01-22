# for demonstration purposes, the bit generator will be from the python random module
from random import getrandbits

def rand_below(n: int):
    """
    Returns a random positive integer less than n.

    This algorithm generates 64 bits at a time,
    multiplies by the provided maximum to get
    something close to uniform in [0, n).

    If the number is borderline (that is, if
    more bits were used, the integer portion of the number
    could be incremented), recursion is used to
    determine if the number should increment.
    """
    assert isinstance(n, int)
    assert n > 1
    # this will likely not work very well for very large maxima
    assert n < 0x8000000000000000
    # 64 random bits will serve as a number essentially in the range [0, 1)
    # It is then multiplied by the desired maximum to transform the range to [0, n)
    m = getrandbits(64) * n
    # If the last 64 bits of m are more than n away from rolling over, we can stop.
    # This is because, even if an infinite stream of 1s were to follow, it would not
    # increment the integer portion of the number (m >> 64).
    # However, if it is possible, use recursion to determine if the number should increment.
    if (-m & 0xFFFFFFFFFFFFFFFF) < n:
        # (-m & (2**64 - 1)) is the 2's complement of the last 64 bits of m
        # If this result is less than n, that means it is possible for the integer
        # portion of m to increment if we were to allow more bits to be supplied.
        # To ensure uniformity, more bits are needed.
        return (m + rand_below(n)) >> 64
    # Return the integer portion of m
    return m >> 64

