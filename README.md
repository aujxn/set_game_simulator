Author: Austen Nelson

This program simulates set games to count how often hands contain no sets.

The probability of a given hand containing no sets is quite low.

For a 12 card hand it is a .9677 probability of containing a set.
For a 15 card hand it is a .9996 probability of containing a set.
For an 18 card hand it is a .99999999 probability of containing a set.

In actual games it seems like 12 and 15 card hands have no sets much more often than this.
Is this a cognitive bias or does the probability of a hand containing a set decrease as sets are removed?
Turns out the second seems to be true.

According to this simulator, 18 card hands that are encountered in games actually have no sets about .1% of the time.
Thats pretty cool. .1% is much higher than the expected 1.42 * 10^-6 % probability.

Here is the output of the program run on 10 million games of set:

setless 12's 14704847
set 12's 215619108
proportion of 12's 0.068198

setless 15's 368864
set 15's 14011627
proportion of 15's 0.026326

setless 18's 401
set 18's 352049
proportion of 18's 0.001139

set 21's 369


A 21 card hand only happens a handful of times every 100,000 games but it is possible.

This confirms the results that Peter Norvig found, but this program can run much larger tests in reasonable time.

Inspired by analysis by Peter Norvig (norvig.com/SET.html) and Don Knuth (cs.stanford.edu/~knuth/programs/setset-all.w)
as well as conversations with Neil Babson.

This code is available under the MIT license.
