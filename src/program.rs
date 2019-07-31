use wordsworth;
pub struct AshPaper;

fn push_syllables_to_stack() {}
fn goto_line_nonactive_register() {}
fn negate_active_register() {}
fn multiply_registers() {}
fn add_registers() {}
fn print_active_register_char() {}
fn print_active_register_val() {}
fn pop_stack_to_active_register() {}
fn push_active_register_to_stack() {}
fn goto_line_active_register() {}
fn noop() {}
fn store_syllables_to_active_register() {}

fn instructions(program: &str) -> Vec<String> {
    vec![String::from("")]
}

pub fn execute(program: &str) {
    let mut val = input
    print!("|{:#?}|", parsers::instruction(program));
}

mod parsers {
    use nom::{
        bytes::complete::{tag, take_till},
        sequence::tuple,
        IResult,
    };
    pub fn instruction(input: &str) -> IResult<&str, &str> {
        take_till(|c| c == '\n')(input)
    }
}
