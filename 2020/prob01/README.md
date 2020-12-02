# Day 1 Thoughts

Interesting problem. Brute force would be O(N<sup>2</sup>), but for part A there's a O(N) solution:

Build hash set of all input values. To find the factors, find element x such that 2020-x is in the hash set. Target value is x * (2020-x).

This somewhat applies to part B. Complete brute force would be O(N<sup>3</sup>). A similar solution to part A's can cut this down to O(N<sup>2</sup>):

Build hash set of all input values. To find the factors, find pair x,y such that (2020-x-y) is in the hash set. Target value is x * y * (2020-x-y).

This pattern can be applied to finding any F factors in O(N<sup>F-1</sup>) time and O(N) memory.
