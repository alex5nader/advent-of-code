# Day 3 Thoughts

Another fairly simple problem, though I over-thought it a lot when I first
saw it. I thought it was going to be a graph/pathfinding problem, so I was
very relieved when I kept reading and found it wasn't. I also didn't realize
the input was repeating at first; I thought it just had to be transposed for
some reason.

The input is a map that repeats infinitely towards +X. Given a dx and dy, a
"path" is all the points given from these equations:

P<sub>0</sub> = (0, 0)

P<sub>i</sub> = P<sub>i-1</sub> + (dx, dy)

The goal is to find the number of `#`s inside different paths, and multiply
those counts together.
