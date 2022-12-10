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
 - Part 1: censored (took 22.76µs)
 - Part 2: censored (took 23.57µs)
Day 2: Rock Paper Scissors
 - Part 1: censored (took 41.45µs)
 - Part 2: censored (took 47.85µs)
Day 3: Rucksack Reorganization
 - Part 1: censored (took 44.58µs)
 - Part 2: censored (took 38.23µs)
Day 4: Camp Cleanup
 - Part 1: censored (took 41.27µs)
 - Part 2: censored (took 41.67µs)
Day 5: Supply Stacks
 - Part 1: censored (took 127.06µs)
 - Part 2: censored (took 118.73µs)
Day 6: Tuning Trouble
 - Part 1: censored (took 9.35µs)
 - Part 2: censored (took 22.16µs)
Day 7: No Space Left On Device
 - Part 1: censored (took 59.58µs)
 - Part 2: censored (took 59.14µs)
Day 8: Treetop Tree House
 - Part 1: censored (took 270.18µs)
 - Part 2: censored (took 222.14µs)
Day 9: Rope Bridge
 - Part 1: censored (took 206.72µs)
 - Part 2: censored (took 348.76µs)
Took a total of 1.75ms
```

Averaged over 10 runs (using `--bench` option):

```
~ Advent of Code 2022 ~
Day 1: Calorie Counting
 - Part 1: censored (took 18.06µs)
 - Part 2: censored (took 21.20µs)
Day 2: Rock Paper Scissors
 - Part 1: censored (took 37.82µs)
 - Part 2: censored (took 43.38µs)
Day 3: Rucksack Reorganization
 - Part 1: censored (took 25.64µs)
 - Part 2: censored (took 17.57µs)
Day 4: Camp Cleanup
 - Part 1: censored (took 37.39µs)
 - Part 2: censored (took 36.06µs)
Day 5: Supply Stacks
 - Part 1: censored (took 108.14µs)
 - Part 2: censored (took 98.65µs)
Day 6: Tuning Trouble
 - Part 1: censored (took 6.72µs)
 - Part 2: censored (took 21.86µs)
Day 7: No Space Left On Device
 - Part 1: censored (took 51.45µs)
 - Part 2: censored (took 47.08µs)
Day 8: Treetop Tree House
 - Part 1: censored (took 255.76µs)
 - Part 2: censored (took 207.88µs)
Day 9: Rope Bridge
 - Part 1: censored (took 200.38µs)
 - Part 2: censored (took 344.93µs)
Took a total of 1.58ms
```
