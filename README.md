# Rusty Xmas
Extensible Skeleton for Rust Advent of Code Submissions

Rusty Xmas is a Rust Template for Advent of Code Submissions. It was made to simplify and standardize my own process of submitting AoC solutions.

## Goals
Rusty Xmas should be:
- Simple
  - Adding a new solution should be as simple as writing its code.
  - I/O and Display functionality should not be rewritten in every solution.
  - The focus here should be on making YOUR solutions as DRY as possible, so any code that is repeated across years should be built-in to the Skeleton.
- Reusable
  - Rusty Xmas' Project Structure should make it easy to reuse functions in the same year or across years.
- Pretty
  - As long as it doesn't sacrifice any of the other goals, Rusty Xmas' UI should be as pretty as possible.
  - TODO: While not necessary, it should be possible and simple to chart solutions, leaving as much work as possible up to the underlying systems.
- TODO: Benchmarkable
  - Since Rusty Xmas is more focused on perfecting solutions than speed, it should be simple to Benchmark and Profile solutions.
  - This should be built-in to the underlying template, similarly to I/O.
  - In a related note, because most AoC challenges should be fast to solve in Rust (especially with --release), Rusty Xmas should also support custom (bigboy) inputs.

## Non-Goals
Rusty Xmas doesn't have to be:
- Fast
  - Rusty Xmas is not intended to be used for getting the fastest submission times.
  - Any Template that enforces structure is going to be slower than unstructured files. So this would be an impossible goal.
  - While speed is a non-goal, Rusty Xmas should also not be glacial, as that would be a massive pain point when iterating through solutions.
- Compatible across Websites
  - Rusty Xmas is intended for Advent of Code, not Leetcode, Project Euler, Hackerrank or any other programming challenge website.
  - I might work on similar templates for other websites, but relying on a specific website lets me make assumptions that improve the overall UX of this Template.
- GUI Focused
  - While I can see how a GUI would help on the Prettyness of this, it's not a goal.
  - The reason for this is that I personally work in the Terminal, so I would have to at least support a TUI version as well.
- Solved
  - The current versions of this Skeleton does have solutions, but these are being added so I can get a notion of what should be done to improve the skeleton.
  - Eventually these solutions will be moved to their own repository, but Tests should be kept where possible, as they should be identical across users.
