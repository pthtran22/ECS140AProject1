
// mod Cstream::CStream;

extern crate custom_error;
use crate::Token::Token;
use crate::Token::check_TokenType;
use crate::Cstream::CStream;
use custom_error::custom_error;

custom_error!{MyError
    General{line_num:i32, char_pos:i32} = "Error"}



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

    
    // pub fn check_TokenType(mut self, text:String) -> <Token::Token as Trait>::TokenType {
    //     let keywords = vec!["unsigned", "char", "short", "int", "long", "float", "double", "while", "if", "return", "void", "main"];
    //     // Keyword
    //     if keywords.contains(text) {
    //         return Token::TokenType::Keyword;
    //     }

    //     // Identifier
    //     if text.chars().nth(0).unwrap() == '_' || (text.chars().nth(0).unwrap()).is_alphabetic() {
    //         if text.to_string.len() == 1 {
    //             return Token::TokenType::Identifier;
    //         }
    //         for c in 1..text.to_string().len() {
    //             if text.chars().nth(c).unwrap() != '_' && !((text.chars().nth(0).unwrap()).is_alphanumeric()) {
    //                 return Token::TokenType::Invalid;
    //             }
    //         }
    //         return Token::TokenType::Identifier;
    //     }


    //     // IntConstant
    //     let dotCount = 0;
    //     // checks if the first two characters are (-) digit
    //     if text.chars().nth(0).unwrap() == '-' || (text.chars().nth(0).unwrap()).is_digit() {
    //         if text.to_string.len() == 1 { // only digit
    //             if (text.chars().nth(0).unwrap()).is_digit() {
    //                 return Token::TokenType::IntConstant;
    //             }
    //             return Token::TokenType::Invalid;
    //         }
    //         // checks if the rest of the characters are digit.{digit}
    //         for c in 1..text.to_string().len() {
    //             if !((text.chars().nth(0).unwrap()).is_digit()) {
    //                 if text.chars().nth(c).unwrap() == '.' && c !=  text.to_string.len() - 1{
    //                     dotCount += 1;
    //                 }
    //                 return Token::TokenType::Invalid;
    //             }
    //         }
    //         if dotCount == 0 {
    //             return Token::TokenType::IntConstant;
    //         }
    //         if dotCount == 1 {
    //             return Token::TokenType::FloatConstant;
    //         }
    //         else {
    //             return Token::TokenType::Invalid;
    //         }
    //     }
    // }
    


    fn get_cur_token(mut self, token:String, line_num:i32, char_pos:i32) -> Token { // check token type & call Token
            let token_type = Token::check_TokenType(token);
            Token::new(token, token_type, line_num, char_pos)
      

    }

    // using content from Cstream use get_next_char to get the char and call get_cur_token to return Token Type and save it into a vector and return
    fn get_all_tokens(mut self, file:String) -> Vec<Token> {
        let token: String = ""; 
        let char_index: i32 = 0;
        let operators = vec!["(", ",", ")", "{", "}", "=", "==", "<", ">", "<=", ">=", "!=", "+", "-", "*", "/", ";"];


        for c in 0..file.to_string().len() {
            if file.chars().nth(c).unwrap() == '\n' {
                self.line_num += 1;
                char_index = -1;
                continue;
            }
            if c == file.to_string().len()-1 { // end of file
                token.push(file.chars().nth(c).unwrap()); // }
                // come back, assume it's always a }
                char_index = char_index + 1;
                self.char_pos = char_index;
                self.all_tokens.push(self.get_cur_token(token, self.line_num, self.char_pos));
                // go back
                return 
            }
            
            if file.chars().nth(c).unwrap() == " " {
                self.all_tokens.push(self.get_cur_token(token, self.line_num, self.char_pos));
                token = "";
                
                char_index = char_index + 1;
                self.char_pos = char_index;
                
                
                
                continue;
            }
            
            if operators.contains(file.chars().nth(c).unwrap().to_string()) {
                // token before operator
                self.all_tokens.push(self.get_cur_token(token, self.line_num, self.char_pos));
                // operator
                token = "";
                token.push(file.chars().nth(c).unwrap());
                char_index = char_index + 1;
                self.char_pos = char_index;
                self.all_tokens.push(self.get_cur_token(token, self.line_num, self.char_pos));


            }

            if (file.chars().nth(c).unwrap().to_string()).is_alphanumeric() || file.chars().nth(c).unwrap().to_string() == "." || file.chars().nth(c).unwrap().to_string() == "_" {
                token.push(file.chars().nth(c).unwrap());
                char_index = char_index + 1;
                // check if the previous char is /n
                if char_index == 0{
                    self.char_pos = char_index;
                }
                
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
    ex.get_all_tokens(filename.to_string());
    //println!("{}", filename.to_string().len())
    
    // let all_tokens: Vec<TokenType>;  // initilize empty vector
    // let mut ex = Scanner::new(Cstream.get_content());
}