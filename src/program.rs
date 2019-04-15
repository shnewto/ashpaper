// use crate::parsers;
use std::default::Default;
use std::fmt;
use std::slice;

impl Memory {
    pub fn new() -> Memory {
        Default::default()
    }
}

impl Default for Memory {
    fn default() -> Self {
        Memory {
            r1: Register::Register1(0),
            r2: Register::Register2(0),
            stack: vec![],
        }
    }
}

type Insctructions = String;

pub struct Iter<'a> {
    iterator: slice::Iter<'a, Insctructions>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a Insctructions;

    fn next(&mut self) -> Option<Self::Item> {
        self.iterator.next()
    }
}

pub struct IterMut<'a> {
    iterator: slice::IterMut<'a, Insctructions>,
}

impl<'a> Iterator for IterMut<'a> {
    type Item = &'a mut Insctructions;

    fn next(&mut self) -> Option<Self::Item> {
        self.iterator.next()
    }
}

impl Program {
    /// Construct a new `Program`
    pub fn new() -> Program {
        Program {
            ..Default::default()
        }
    }

    /// Construct an `Program` from `Insctructions`s
    fn from_parts(v: Vec<Insctructions>) -> Program {
        Program {
            instructions: v,
            ..Default::default()
        }
    }

    // Get `Program` by parsing a string
    pub fn from_str(s: &str) -> Program {
        Program {
            instructions: s.lines().map(|i| String::from(i)).collect::<Vec<String>>(),
            ..Default::default()
        }
    }

    /// Get iterator of the `Program`
    pub fn iter(&self) -> Iter {
        Iter {
            iterator: self.instructions.iter(),
        }
    }

    /// Get mutable iterator of the `Production`
    pub fn iter_mut(&mut self) -> IterMut {
        IterMut {
            iterator: self.instructions.iter_mut(),
        }
    }
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.instructions
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
                .join("")
        )
    }
}

#[derive(Clone, Debug)]
pub enum Command {
    GOTOIF,
    STORE,
    NEGATE,
    MULTIPLY,
    ADD,
    PRINT,
    PRINTR,
    POP,
    PUSH,
    GOTO,
}

pub struct Memory {
    pub r1: Register,
    pub r2: Register,
    pub stack: Vec<i64>,
}

pub enum Register {
    Register1(i64),
    Register2(i64),
}

#[derive(Default)]
pub struct Program {
    instructions: Vec<Insctructions>,
    memory: Memory,
    execution_index: u64,
    output: String,
}

const ASH_PAPER: &str = "
    <poem>                 ::= <lines>
    <lines>                ::= <line> | <line> <lines>
    <line>                 ::= <words> <eol> | <opt whitespace> <eol>
                            | <whitespace first> <words> <eol>
    <whitespace first>     ::= <whitespace> <text>
    <words>                ::= <word> | <word> <whitespace> <words>
    <word>                 ::= <characters> | <like> | <as>
    <characters>           ::= <character> | <character> <characters>
    <character>            ::= <upper case> | <lower case> | <digit> | <symbol>
    <like>                 ::= 'like'
    <as>                   ::= 'as'
    <whitespace>           ::= <whitespace character> 
                            | <whitespace character> <whitespace>
    <whitespace character> ::= ' ' | '\t' 
    <opt whitespace>       ::= '' | <whitespace>
    <eol>                  ::= '\n' | '\r'
    <lower case>           ::= 'a' | 'b' | 'c' | 'd'
                            | 'e' | 'f' | 'g' | 'h' | 'i' | 'j'
                            | 'k' | 'l' | 'm' | 'n' | 'o' | 'p'
                            | 'q' | 'r' | 's' | 't' | 'u' | 'v'
                            | 'w' | 'x' | 'y' | 'z'
    <upper case>           ::= 'A' | 'B' | 'C' | 'D' | 'E' | 'F'
                            | 'G' | 'H' | 'I' | 'J' | 'K' | 'L'
                            | 'M' | 'N' | 'O' | 'P' | 'Q' | 'R'
                            | 'S' | 'T' | 'U' | 'V' | 'W' | 'X'
                            | 'Y' | 'Z' 
    <digit>                ::= '0' | '1' | '2' | '3' | '4' | '5'
                            | '6' | '7' | '8' | '9'
    <symbol>               ::=  '|' | '!' | '#' | '$' | '%' 
                            | '&' | '(' | ')' | '*' | '+'
                            | <comma>  | <hyphen> | <full stop> 
                            | <forward slash> | <question mark>
                            | ':' | ';' |'>' | '=' | '<' | '@' 
                            | '[' | '\\' | ']' | '^' | '_' | '`'
                            | '{{' | '}}' | '~' | '\"' | \"'\"
    <question mark> ::= '?'
    <forward slash> ::= '/'
    <full stop> ::= '.'
    <comma> ::= ','
    <hyphen> ::= '-'
    ";

pub fn execute(instructions: &str) -> Program {
    // let mut program = Program::from_str(instructions);

    let ast = bnf::Grammar::from_str(ASH_PAPER);

    match ast {
        Ok(g) => println!("{:#?}", g),
        Err(e) => println!("Failed to make grammar from String: {}", e),
    }
    // for (i, instruction) in program.iter_mut().enumerate() {
    //     println!("|{}|", instruction);
    //     match parsers::register(instruction.as_bytes()) {
    //         Ok((_, Register::Register1(val))) => {
    //             println!("r1:\n{}", instruction);
    //         }

    //         Ok((_, Register::Register2(val))) => {
    //             println!("r2:\n{}", instruction);
    //         }
    //         Err(e) => println!("??:\n{} -- {}", instruction, e),
    //     }
    // }

    Program::new()
    // program
}

fn execute_line(line: usize, instruction: Insctructions) -> Program {
    // let syllables = wordsworth::syllable_counter(&instruction);
    // let register = 0;

    Program::new()
}
