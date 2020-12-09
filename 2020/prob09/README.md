# Day 9 Thoughts

This problem is a more complex version of day 1's problem. Rather than just
checking a single number, you have to verify that all numbers are sums of
two other numbers, and find the single number that isn't. Then, find a region
that sums to that number.

While solving part A, I ran into an underflow panic. My quick and dirty solution
was to just use signed integers. While this work, the underflow actually
indicated a possible optimization. If the number being checked is already bigger
than the target, adding something to it can't decrease it, so don't even bother
checking.

I just brute forced part B, and it took around 5 seconds to run. After submitting
my answer, I looked up the problem and it turns out you can add a single check
to the brute force that speeds up the algorithm dramatically.

I wanted to keep my original implementation, so these two optimizations are
implemented in [better.rs](better.rs).
