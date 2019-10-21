[![Build Status](https://travis-ci.org/shnewto/ashpaper.svg?branch=master)](https://travis-ci.org/shnewto/ashpaper)
[![Crates.io Version](https://img.shields.io/crates/v/ashpaper.svg)](https://crates.io/crates/ashpaper)
[![Crates.io](https://img.shields.io/crates/d/ashpaper.svg)](https://crates.io/crates/ashpaper)

# AshPaper
An inpterpreter for the Esopo language AshPaper conceived by [William Hicks](https://github.com/wphicks). You can read about it and the Esopo project n Willian Hick's own words [here](https://wphicks.github.io/esopo/). Daniel Temkin also wrote about it on esoteric.codes, you can read that [here](https://esoteric.codes/blog/esopo-turing-complete-poetry). And of course the spec! Checkout that out [here](https://github.com/wphicks/Esopo/blob/master/AshPaper/informal_specs.txt).

## How it works

You can execute poetry!

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
poem or calculator or nothing
how lovely can it be?
```
Using this library, you can run it with a program that looked like this:
```rust
extern crate ashpaper;

use std::fs;

pub fn main() {
    let fname = "lovely-poem.eso";
    let contents = fs::read_to_string(fname).expect("Something went wrong reading input file!");
    ashpaper::program::execute(&contents);
}
```

And it will print the following to stdout:
```ignore
24
```

## Some caveats about compliance with the informal spec
- It's entirely possible at this point that some of my implementation deviates from the spec in unintended ways. If you spot anything like that, please raise an issue :heart: :heart:
- As I've read it, the rule for 'storing syllables' used in the title has less precedence that the rest of the rules, that means, for a program to successfully consume input, there must be no others instructions present.
- The alliteration and rhyming rules are still unimplemented.
