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
 - Part 1: censored (took 23.98µs)
 - Part 2: censored (took 24.68µs)
Day 2: Rock Paper Scissors
 - Part 1: censored (took 41.41µs)
 - Part 2: censored (took 47.88µs)
Day 3: Rucksack Reorganization
 - Part 1: censored (took 44.39µs)
 - Part 2: censored (took 38.14µs)
Day 4: Camp Cleanup
 - Part 1: censored (took 40.95µs)
 - Part 2: censored (took 41.29µs)
Day 5: Supply Stacks
 - Part 1: censored (took 125.11µs)
 - Part 2: censored (took 117.61µs)
Day 6: Tuning Trouble
 - Part 1: censored (took 9.21µs)
 - Part 2: censored (took 22.07µs)
Day 7: No Space Left On Device
 - Part 1: censored (took 60.00µs)
 - Part 2: censored (took 54.88µs)
Day 8: Treetop Tree House
 - Part 1: censored (took 272.62µs)
 - Part 2: censored (took 223.68µs)
Day 9: Rope Bridge
 - Part 1: censored (took 351.08µs)
 - Part 2: censored (took 498.36µs)
Took a total of 2.04ms
```

Averaged over 10 runs (using `--bench` option):

```
~ Advent of Code 2022 ~
Day 1: Calorie Counting
 - Part 1: censored (took 19.77µs)
 - Part 2: censored (took 22.23µs)
Day 2: Rock Paper Scissors
 - Part 1: censored (took 38.66µs)
 - Part 2: censored (took 43.44µs)
Day 3: Rucksack Reorganization
 - Part 1: censored (took 28.50µs)
 - Part 2: censored (took 31.28µs)
Day 4: Camp Cleanup
 - Part 1: censored (took 40.55µs)
 - Part 2: censored (took 38.45µs)
Day 5: Supply Stacks
 - Part 1: censored (took 107.72µs)
 - Part 2: censored (took 102.30µs)
Day 6: Tuning Trouble
 - Part 1: censored (took 9.00µs)
 - Part 2: censored (took 23.19µs)
Day 7: No Space Left On Device
 - Part 1: censored (took 67.35µs)
 - Part 2: censored (took 50.14µs)
Day 8: Treetop Tree House
 - Part 1: censored (took 268.04µs)
 - Part 2: censored (took 209.02µs)
Day 9: Rope Bridge
 - Part 1: censored (took 348.73µs)
 - Part 2: censored (took 486.42µs)
Took a total of 1.93ms
```
