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
 - Part 1: censored (took 44.06µs)
 - Part 2: censored (took 47.51µs)
Day 2: Rock Paper Scissors
 - Part 1: censored (took 40.40µs)
 - Part 2: censored (took 51.13µs)
Day 3: Rucksack Reorganization
 - Part 1: censored (took 43.23µs)
 - Part 2: censored (took 39.47µs)
Day 4: Camp Cleanup
 - Part 1: censored (took 55.55µs)
 - Part 2: censored (took 41.89µs)
Day 5: Supply Stacks
 - Part 1: censored (took 125.25µs)
 - Part 2: censored (took 103.81µs)
Day 6: Tuning Trouble
 - Part 1: censored (took 8.68µs)
 - Part 2: censored (took 22.95µs)
Took a total of 623.97µs
```

Averaged over 10 runs (using `--bench` option):

```
~ Advent of Code 2022 ~
Day 1: Calorie Counting
 - Part 1: censored (took 35.56µs)
 - Part 2: censored (took 36.70µs)
Day 2: Rock Paper Scissors
 - Part 1: censored (took 37.40µs)
 - Part 2: censored (took 46.02µs)
Day 3: Rucksack Reorganization
 - Part 1: censored (took 25.02µs)
 - Part 2: censored (took 16.95µs)
Day 4: Camp Cleanup
 - Part 1: censored (took 36.86µs)
 - Part 2: censored (took 36.52µs)
Day 5: Supply Stacks
 - Part 1: censored (took 113.92µs)
 - Part 2: censored (took 103.77µs)
Day 6: Tuning Trouble
 - Part 1: censored (took 6.77µs)
 - Part 2: censored (took 21.89µs)
Took a total of 517.37µs
```
