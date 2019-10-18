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
poem or calculator or nothing
how lovely can it be?

;; This poem-program (poegram?) calculates factorials.
;; (I learned a lot from reading the poem 'other woodwork' by William Hicks)
;; Input is the number of syllables in the title.
```

You can run it with:
```
ashpaper-bin lovely-poem.eso
```

And it will produce the output:
```
24
```

## Issues
This project is really bare bones atm, if you find something broken, please raise an issue :heart: :heart:
