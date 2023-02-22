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

The algorithm represents the "points" as `(lo, hi) = n * the 64 random bits`, so this case happens if the lo bits are close to overflowing.

Consider an extreme example, with the same 0-6 number line, but suppose we only generated 3 bits.

The last bit therefore represents 1/8 * 6 = 0.75 of value

000 -> 0, which cannot increase to 1 anymore (see later), done, result = 0

001 -> 0.75

010 -> 1.5

011 -> 2.25, which cannot ever reach 3 even with infinite future 1s, done, result = 2

100 -> 3, which cannot increase to 4 anymore (see later), done, result = 3

101 -> 3.75

110 -> 4.5

111 -> 5.25, which cannot ever reach 6 even with infinite future 1s, done, result = 5

The 4 cases within less than 0.75 of incrementing I claim cannot yet terminate, while the other 4 I claim are done.

Consider 101: `hi, lo = 101 * 110 (six) -> 11, 110`

The low bits are `110`. The algorithm checks this portion against the 2's complement of the upper bound, `010` 

Suppose the low bits are less than or equal to the 2's complement of n. What does that mean?
- If we were to get infinite 1s afterward, then multiply by n, this represents
- .111111111111111... * n, which *approaches* n, and thus, if the low bits are less than
- 2's complement of n, they will never overflow, and if they're equal, it will appraoch overflow
- but never reach it. Therefore, we are done because we know the high bits can never increment.
- Example: 111 * 110 -> 101 hi, 010 low, so low can only ever reach 111.111111111..., and high won't increment.


Since 110 > 010, we can conclude that, if we were to supply more bits, it's possible for low to overflow,
therefore incrementing the high bits. We need more bits to be sure!

Repeat the process. This time, though, we can take another early out.
- If our new set of high bits for sure increments or does not increment the old low bits, we know we're done
- We check this with a comparison with !lo.
- hi > !lo implies hi + lo overflows, therefore we can return result + 1
- hi < !lo implies hi + lo does not overflow (and never will). We can return result
- hi == lo implies uncertainty, though. hi + lo then equals 111... -> if we're also on the "borderline" with our "new numberline", we're not done yet
- - By this point, we can simply repeat the process, hence why the loop condition is lo > n.wrapping_neg()

# The Algorithm

Suppose we have access to a generator for a uniform 64 bit unsigned integer.

1. Generate the number (remember we're imagining a radix point at the start so it's in the range [0, 1))
2. Multiply and store into a variable for the lower 64 and higher 64 bits.
3. Remember that if low <= the 2's complement of the upper bound, we're for sure done and we can return the original high bits
- Thus, we will use a while loop with condition low > n.wrapping_neg(), and return outside the loop
4. Inside the loop, we'll remember the original high bits (so we have a return value) and generate 64 new random bits
5. Calculate the new high bits. From here, we can compare to the old low bits (which we will have not yet replaced with the new random number)
- To avoid branching, I decide to add (hi > !lo) as a u64 to result (so we return result + 1 if necessary)
6. Calculate the new low bits. (Again, to avoid branching, I multiply by (hi == lo), so it evaluates to 0 if we're certain our result is determined, so we exit the loop)
7. Continue the loop if necessary, and outside the loop, return the result.

This algorithm intends to be a divisionless alternative to Lemire's algorithm. It is inspired by Canon's proposed Swift algorithm, but by using a loop, it can ensure uniformity and avoid consuming an extra random word if it's not needed.