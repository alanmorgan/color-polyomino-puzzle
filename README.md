# color-polyomino-puzzle

Bootstraps off my polyomino solver, rust-polyomino-solver.

The objective:

Find a set of pentominoes that are colored, such that when you form a
rectangle, the colors themselves make a complete set of pentominoes.

> polycolorpuzzle x y

Generates all x by y pentomino tilings and attempts to 3 or 4 color
them. Prints the index of each tiling that has a "good" coloring.
A good coloring is defined as one where each color appears equally
often. So a "good" 3-coloring will have 4 pentominoes of each color.

Having exactly 12 pentominoes is very convenient as it's divisible by
both 3 and 4. The definition of "good" coloring would need to be
adjusted slightly when searching for tiling solutions with hexominoes,
etc. 

We don't make any effort to nudge the colorer to produce a "good"
solution.

> polycolorpuzzle x y n

Shows the discovered coloring for solution `n`.

> polycolorpuzzle x y n m

The goal. We impose the coloring of solution `n` on solution `m` and
then generate all x by y tilings with those new pentominoes. For each
solution we check to see if the colors (not the pentominoes themselves)
form a complete set of pentominoes.

We print out the coloring for `n`, the imposed coloring for `m`, and then
all valid solutions using the `m` colored pentominoes. Ideally there will
be exactly one. 

(See solution.png for an example)

This has always been the case. I have yet to find any cases where a coloring
for `m` yields pentominoes that have two different valid solutions.

Note: The program immediately rejects `n` and `m` if they have the same
pentomino in the same place. That would give a mono-colored pentomino
for `m` and that's boring.

If anyone using this would like to print out a nice set of colored
pentominoes based on this code or can find a set of colored pentominos
that have two different valid tilings, I would love to hear about it.
