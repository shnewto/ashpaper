[![Build Status](https://travis-ci.org/shnewto/ashpaper.svg?branch=master)](https://travis-ci.org/shnewto/ashpaper)
[![codecov](https://codecov.io/gh/shnewto/ashpaper/branch/master/graph/badge.svg)](https://codecov.io/gh/shnewto/ashpaper)
[![Crates.io Version](https://img.shields.io/crates/v/ashpaper.svg)](https://crates.io/crates/ashpaper)
[![Crates.io](https://img.shields.io/crates/d/ashpaper.svg)](https://crates.io/crates/ashpaper)

# AshPaper
An inpterpreter for the Esopo language AshPaper conceived by [William Hicks](https://github.com/wphicks). You can read about it and the Esopo project n Willian Hick's own words [here](https://wphicks.github.io/esopo/). Daniel Temkin also wrote about it on esoteric.codes, you can read that [here](https://esoteric.codes/blog/esopo-turing-complete-poetry). And of course the spec! Checkout that out [here](https://github.com/wphicks/Esopo/blob/master/AshPaper/informal_specs.txt).

## How it works

Poetry is your program.

You have two registers at your disposal, r0 and r1 which store signed integers (`i64`).
You also have an stack which can store signed integers (bounds are only that of `Vec<i64>`).

Here are the instructions at your disposal (in order that they get precedence):
- _End rhyme with previous line_: Unimplemented.
- Line contains `/`: If the value in the active register is greater than the number of syllables in the line, go to the line number that corresponds to the value in the **non-active** register. If abs(n) <= lines then n, else n % lines.
- _Capital letter appears inside a word_: Negate the active register.
- _Capital letter appears at the beginning of a word_: Multiply registers and store result in the active register.
- _Line contains the words 'like' or 'as'_: Add registers and store in the active register.
- _Line contains `?`_: Print ASCII character associated with value of the active register. If abs(n) <= u8::MAX n, else n % u8::MAX.
- _Line contains `.`_: Print integer value of the active register.
- _Line contains `,`_: Pop from the stack and store in the active register.
- _Line contains `-`_: Push the value of the active register to the stack.
- _Alleteration of consecutive words_: Unimplemented.
- _Blank line_: no-op.
- _Everything else_: Store number of syllables in the line to the active register.


Let's take this poem in a file called lovely-poem.eso.This poem-program (poegram?) calculates factorials and input in the number of syllables in the title. (I learned a lot from reading the poem "other woodwork" by William Hicks)
```ignore
lovely poem

  it is a calculator, like a
      poem, is a poem, and finds
        factori-
          als
  The input is the syllAbles
in the title, count them, as one counts
  (q) what other poem, programs can be writ
  (a) anything a Turing
    machine-machine-machine
    would do
re/cur
    sion works too, in poems, programs, and this
       a lovely.
poem or a calculator or nothing
how lovely can it be?
```
Using this library, you can run it with a program that looked like this:
```rust
extern crate ashpaper;

use std::fs;

pub fn main() {
    let fname = "lovely-poem.eso";
    let contents = fs::read_to_string(fname).expect("Something went wrong reading input file!");
    if let Ok(res) = ashpaper::program::execute(&contents) {
        print!("{}", res);
    } else {
        // TODO: really need some helpful error reporting
        eprintln!("Error executing program!");
    }
}
```

And it will print the following to stdout:
```ignore
24
```

## Some caveats about compliance with the informal spec
- It's entirely possible at this point that some of my implementation deviates from the spec in unintended ways. If you spot anything like that, please raise an issue :heart: :heart:
- The alliteration and rhyming rules are still unimplemented.
