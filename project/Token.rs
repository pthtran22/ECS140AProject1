use crate::Cstream::CStream;
#[derive(Copy, Clone)]
#[derive(Debug)]
pub enum TokenType {
    IntConstant,
    FloatConstant,
    Keyword,
    Operator,
    Identifier,
    Invalid,
    Init,
}

 
pub struct Token {
    pub text: String,
    pub token_type: TokenType,
    pub line_num: i32,
    pub char_pos: i32,
}

impl Token {
    pub fn new(t: &str, token_type:TokenType, line_num: i32, char_pos:i32) -> Token { // reads in file content
        Token {
            text: t.to_string(),
            token_type: token_type, // what should its initialize be??
            line_num: line_num, // will change to 0 at first line?
            char_pos: char_pos,
            // overall_pos: -1,
        }
    }

}


// pub fn run() {
//     // Token::new("hi", TokenType::Keyword);
//     // println!("hi");
//     // Cstream::run();
// }