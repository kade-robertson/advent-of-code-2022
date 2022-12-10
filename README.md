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
 - Part 1: censored (took 23.05µs)
 - Part 2: censored (took 24.48µs)
Day 2: Rock Paper Scissors
 - Part 1: censored (took 40.69µs)
 - Part 2: censored (took 55.75µs)
Day 3: Rucksack Reorganization
 - Part 1: censored (took 46.03µs)
 - Part 2: censored (took 39.32µs)
Day 4: Camp Cleanup
 - Part 1: censored (took 40.86µs)
 - Part 2: censored (took 40.85µs)
Day 5: Supply Stacks
 - Part 1: censored (took 122.01µs)
 - Part 2: censored (took 112.86µs)
Day 6: Tuning Trouble
 - Part 1: censored (took 8.50µs)
 - Part 2: censored (took 22.07µs)
Day 7: No Space Left On Device
 - Part 1: censored (took 59.76µs)
 - Part 2: censored (took 56.16µs)
Day 8: Treetop Tree House
 - Part 1: censored (took 273.86µs)
 - Part 2: censored (took 223.18µs)
Took a total of 1.19ms
```

Averaged over 10 runs (using `--bench` option):

```
~ Advent of Code 2022 ~
Day 1: Calorie Counting
 - Part 1: censored (took 18.36µs)
 - Part 2: censored (took 21.47µs)
Day 2: Rock Paper Scissors
 - Part 1: censored (took 36.78µs)
 - Part 2: censored (took 43.52µs)
Day 3: Rucksack Reorganization
 - Part 1: censored (took 25.29µs)
 - Part 2: censored (took 17.05µs)
Day 4: Camp Cleanup
 - Part 1: censored (took 36.31µs)
 - Part 2: censored (took 36.00µs)
Day 5: Supply Stacks
 - Part 1: censored (took 109.43µs)
 - Part 2: censored (took 95.70µs)
Day 6: Tuning Trouble
 - Part 1: censored (took 6.79µs)
 - Part 2: censored (took 21.86µs)
Day 7: No Space Left On Device
 - Part 1: censored (took 57.55µs)
 - Part 2: censored (took 48.39µs)
Day 8: Treetop Tree House
 - Part 1: censored (took 257.41µs)
 - Part 2: censored (took 208.09µs)
Took a total of 1.04ms
```
