# Advent of Code 2020

This year's Advent of Code! Each day is its own binary in `src/bin`, with common
utilities in the `aoc2020` crate in `src`. Proc macro magic is in `aoc2020-derive`.

## TODO

- [ ] Sort out crate imports in proc macro. `p2` doesn't directly link to `regex` but it still works because `aoc2020` has it in `Cargo.toml`. Also annoying that we have to import lazy static.
- [ ] Optional fields in regex macro
- [x] ~~Repeated fields in regex macro turning into a `Vec`~~. Not possible due to regex capturing only last match.
- [ ] Enum variants macro parser?
