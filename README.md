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
 - Part 1: censored (took 23.36µs)
 - Part 2: censored (took 24.35µs)
Day 2: Rock Paper Scissors
 - Part 1: censored (took 41.70µs)
 - Part 2: censored (took 49.56µs)
Day 3: Rucksack Reorganization
 - Part 1: censored (took 43.65µs)
 - Part 2: censored (took 38.85µs)
Day 4: Camp Cleanup
 - Part 1: censored (took 41.04µs)
 - Part 2: censored (took 41.24µs)
Day 5: Supply Stacks
 - Part 1: censored (took 126.29µs)
 - Part 2: censored (took 118.48µs)
Day 6: Tuning Trouble
 - Part 1: censored (took 9.00µs)
 - Part 2: censored (took 22.10µs)
Day 7: No Space Left On Device
 - Part 1: censored (took 61.08µs)
 - Part 2: censored (took 58.53µs)
Day 8: Treetop Tree House
 - Part 1: censored (took 114.20µs)
 - Part 2: censored (took 225.86µs)
Day 9: Rope Bridge
 - Part 1: censored (took 341.98µs)
 - Part 2: censored (took 480.10µs)
Took a total of 1.86ms
```

Averaged over 10 runs (using `--bench` option):

```
~ Advent of Code 2022 ~
Day 1: Calorie Counting
 - Part 1: censored (took 18.72µs)
 - Part 2: censored (took 21.92µs)
Day 2: Rock Paper Scissors
 - Part 1: censored (took 38.05µs)
 - Part 2: censored (took 45.24µs)
Day 3: Rucksack Reorganization
 - Part 1: censored (took 26.55µs)
 - Part 2: censored (took 17.07µs)
Day 4: Camp Cleanup
 - Part 1: censored (took 36.49µs)
 - Part 2: censored (took 36.05µs)
Day 5: Supply Stacks
 - Part 1: censored (took 106.22µs)
 - Part 2: censored (took 98.11µs)
Day 6: Tuning Trouble
 - Part 1: censored (took 6.72µs)
 - Part 2: censored (took 21.86µs)
Day 7: No Space Left On Device
 - Part 1: censored (took 51.96µs)
 - Part 2: censored (took 47.07µs)
Day 8: Treetop Tree House
 - Part 1: censored (took 105.67µs)
 - Part 2: censored (took 206.58µs)
Day 9: Rope Bridge
 - Part 1: censored (took 327.18µs)
 - Part 2: censored (took 455.57µs)
Took a total of 1.67ms
```
