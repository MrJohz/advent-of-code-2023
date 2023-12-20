# Advent of Code 2023

> **Goal**: Speed of execution, specifically, that all results are computed in less than 1ms. That means each day has 41.67 microseconds to play with. Timings are done on a Macbook Pro M2 machine, using Criterion.

## Results

| Day | Part 1    | Part 2    | Total     |
| --- | --------- | --------- | --------- |
| 1   | 9.5885 µs | 21.704 µs | 31.293 µs |
| 2   | 3.3542 µs | 5.6706 µs | 9.0248 µs |
| 3   | 16.477 µs | 7.5925 µs | 24.070 µs |
| 4   | 14.293 µs | 14.737 µs | 29.030 µs |
| 5   | 9.5772 µs | 27.905 µs | 37.482 µs |
| 6   | 0.0591 µs | 0.0394 µs | 0.0985 µs |

## Notes

- [Simd](https://doc.rust-lang.org/std/simd/index.html) helps a lot, if you can find a good place for it. See also [memchr](https://docs.rs/memchr/latest/memchr/).
- [ArrayVec](https://docs.rs/arrayvec/latest/arrayvec/) is useful for small arrays where we can avoid heap allocation.
- Day 6 is a complete freebie! 😅

## TODOs

- `find_byte` for day 2 produces odd errors with `memchr` when trying to find a specific byte. Explore?
- Day 5: It may be possible to fuse ranges together when their ends are equal. (e.g. if 10..50 and 50..100 are both present in the list of ranges, then they can be fused to create 10..100) This would reduce the amount of data that needs to be processed, but would require sorting the arrays, or keeping them in order while updating them.
- Day 8: ???
- Day 9: Complexity optimisation: currently, we build a stack containing the difference between n and n', the difference between (n - n') and (n' and n''), etc. However, the end of this stack is always zero once a stable set of differences has been found. We could therefore stop building the stack once we reach zero, and then use the stack to calculate the final value. However, there are situations where a given layer in the stack may appear to be zero, but becomes nonzero later. (e.g. third example for day9: the 3rd layer goes (0, 2, 4, 6) - the first zero here does not indicate that we have a stable set of differences.)
