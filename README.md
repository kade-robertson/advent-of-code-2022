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
 - Part 1: censored (took 23.42µs)
 - Part 2: censored (took 23.74µs)
Day 2: Rock Paper Scissors
 - Part 1: censored (took 39.70µs)
 - Part 2: censored (took 47.01µs)
Day 3: Rucksack Reorganization
 - Part 1: censored (took 44.72µs)
 - Part 2: censored (took 38.63µs)
Day 4: Camp Cleanup
 - Part 1: censored (took 41.57µs)
 - Part 2: censored (took 41.25µs)
Day 5: Supply Stacks
 - Part 1: censored (took 124.11µs)
 - Part 2: censored (took 116.11µs)
Day 6: Tuning Trouble
 - Part 1: censored (took 8.97µs)
 - Part 2: censored (took 22.05µs)
Day 7: No Space Left On Device
 - Part 1: censored (took 60.36µs)
 - Part 2: censored (took 55.26µs)
Day 8: Treetop Tree House
 - Part 1: censored (took 271.98µs)
 - Part 2: censored (took 223.53µs)
Day 9: Rope Bridge
 - Part 1: censored (took 359.42µs)
 - Part 2: censored (took 598.99µs)
Took a total of 2.14ms
```

Averaged over 10 runs (using `--bench` option):

```
~ Advent of Code 2022 ~
Day 1: Calorie Counting
 - Part 1: censored (took 18.08µs)
 - Part 2: censored (took 21.22µs)
Day 2: Rock Paper Scissors
 - Part 1: censored (took 36.91µs)
 - Part 2: censored (took 42.25µs)
Day 3: Rucksack Reorganization
 - Part 1: censored (took 25.71µs)
 - Part 2: censored (took 17.43µs)
Day 4: Camp Cleanup
 - Part 1: censored (took 36.93µs)
 - Part 2: censored (took 36.41µs)
Day 5: Supply Stacks
 - Part 1: censored (took 104.20µs)
 - Part 2: censored (took 97.09µs)
Day 6: Tuning Trouble
 - Part 1: censored (took 6.76µs)
 - Part 2: censored (took 21.87µs)
Day 7: No Space Left On Device
 - Part 1: censored (took 50.86µs)
 - Part 2: censored (took 45.95µs)
Day 8: Treetop Tree House
 - Part 1: censored (took 254.85µs)
 - Part 2: censored (took 206.78µs)
Day 9: Rope Bridge
 - Part 1: censored (took 328.96µs)
 - Part 2: censored (took 587.48µs)
Took a total of 1.94ms
```
