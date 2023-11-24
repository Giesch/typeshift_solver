# Typeshift Solver

This is a solver for the word game [Typeshift](http://www.playtypeshift.com), which you can play at [Puzzmo](https://www.puzzmo.com).

It's mostly an exercise in chasing microbenchmarks. I've gotten it down to 100-500 microseconds depending on the size of the puzzle. There's still some potential allocations that could be removed, and the opportunity to use plain ascii instead of utf8.
