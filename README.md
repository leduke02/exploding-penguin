# Oregon Trail Starter Project

Every team starts from this same code. All functions compile and return a harmless default,
so the project builds and runs from day one. Replace the skeleton bodies with your own code.

## Layout

| File | Owner | Notes |
|---|---|---|
| `src/types.rs` | Instructor | Shared types. Do not edit on your own. |
| `src/rng.rs` | Instructor | `rnd()`. Use it for all randomness. Do not use the `rand` crate directly. |
| `src/io.rs` | Instructor | `read_line`, `ask`, `ask_number`, `fmt_num`, `tab`. Do not read stdin directly. |
| `src/setup.rs` | Part 1 student | 5 functions. |
| `src/fort_hunting.rs` | Part 2 student | 5 functions. |
| `src/travel.rs` | Part 3 student | 5 functions, plus the `Hostility` type. |
| `src/events.rs` | Part 4 student | 6 functions. On a 3-person team, leave this file untouched: it is the placeholder. |
| `src/main.rs` | Team | Written together at Checkpoint 3. |

## Build and run

```
cargo build
cargo run
```

## Rules

- Work only in your own part's file.
- Keep each function's signature. If your part needs a signature or a type changed, raise
  it with your team and your instructor first.
- When the parts are integrated, remove the `#![allow(...)]` line at the top of `main.rs`
  and fix the warnings it was hiding.
