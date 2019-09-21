# Set Simulator

This program simulates Set games to count how often hands contain no sets.

The raw probability of a given hand containing no sets is quite low.
Randomly selecting cards you get the following probability of containing sets:

12 cards is .9677

15 cards is .9996

18 cards is .99999999

In actual games 12 and 15 card hands have no sets much more often than this.
This program investigates how the probability of encountering a hand with no sets changes as the game progresses.

Other's analysis has shown that the probability of getting hands with no sets in game is higher than randomly selecting cards.
This leads to the intuitive hypothesis that as sets are removed from the hand and random cards replace them, the
"quality" of the hand and deck decreases. By quality I mean the likelyhood of a hand, regardless of size, containing a set
decreases as the game is played. When discussing this hypothesis with another math enthusiast they rejected it on the claim
that 12 card hands would decrease in quality but the following 15 card hand would have a consistent probability of containing a set.

It turns out we were both wrong. The probability is variable but in an interesting way. I don't currently have a convincing hypothesis why.

## Running the Simulation

Included in the python folder is the output of the simulation run on 1_000_000_000 games.

If you want to run this simulation yourself clone the repository.
```bash
git clone https://github.com/aujxn/set_game_simulator.git
cd set_game_simulator
```

At this step you have enough to generate the raw data. To run the program you must have rustc or cargo installed.
Make sure to include the release flag or it will take forever. The output data is exported to ./python/data.txt
```bash
cargo run --release
```

And if you would like to generate the plotly graphs, create and activate a python virtual environment and get the required libraries.
```bash
python3 -m venv venv
source ./venv/bin/activate
pip install -r requirements.txt
```

The python script must be run from the root of the project to find the data file. Navigate to localhost:8050 in your browser to see the plots.
```bash
python3 ./python/graph.py
```

## TODO
Add some more comments and a README for the data format
Add command line argument for selecting number of games
Have option for each run to accumulate data instead of wiping old data
Analyze total number of sets at each point in the game
Try removing random sets instead of the first set encountered


## Credit
Inspired by analysis by [Peter Norvig](https://norvig.com/SET.html) and [Don Knuth](https://cs.stanford.edu/~knuth/programs/setset-all.w)
as well as conversations with Neil Babson.

## Licence
[MIT](https://choosealicense.com/licenses/mit)
