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
 - Part 1: censored (took 46.19µs)
 - Part 2: censored (took 47.93µs)
Day 2: Rock Paper Scissors
 - Part 1: censored (took 38.46µs)
 - Part 2: censored (took 49.19µs)
Day 3: Rucksack Reorganization
 - Part 1: censored (took 43.91µs)
 - Part 2: censored (took 40.82µs)
Day 4: Camp Cleanup
 - Part 1: censored (took 41.91µs)
 - Part 2: censored (took 42.54µs)
Day 5: Supply Stacks
 - Part 1: censored (took 123.64µs)
 - Part 2: censored (took 99.53µs)
Day 6: Tuning Trouble
 - Part 1: censored (took 7.64µs)
 - Part 2: censored (took 22.08µs)
Day 7: No Space Left On Device
 - Part 1: censored (took 59.31µs)
 - Part 2: censored (took 55.97µs)
Took a total of 719.17µs
```

Averaged over 10 runs (using `--bench` option):

```
~ Advent of Code 2022 ~
Day 1: Calorie Counting
 - Part 1: censored (took 36.47µs)
 - Part 2: censored (took 39.59µs)
Day 2: Rock Paper Scissors
 - Part 1: censored (took 35.93µs)
 - Part 2: censored (took 44.46µs)
Day 3: Rucksack Reorganization
 - Part 1: censored (took 28.23µs)
 - Part 2: censored (took 18.61µs)
Day 4: Camp Cleanup
 - Part 1: censored (took 38.30µs)
 - Part 2: censored (took 37.23µs)
Day 5: Supply Stacks
 - Part 1: censored (took 108.94µs)
 - Part 2: censored (took 101.36µs)
Day 6: Tuning Trouble
 - Part 1: censored (took 6.70µs)
 - Part 2: censored (took 21.88µs)
Day 7: No Space Left On Device
 - Part 1: censored (took 52.98µs)
 - Part 2: censored (took 47.54µs)
Took a total of 618.22µs
```
