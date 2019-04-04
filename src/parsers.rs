use crate::program::Register;

named!(pub register< &[u8], Register >, alt!(register2 | register1));

const WS: &str = " \t\r\n";

named!( 
    register2< &[u8], Register>,
    do_parse!(
        take_while_m_n!(1,1, nom::is_space) >>
        (Register::Register2)
    )
);

named!( 
    register1< &[u8], Register>,
    do_parse!(
        peek!(take!(1))>>
        (Register::Register1)
    )
);