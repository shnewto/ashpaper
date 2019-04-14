use crate::program::Register;
use crate::program::Memory;

named!(pub register< &[u8], Register >, alt!(register2 | register1));

named!( 
    register2< &[u8], Register>,
    do_parse!(
        take_while_m_n!(1,1, nom::is_space) >>
        (Memory::new().r2)
    )
);

named!( 
    register1< &[u8], Register>,
    do_parse!(
        peek!(take!(1))>>
        (Memory::new().r1)
    )
);