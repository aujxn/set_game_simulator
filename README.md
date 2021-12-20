# Set Simulator

## Purpose
The goal of this software is to investigate emergent combinatorial properties of Set (the
card game). The questions of interest are:
- How does the probability of encountering setless hands change as the game progresses?
- How does the number of sets in a hand change as the game progresses?
- Why do these probabilities behave this way?

To answer these questions I have implemented a Monte Carlo simulation that plays the card
game. In 2019 I originally implemented this as a way to learn the Rust language but recently
(12/2021) refactored and simplified the code.

The recent inspiration to revisit this problem came from two sources. First, I realized I
should be looking at the probabilities of 'ascending' versus 'descending' hands (see definitions).
Second, I recently gained access to the cluster computing environment through my university.
The data I previously collected was extremely noisy when looking at 18 and 21 card hands
because the probabilities related to these events are so small. I decided it would be fun to
run this on the cluster for a few weeks and see if I could get enough data points to clean
things up.

## Inspiration
Inspired by analysis by [Peter Norvig](https://norvig.com/SET.html) and
[Don Knuth](https://cs.stanford.edu/~knuth/programs/setset-all.w)
as well as conversations with Neil Babson.

## Definitions and Rules of Set

### Rules
Knowing how to play set is important to understanding this analysis. Set is a card game with
81 cards. Each card has 4 attributes: number, color, shading, and shape. Each attribute has 3
values. Number can be 1/2/3, color can be red/blue/green, shading can be solid/striped/hollow,
and shape can be oval/diamond/squiggle. Each combination of attributes appears once in the deck
making 3^4 = 81 cards. The goal of the game is to find 'sets'. A set is a group of 3 cards 
where each attribute is the same value or the three different values. Below is an example of some
sets. The game can be played single player or multiplayer. To play:
- shuffle the 81 card deck
- deal 12 cards face up
- search for a set
   - if you found a set, remove the set from the face up cards
   - if there is no set, deal 3 more cards and go back to searching for a set
      - 21 cards are required to guarantee there is a set but getting to this point is extremely rare
- after removing a set
   - if there are 9 cards in play then deal 3 more cards to make 12
   - otherwise go back to searching for sets

These steps are continued until you run out of cards in the deck and there are no sets remaining
in the face up cards. In multiplayer, whoever found the most sets is the winner.

### Definitions
- hand: I often refer to the face up (in play) cards as the hand.
- deck: Face down cards that haven't been played.
- setless hand: A hand that contains 0 sets. Excluding the last hand of the game only 12, 15,
and 18 card hands can be setless. A 21 card hand has to have a set.
- ascending hand: If the previously played hand had less cards, or if cards were just dealt
into the hand, I call it an ascending hand.
- descending hand: If the previously played hand had more cards, or if no cards were dealt to
create the current hand, I call it a descending hand.

## Hypothesis
The raw probability of a given hand containing no sets is quite low.
Randomly selecting cards from the 81 card deck, you get the following probabilities of
the hand containing sets:
- 12 cards is .9677
- 15 cards is .9996
- 18 cards is .99999999

In actual games, it feels like 12 and 15 card hands have no sets much more often than this. My
original hypothesis was that this is not a result of some sort of cognitive bias but that the
probability of setless hands increases as the game is played. The reasoning for this is because
you are removing sets as the game is played and replacing them with random cards. I proposed that
the probability of setless hands would increase for all possible hand sizes as the game progresses.
Neil Babson, a master of the game, posited that 12 card hands would have increasing probability
of no sets but 15 and 18 card hands would be consistent. This is loosely based on the argument
that every time you encounter 15 and 18 card hand there recently was a setless hand which results
in a sort of combinatorial reset. This assumes that the 'quality' of the overall deck doesn't
change as sets are removed.

Other mathematician's analysis has shown that the probability of getting setless 12 card hands
in game is higher than randomly selecting cards, supporting the first part of our hypothesis.
(see inspiration section for their analysis)

## Testing the Hypothesis
To test our hypotheses I implemented a Monte Carlo Simulation (MCS) of the game. Every time the
simulation gets to the 'search for sets' step in the game algorithm some data is recorded:
- how many times have cards been dealt? (or number of cards left in the deck)
- how many sets are found?
- how many cards are in the current hand?
- is the hand ascending or descending?
After recording the data the simulation chooses a random set to remove from the hand and plays
the game to completion.

After playing the game many times we can look at how the probability of setless hands as well
as the number of sets found changes for each hand type (size and asc/desc) as the game progresses.

## Results and Analysis
### 12 card hands
### 15 card hands
### 18 card hands
### 21 card hands

## Creating the Interactive Graphs
In the repo I have included the data from running the simulation for a long time on the cluster.
I used the python library ploty to generate the graphs, which are interactive. To create these
graphs from the data:

Create a python virtual environment and install the dependencies:
```bash
python3 -m venv venv
source ./venv/bin/activate
pip install --upgrade pip
pip install -r requirements.txt
```

Make the plots:
```bash
python3 graph.py
```
Navigate to localhost:8050 in your browser to see the plots.

## Running the Simulation
If you want to delete all the data and start over remove all the data files:
```bash
rm data/* data.csv
```

For usage instructions:
```bash
cargo run --release -- --help
```
Example, to run the simulation for 10 minutes using 4 threads:
```bash
cargo run --release -- run -- --minutes 10 --threads 4
```

If you're on a computing cluster that uses SLURM for work scheduling there is a script I used
that can be modified for your environment:
```bash
sbatch slurm.sh
```

After running the simulation the data is saved in the `data` directory. Each run creates a new
file with the computer hostname and the timestamp of the run as the filename. If you want to
combine all of the data files into one there is a command to do so:
```bash
cargo run --release -- consolidate
```
Which will save the combined data in a file called `data.csv`. This is the file that the python
script looks for to create the graphs.

## Licence
[MIT](https://choosealicense.com/licenses/mit)
