

extern crate custom_error;
use crate::Token::Token;
use crate::Token::TokenType;
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


    fn get_cur_token(&mut self, token: &mut String, line_num:i32, char_pos:i32) -> Token { // check token type & call Token
            let token_type = check_TokenType(token.to_owned());
            Token::new(&*token, token_type, line_num, char_pos)

    }

    
    // using content from Cstream use get_next_char to get the char and call get_cur_token to return Token Type and save it into a vector and return
    fn get_all_tokens(&mut self, file:String) -> &mut Vec<Token> {
        let mut token: &mut String = &mut "".to_string(); 
        
        let mut char_index: i32 = 0;
        let operators = vec!["(", ",", ")", "{", "}", "=", "==", "<", ">", "<=", ">=", "!=", "+", "-", "*", "/", ";"];


        for c in 0..file.to_string().len() {
            if file.chars().nth(c).unwrap() == '\n' {
                self.line_num += 1;
                char_index = -1;
                continue;
            }
            if c == file.to_string().len()-1 { // end of file
                token.push_str(&file.chars().nth(c).unwrap().to_string()); // }
                //token = + &file.chars().nth(c).unwrap().to_string();
                //go back, for now assume the last char would always be '}'
                char_index = char_index + 1;
                self.char_pos = char_index;
                let token_obj: Token = self.get_cur_token(token, self.line_num, self.char_pos);
                self.all_tokens.push(token_obj);
                
                // go back, return the vector <all_tokens>
                return &mut self.all_tokens;
            }

            if file.chars().nth(c).unwrap() == ' ' {
                let token_obj: Token = self.get_cur_token(token, self.line_num, self.char_pos);
                self.all_tokens.push(token_obj);
                //token = &mut "".to_string(); 
                *token = "".to_string();
                
                char_index = char_index + 1;
                self.char_pos = char_index;
                
                
                
                continue;
            }
            
            if operators.contains(&&*(file.chars().nth(c).unwrap().to_string()).to_string()) {
                // token before operator
                let token_obj: Token = self.get_cur_token(token, self.line_num, self.char_pos);
                self.all_tokens.push(token_obj);
            
                // operator
                //token = &mut "".to_string(); 
                *token = "".to_string();
                token.push(file.chars().nth(c).unwrap());
                char_index = char_index + 1;
                self.char_pos = char_index; 
                let token_obj: Token = self.get_cur_token(token, self.line_num, self.char_pos);
                self.all_tokens.push(token_obj);


            }

            if (file.chars().nth(c).unwrap()).is_alphanumeric() || file.chars().nth(c).unwrap().to_string() == "." || file.chars().nth(c).unwrap().to_string() == "_" {
                token.push(file.chars().nth(c).unwrap());
                char_index = char_index + 1;
                // check if the previous char is /n
                if char_index == 0{
                    self.char_pos = char_index;
                }
                
            }
            else{
                //COME BACK INCORRECT
                return &mut self.all_tokens;
            }
        }

        return &mut self.all_tokens;

    }

}


pub fn check_TokenType(text:String) -> TokenType {
    let keywords = vec!["unsigned", "char", "short", "int", "long", "float", "double", "while", "if", "return", "void", "main"];
    // Keyword
    if keywords.contains(&&*text) {
        return TokenType::Keyword;
    }

    // Identifier
    if text.chars().nth(0).unwrap() == '_' || (text.chars().nth(0).unwrap()).is_alphabetic() {
        if text.len() == 1 {
            return TokenType::Identifier;
        }
        for c in 1..text.to_string().len() {
            if text.chars().nth(c).unwrap() != '_' && !((text.chars().nth(0).unwrap()).is_alphanumeric()) {
                return TokenType::Invalid;
            }
        }
        return TokenType::Identifier;
    }


    // IntConstant
    let mut dotCount = 0;
    // checks if the first two characters are (-) digit
    if text.chars().nth(0).unwrap() == '-' || (text.chars().nth(0).unwrap()).is_digit(10) {
        if text.len() == 1 { // only digit
            if (text.chars().nth(0).unwrap()).is_digit(10) {
                return TokenType::IntConstant;
            }
            return TokenType::Invalid;
        }
        // checks if the rest of the characters are digit.{digit}
        for c in 1..text.to_string().len() {
            if !((text.chars().nth(0).unwrap()).is_digit(10)) {
                if text.chars().nth(c).unwrap() == '.' && c !=  text.len() - 1{
                    dotCount = dotCount + 1;
                }
                return TokenType::Invalid;
            }
        }
        if dotCount == 0 {
            return TokenType::IntConstant;
        }
        if dotCount == 1 {
            return TokenType::FloatConstant;
        }
        else {
            return TokenType::Invalid;
        }
    }
    else {
        return TokenType::Invalid;
    }
}

pub fn run(filename:&str) {
    let mut ex = Scanner::new(filename.to_string());
    ex.get_all_tokens(filename.to_string());
    //println!("{}", filename.to_string().len())
    
    // let all_tokens: Vec<TokenType>;  // initilize empty vector
    // let mut ex = Scanner::new(Cstream.get_content());
}