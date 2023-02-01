# Fast Random Integer Generation in a Range

This is a repository for the code I've created as part of a research project.

The intent is to create an efficient algorithm for generating random integers in a range that isn't a power of 2.

# Methodology

Imagine a continuous number line from 0 to n (the upper bound supplied to the algorithm)

For instance: n = 6

0 --- 1 --- 2 --- 3 --- 4 --- 5 --- 6

This algorithm seeks to pick a uniform random point on this line (including 0, excluding n),
such that the number on or to the left of the chosen point is the resulting integer.

To do so, 64 random bits are generated. If we imagine a radix point at the start of these 64
bits, the result is essentially some number in the range [0, 1). It will represent where exactly
on the number line this point will fall.

For simplicity's sake, suppose the number generated is exactly 111, followed by 61 zeroes.
It therefore falls 7/8 of the way on the line, so around here.

                          |    
                          V
0 --- 1 --- 2 --- 3 --- 4 --- 5 --- 6

Therefore, the algorithm would choose 5 in this case.

However, if the input upper bound is not a power of 2, there is a possibility that the point lands *very* close
to a number on its right. This means that, if we were to request more bits to choose this point, there is a chance
these new bits could push the point over to the right of the number, thus changing the output.

The algorithm represents the "points" as `m = n * the 64 random bits`, so this case happens if the last 64 bits of m are
very close to overflowing.

Consider an extreme example, with the same 0-6 number line, but suppose we only generated 3 bits.

The last bit therefore represents 1/8 * 6 = 0.75 of value

000 -> 0, which cannot increase to 1 anymore (see later), done, result = 0

001 -> 0.75

010 -> 1.5

011 -> 2.25, which cannot ever reach 3 even with infinite future 1s, done, result = 5

100 -> 3, which cannot increase to 4 anymore (see later), done, result = 0

101 -> 3.75

110 -> 4.5

111 -> 5.25, which cannot ever reach 6 even with infinite future 1s, done, result = 5

The 4 cases within less than 0.75 of incrementing I claim cannot yet terminate, while the other 4 I claim are done.

Consider 101: `m = 101 * 110 (six) = 11110`

The last three bits of m are `110`. The algorithm checks the 1s complement of this number, `001` 
