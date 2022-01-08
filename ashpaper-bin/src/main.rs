/// # ashpaper-bin
/// CLI for the [ashpaper crate](https://crates.io/crates/ashpaper), an inpterpreter for Esopo language AshPaper conceived by William Hicks. Now you can run poetry-programs from the command line!
///
/// ## Usage
///
/// Take the following "poegram" called 'lovely-poem.eso' (in this repositories poetry directory):
/// ```txt
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
/// ;; (I learned a lot from reading the poem 'other woodwork' by William Hicks)
/// ;; Input is the number of syllables in the title.
/// ```
///
/// You can run it with:
/// ```bash
/// ashpaper-bin ashpaper-bin/poems/lovely-poem.eso
/// ```
///
/// And it will produce the output:
/// ```txt
/// 24
/// ```
extern crate ashpaper;
extern crate clap;
extern crate log;

use clap::App;
use std::fs;

#[cfg(not(tarpaulin_include))]
pub fn main() {
    let matches = App::new("ashpaper")
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about("An AshPaper interpreter that executes poetry!")
        .arg(clap::arg!(<INPUT>    "'.eso file to compile'"))
        .get_matches();

    env_logger::init();

    let fname = matches.value_of("INPUT").unwrap();
    let contents = fs::read_to_string(fname).expect("Something went wrong reading input file!");

    match ashpaper::program::execute(&contents) {
        Ok(res) => print!("{}", res),
        Err(e) => eprintln!("{}", e),
    }
}
