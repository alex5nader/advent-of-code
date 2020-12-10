# Day 5 Thoughts

Finals kept me from doing this one when it was released, so I worked on it the next day.

The problem describes the input format in terms of binary search, so I implemented
my solution using that algorithm. I later found out that the input actually just
directly encodes the ID in binary with 0 and 1 swapped for F, B, R, and L, which I
thought was fairly well disguised.

I also realized that rust's UTF-8 validation could be ignored by simply using
the bytes of the input directly (since every problem so far's input has been
ascii anyway).
