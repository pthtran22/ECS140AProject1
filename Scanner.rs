
// mod Cstream::CStream;
use crate::Token::Token;
use crate::Cstream::CStream;
use custom_error::custom_error;
custom_error!{MyError
    General{line_num:i32, char_pos:i32} = "Error"}

let operators: vec!["(", ",", ")", "{", "}", "=", "==", "<", ">", "<=", ">=", "!=", "+", "-", "*", "/", ";"];

pub struct Scanner {
    all_tokens: Vec<Token>,
    line_num: i32,
    char_pos: i32,
}

impl Scanner {
    pub fn new(file: String) -> Scanner {
        Scanner{
            all_tokens: Vec::new(),
            line_num: 0,
            char_pos:0,
        }    
    }

    fn get_cur_token(mut self, token:String) -> Token { // check token type & call Token
            //?
    }

    // using content from Cstream use get_next_char to get the char and call get_cur_token to return Token Type and save it into a vector and return
    fn get_all_tokens(mut self, file:String) -> Vec<Token> {
        let token: String = ""; 
        let token_first_index: i32 = 0;
        for c in 0..file.to_string().len() {
            if file.chars().nth(c).unwrap() == '\n' {
                self.line_num += 1;
                self.char_pos = -1;
                continue;
            }
            if c == filename.to_string().len()-1 { // end of file
                token.push(file.chars().nth(c).unwrap()); // }
                // go back
                return 
            }
            
            if file.chars().nth(c).unwrap() == " " {
                all_tokens.push(get_cur_token(token));
                token = "";

                self.char_pos =+ 1;
                token_first_index = self.char_pos;
                continue;
            }
            
            if operators.contains(file.chars().nth(c).unwrap().to_string()) {
                all_tokens.push(get_cur_token(token));
                token = ""
                token.push(file.chars().nth(c).unwrap());
                self.char_pos =+ 1;
                token_first_index = self.char_pos;
            }

            if isalphanumeric(file.chars().nth(c).unwrap().to_string()) || file.chars().nth(c).unwrap().to_string() == "." || file.chars().nth(c).unwrap().to_string() == "_" {
                token.push(file.chars().nth(c).unwrap());
                self.char_pos =+ 1;
                token_first_index = self.char_pos;
            }
            else{
                //COME BACK INCORRECT
                return self.all_tokens;
            }
        }

    }
}





pub fn run(filename:&str) {
    let mut ex = Scanner::new(filename.to_string());
    get_all_tokens(filename.to_string());
    //println!("{}", filename.to_string().len())
    
    // let all_tokens: Vec<TokenType>;  // initilize empty vector
    // let mut ex = Scanner::new(Cstream.get_content());
}