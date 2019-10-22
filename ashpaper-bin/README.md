[![Build Status](https://travis-ci.org/shnewto/ashpaper.svg?branch=master)](https://travis-ci.org/shnewto/ashpaper)
[![Crates.io Version](https://img.shields.io/crates/v/ashpaper-bin.svg)](https://crates.io/crates/ashpaper-bin)
[![Crates.io](https://img.shields.io/crates/d/ashpaper-bin.svg)](https://crates.io/crates/ashpaper-bin)

# ashpaper-bin
CLI for the [ashpaper crate](https://crates.io/crates/ashpaper), an inpterpreter for Esopo language AshPaper conceived by William Hicks. Now you can run poetry-programs from the command line!

## Usage

Take the following "poegram" called 'lovely-poem.eso' (in this repositories poetry directory):
```
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

You can run it with:
```
ashpaper lovely-poem.eso
```

And it will produce the String:
```
24
```

When `RUST_LOG=info` (`RUST_LOG=info ashpaper lovely-poem.eso`) is set, you can get at program evaluation info. Here's what lovely-poem.eso looks like.
```txt
instruction                                         |  r0  |  r1  |  stack
--------------------------------------------------- | ---- | ---- | -------
lovely poem                                         |  4   |  0   | []
                                                    |  4   |  0   | []
  it is a calculator, like a                        |  4   |  4   | []
      poem, is a poem, and finds                    |  4   |  4   | []
        factori-                                    |  4   |  4   | [4]
          als                                       |  4   |  1   | [4]
  The input is the syllAbles                        |  4   |  -1  | [4]
in the title, count them, as one counts             |  3   |  -1  | [4]
  (q) what other poem, programs can be writ         |  3   |  4   | []
  (a) anything a Turing                             |  3   |  12  | []
    machine-machine-machine                         |  3   |  12  | [12]
    would do                                        |  3   |  2   | [12]
  it is a calculator, like a                        |  3   |  5   | [12]
      poem, is a poem, and finds                    |  3   |  12  | []
        factori-                                    |  3   |  12  | [12]
          als                                       |  3   |  1   | [12]
  The input is the syllAbles                        |  3   |  -1  | [12]
in the title, count them, as one counts             |  2   |  -1  | [12]
  (q) what other poem, programs can be writ         |  2   |  12  | []
  (a) anything a Turing                             |  2   |  24  | []
    machine-machine-machine                         |  2   |  24  | [24]
    would do                                        |  2   |  2   | [24]
re/cur                                              |  2   |  2   | [24]
    sion works too, in poems, programs, and this    |  2   |  24  | []
       a lovely.                                    |  2   |  24  | []
poem or a calculator or nothing                     |  10  |  24  | []
how lovely can it be?                               |  10  |  24  | []
```
## Issues
This project is really bare bones atm, if you find something broken, please raise an issue :heart: :heart:
