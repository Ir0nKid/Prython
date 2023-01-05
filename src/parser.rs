pub enum ParseErr {}

pub struct Parser {}

impl Parser {
    pub fn new() -> Parser {
        Self {}
    }

    pub fn parse(&self, py_byte_code: u16) -> Result<(), ParseErr> {
        println!("{:?}", py_byte_code);
        Ok(())
    }
}
