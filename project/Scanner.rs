extern crate custom_error;
use crate::Token::Token;
use crate::Token::TokenType;
use crate::Cstream::CStream;
use custom_error::custom_error;

custom_error!{MyError
    General{line_num:i32, char_pos:i32} = "Error"}


pub struct Scanner {
    pub all_tokens: Vec<Token>,
    pub line_num: i32,
    pub char_pos: i32,
    pub file: String,
}


impl Scanner {
    pub fn new(file: String) -> Scanner {
        Scanner{
            all_tokens: Vec::new(),
            line_num: 0,
            char_pos:0,
            file: file
        }    
    }


    pub fn get_cur_token(&mut self, token: &mut String, line_num:i32, char_pos:i32) -> Token { // check token type & call Token
            let token_type = check_TokenType(token.to_owned());
            Token::new(&*token, token_type, line_num, char_pos)

    }

    
    // using content from Cstream use get_next_char to get the char and call get_cur_token to return Token Type and save it into a vector and return
    pub fn get_all_tokens(&mut self) -> &mut Vec<Token> {

        // let mut space_count = 0;

        let mut token: &mut String = &mut "".to_string(); 
        let mut char_index: i32 = 0;
        let operators = vec!["(", ",", ")", "{", "}", "=", "==", "<", ">", "<=", ">=", "!=", "+", "-", "*", "/", ";"];
        let mut flag_space = 0;

        // loop over the file char by char
        for c in 0..self.file.to_string().len() {
            // println!("char pos {}", self.char_pos);
            // println!("char index {}", char_index);
            // println!("char: {}", self.file.chars().nth(c).unwrap());
            // check for newline
            if self.file.chars().nth(c).unwrap() == '\n' {
                self.line_num += 1;
                char_index = -1;
                flag_space = 0;
                continue;
            }

            else if c == self.file.to_string().len()-1 { // end of file
                token.push_str(&self.file.chars().nth(c).unwrap().to_string()); // }
                //token = + &file.chars().nth(c).unwrap().to_string();
                //go back, for now assume the last char would always be '}'
                char_index = char_index + 1;
                self.char_pos = char_index;
                let token_obj: Token = self.get_cur_token(token, self.line_num, self.char_pos);
                self.all_tokens.push(token_obj);
                
                // go back, return the vector <all_tokens>
                return &mut self.all_tokens;
            }

            else if self.file.chars().nth(c).unwrap() == ' ' {  // float Foo  "_ _ _ _"
                // space_count = space_count + 1;
                // println!("{}",space_count);

                if flag_space == 0 {
                    // push token before ' '
                    if token.is_empty() {
                        char_index = char_index + 1;
                        continue;
                    }
                    
                    self.char_pos = char_index;
                    // println!("{}", char_index);

                    let token_obj: Token = self.get_cur_token(token, self.line_num, self.char_pos);
                    self.all_tokens.push(token_obj);
                    char_index = char_index + 1;

                    // println!("{}", char_index);
                
                    *token = "".to_string();
                    char_index = char_index + 1;
                    self.char_pos = char_index;

                    // println!("{}", char_index);
                    
                    flag_space = 1;
                }
                else {
                    if token.is_empty() {
                        char_index = char_index + 1;
                        continue;
                    }
                    let token_obj: Token = self.get_cur_token(token, self.line_num, self.char_pos);
                    self.all_tokens.push(token_obj);
                    *token = "".to_string();
                    self.char_pos = char_index;
                    println!("{}", char_index);

                }
                
        
                continue;
            }
            
            else if operators.contains(&&*(self.file.chars().nth(c).unwrap().to_string()).to_string()) {
                // edge case: operator followed by another operator, e.g. val);
                if token == "" {
                    token.push(self.file.chars().nth(c).unwrap());
                    let token_obj: Token = self.get_cur_token(token, self.line_num, self.char_pos);
                    self.all_tokens.push(token_obj);
                    self.char_pos = char_index; 
                    char_index = char_index + 1;
                    *token = "".to_string();

                    continue;
                    
                }

                


                // token before operator
                let token_obj: Token = self.get_cur_token(token, self.line_num, self.char_pos);
                self.all_tokens.push(token_obj);
                self.char_pos = char_index; 
                char_index = char_index + 1;
                // operator
                *token = "".to_string();
                token.push(self.file.chars().nth(c).unwrap());
                // char_index = char_index + 1;
                // self.char_pos = char_index; 
                let token_obj: Token = self.get_cur_token(token, self.line_num, self.char_pos);
                self.all_tokens.push(token_obj);
                
                // if the next token is also operator, e.g val); 
                *token = "".to_string();
                self.char_pos = char_index; 
                char_index = char_index + 1;


            }

            else if (self.file.chars().nth(c).unwrap()).is_alphanumeric() || self.file.chars().nth(c).unwrap().to_string() == "." || self.file.chars().nth(c).unwrap().to_string() == "_" {
            
                token.push(self.file.chars().nth(c).unwrap());
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
    if text.is_empty() {
        return TokenType::Invalid;
    }
    let operators = vec!["(", ",", ")", "{", "}", "=", "==", "<", ">", "<=", ">=", "!=", "+", "-", "*", "/", ";"];
    let keywords = vec!["unsigned", "char", "short", "int", "long", "float", "double", "while", "if", "return", "void", "main"];
    // Keyword
    if keywords.contains(&&*text) {
        return TokenType::Keyword;
    }

    if operators.contains(&&*text) {
        return TokenType::Operator;
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

// pub fn run(filename:&str) -> &mut Vec<Token> {
//     let mut ex = Scanner::new(filename.to_string());
//     ex.get_all_tokens(filename.to_string())
//     // for i in 0..token_list.len() {
//     //     println!("{}", token_list[i].text);
//     // }

//     // return token_list;
    
//     //println!("{}", filename.to_string().len())
    
//     // let all_tokens: Vec<TokenType>;  // initilize empty vector
//     // let mut ex = Scanner::new(Cstream.get_content());
// }
