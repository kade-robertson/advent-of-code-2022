# advent-of-code-2022

To run:

1. Download from the Releases tab.

OR

2. [Install Rust](https://www.rust-lang.org/learn/get-started), clone this repo and compile.

   ```sh
   git clone https://github.com/kade-robertson/advent-of-code-2022.git
   cd advent-of-code-2022
   cargo build --release
   ```

   Output should be in `target/release`.

You can run with the `--censor` command line argument to avoid spoiling results, though AoC
usually provides different input for each user so the results here are not useful.

### Performance

Run on a Ryzen 7 5800X in Pop!\_OS, in release mode. Answers are censored.

```
~ Advent of Code 2022 ~
Day 1: Calorie Counting
 - Part 1: censored (took 45.45µs)
 - Part 2: censored (took 42.21µs)
Day 2: Rock Paper Scissors
 - Part 1: censored (took 39.93µs)
 - Part 2: censored (took 39.08µs)
Day 3: Rucksack Reorganization
 - Part 1: censored (took 43.23µs)
 - Part 2: censored (took 39.04µs)
Day 4: Camp Cleanup
 - Part 1: censored (took 40.46µs)
 - Part 2: censored (took 39.12µs)
Day 5: Supply Stacks
 - Part 1: censored (took 121.92µs)
 - Part 2: censored (took 108.35µs)
```

Averaged over 10 runs (using `--bench` option):

```
~ Advent of Code 2022 ~
Day 1: Calorie Counting
 - Part 1: censored (took 35.88µs)
 - Part 2: censored (took 32.56µs)
Day 2: Rock Paper Scissors
 - Part 1: censored (took 37.61µs)
 - Part 2: censored (took 36.01µs)
Day 3: Rucksack Reorganization
 - Part 1: censored (took 24.80µs)
 - Part 2: censored (took 17.35µs)
Day 4: Camp Cleanup
 - Part 1: censored (took 35.46µs)
 - Part 2: censored (took 34.12µs)
Day 5: Supply Stacks
 - Part 1: censored (took 105.76µs)
 - Part 2: censored (took 103.55µs)
Took a total of 463.10µs
```
