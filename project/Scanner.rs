extern crate custom_error;
use crate::Token::Token;
use crate::Token::TokenType;
use crate::Cstream::CStream;
use custom_error::custom_error;

// custom_error!{MyError
//     General{line_num:i32, char_pos:i32} = "Error"}


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
        let mut space_count: i32 = 0;
        let mut token: &mut String = &mut "".to_string(); 
        let mut char_index: i32 = 0;
        let operators = vec!["(", ",", ")", "{", "}", "=", "==", "<", ">", "<=", ">=", "!=", "+", "-", "*", "/", ";"];
        let mut flag_space = 0;
        let mut flag_first_non_space = 0;
        let mut flag_cont_operator = 0;
        let mut after_newline = 0;


        //println!("file len: {}", self.file.to_string().len());
        //println!("last char: {}.", self.file.chars().nth(245).unwrap());

        // loop over the file char by char
        for c in 0..self.file.to_string().len() {
            //println!("c= {}", c);
            //println!("char: {}",self.file.chars().nth(c).unwrap());
            // check for newline
            if self.file.chars().nth(c).unwrap() == '\n' {
                flag_cont_operator = 0;
                after_newline = 1;

                self.line_num += 1;
                char_index = -1;
                flag_space = 0;
                flag_cont_operator = 0;
                *token = "".to_string();
                // if c == self.file.to_string().len()-1 {
                //     // go back, return the vector <all_tokens>
                //     return &mut self.all_tokens;
                // }
                
            }

            else if c == self.file.to_string().len()-2 { // end of file
                //println!("end of file, cur char is {}",self.file.chars().nth(c).unwrap());
                flag_cont_operator = 0;
                after_newline = 0;

                // if file ends with newline
                if self.file.chars().nth(c).unwrap() == '\n' {
                    let token_obj: Token = self.get_cur_token(token, self.line_num, self.char_pos);
                    self.all_tokens.push(token_obj);
                    return &mut self.all_tokens;
                }
                // if file ends with operator
                else {
                    // push whatever before operator
                    let token_obj: Token = self.get_cur_token(token, self.line_num, self.char_pos);
                    self.all_tokens.push(token_obj);
                    *token = "".to_string();
                    char_index = char_index + 1;
                    self.char_pos = char_index;

                    // push the last operator
                    token.push_str(&self.file.chars().nth(c).unwrap().to_string()); 
                    let token_obj: Token = self.get_cur_token(token, self.line_num, self.char_pos);
                    self.all_tokens.push(token_obj);

                    // go back, return the vector <all_tokens>
                    return &mut self.all_tokens;

                }




                // if flag_space == 0 {
                //     self.char_pos = char_index;
                // }

            }

            else if self.file.chars().nth(c).unwrap() == ' ' {  // float Foo  "_ _ _ _"
                
                flag_cont_operator = 0;

                // if whitespace after newline (indentation) e.g. _ _ _ _ float
                // skip whitespace
                if after_newline == 1 {
                    if char_index == -1 {
                        char_index = char_index + 1;
                    }
                    char_index = char_index + 1;
                    self.char_pos = char_index;
                    continue;
                }
                
                else {
                    // if whitespace in the middle, e.g. float_Foo, value_=_
                    // push float
                    if token.is_empty() {
                        char_index = char_index + 1;
                        self.char_pos = char_index;
                        continue;
                    }
                    let token_obj: Token = self.get_cur_token(token, self.line_num, self.char_pos);
                    self.all_tokens.push(token_obj);
                    *token = "".to_string();


                    // skip whitespace
                    char_index = char_index + 1;
                    *token = "".to_string();
                    self.char_pos = char_index;
                }


            }

            
            else if operators.contains(&&*(self.file.chars().nth(c).unwrap().to_string()).to_string()) {
               
            

                //if operator at the end of line, e.g. }/n
                if after_newline == 1 {
                    // push ; current operator
                    if char_index == -1 {
                        char_index = char_index + 1;
                    }
                    token.push(self.file.chars().nth(c).unwrap());
                    self.char_pos = char_index;

                    let token_obj: Token = self.get_cur_token(token, self.line_num, self.char_pos);
                    self.all_tokens.push(token_obj);

                    flag_cont_operator = 0;
                    after_newline = 0;
    
                    char_index = char_index + 1;
                    self.char_pos = char_index;
                    flag_space = 0;
                    flag_cont_operator = 0;
                    *token = "".to_string();
                    
                    continue;
                }


                after_newline = 0;
                // Foo(int or value_=_Foo
                if flag_cont_operator == 0 {

                    flag_cont_operator = flag_cont_operator + 1;
                    // push Foo
                    if token.is_empty() {
                        token.push(self.file.chars().nth(c).unwrap());
                    }

                    // e.g. value_=_ skip whitespace before =
                    if c != 0 && self.file.chars().nth(c-1).unwrap() == ' ' {
                        // push operator =
                        *token = "".to_string();
                        self.char_pos = char_index;
                        token.push(self.file.chars().nth(c).unwrap());
                        if token.is_empty() {
                            token.push(self.file.chars().nth(c).unwrap());
                        }
                       
                        let token_obj: Token = self.get_cur_token(token, self.line_num, self.char_pos);
                        self.all_tokens.push(token_obj);
                        char_index = char_index + 1;
                        self.char_pos = char_index;
                        *token = "".to_string();

                        continue;
                        
                    }
                    
                    let token_obj: Token = self.get_cur_token(token, self.line_num, self.char_pos);
                    self.all_tokens.push(token_obj);

                    // push operator
                    *token = "".to_string();
                    self.char_pos = char_index;
                    token.push(self.file.chars().nth(c).unwrap());
                    if token.is_empty() {
                        token.push(self.file.chars().nth(c).unwrap());
                    }
                
                    let token_obj: Token = self.get_cur_token(token, self.line_num, self.char_pos);
                    self.all_tokens.push(token_obj);
                    char_index = char_index + 1;
                    self.char_pos = char_index;
                    *token = "".to_string();


                    
                } 
                // if continuous operators (){ 
                else if flag_cont_operator != 0 {
            
                    // ( already in, push )
                    token.push(self.file.chars().nth(c).unwrap());
                    let token_obj: Token = self.get_cur_token(token, self.line_num, self.char_pos);
                    self.all_tokens.push(token_obj);
                    flag_cont_operator = flag_cont_operator + 1;
                    *token = "".to_string();
                    char_index = char_index + 1;
                    self.char_pos = char_index;
                    

                   

                }



            }

            else if (self.file.chars().nth(c).unwrap()).is_alphanumeric() || self.file.chars().nth(c).unwrap().to_string() == "." || self.file.chars().nth(c).unwrap().to_string() == "_" {
                after_newline = 0;
                flag_cont_operator = 0;
                // first non whitespace char, e.g. _ _ _ _ float
                if flag_space == 0 && char_index == -1 {
                    char_index = char_index + 1;
                    self.char_pos = char_index;
                    token.push(self.file.chars().nth(c).unwrap());
                    char_index = char_index + 1;
                    flag_space = 1;
                }

                else {
                    token.push(self.file.chars().nth(c).unwrap());
                    char_index = char_index + 1;
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

