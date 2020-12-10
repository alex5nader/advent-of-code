# Day 7 Thoughts

Nice problem today! The input format looked much scarier to parse than it turned
out to be. I did actually use strings this time, since `&[u8]` can't exactly be
trimmed or split.

Parts A and B had you traverse the graph in opposite ways, which was super
interesting. I know there should be a DP solution to part B, but I decided
to do it recursively both because there's no time constraint and because I'm
more familiar with recursion.
