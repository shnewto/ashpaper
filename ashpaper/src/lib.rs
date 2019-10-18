/// # AshPaper
/// An inpterpreter for the Esopo language AshPaper conceived by [William Hicks](https://github.com/wphicks). You can read about it and the Esopo project n Willian Hick's own words [here](https://wphicks.github.io/esopo/). Daniel Temkin also wrote about it on esoteric.codes, you can read that [here](https://esoteric.codes/blog/esopo-turing-complete-poetry). And of course the spec! Checkout that out [here](https://github.com/wphicks/Esopo/blob/master/AshPaper/informal_specs.txt).
///
/// ## How it works
///
/// You can execute poetry!
///
/// Let's take this poem in a file called lovely-poem.eso:
/// ```ignore
/// lovely poem
///   it is a calculator, like a
///       poem, is a poem, and finds
///         factori-
///           als
///   The input is the syllAbles
/// in the title, count them, as one counts
///   (q) what other poem, programs can be writ
///   (a) anything a Turing
///     machine-machine-machine
///     would do
/// re/cur
///     sion works too, in poems, programs, and this
///        a lovely.
/// poem or calculator or nothing
/// how lovely can it be?
///
/// ;; This poem-program (poegram?) calculates factorials.
/// ;; (I learned a lot from reading the poem "other woodwork" by William Hicks)
/// ;; Input is the number of syllables in the title.
/// ```
/// Using this library, you can run it with a program that looked like this:
/// ```ignore
/// extern crate ashpaper;
///
/// use std::fs;
///
/// pub fn main() {
///     let fname = "lovely-poem.eso";
///     let contents = fs::read_to_string(fname).expect("Something went wrong reading input file!");
///     ashpaper::program::execute(&contents);
/// }
/// ```
///
/// And it will produce the output:
/// ```ignore
/// 24
/// ```
///
/// ## Some caveats about compliance with the informal spec
/// - The wording around when to use a specific register vs. the active register has lead me to make some choices. While I've attempted to reproduce the behavior of the author's implementation, you may discover that it deviates. If you do, please raise an issue :heart: :heart:
/// - There are rules for the words 'like' and 'as' that I've taken to mean simply the presence of the patterns 'like' or 'as'. So the rule for 'like' has the potential to match 'likes' and 'has' to match the rule for 'as' (barring any other rule that takes precedence).
/// - Comments are not a part of the spec but are implemented in this version. They are denoted by lines beginning with the `;;` characters. For now, they will have an effect on the execution unless they are put at the end of the source file.
/// - As I've read it, the rule for 'storing syllables' used in the title has less precedence that the rest of the rules, that means, for a program to successfully consume input, there must be no others instructions present.
extern crate pest;
#[macro_use]
extern crate pest_derive;
extern crate wordsworth;
pub mod program;
