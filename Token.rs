use crate::Cstream::CStream;

enum TokenType {
    IntConstant,
    FloatConstant,
    Keyword,
    Operator,
    Identifier,
    Invalid,
}

let keywords: vec!["unsigned", "char", "short", "int", "long", "float", "double", "while", "if", "return", "void", "main"]
 
pub struct Token {
    text: String,
    token_type: TokenType,
    line_num: i32,
    char_pos: i32,
    overall_pos: i32,
}

impl Token {
    fn new(t: &str, tokenType: TokenType) -> Token { // reads in file content
        Token {
            text: t.to_string(),
            token_type: tokenType, // what should its initialize be??
            line_num: -1, // will change to 0 at first line?
            char_pos: -1,
            overall_pos: -1,
        }
    }

    fn get_next_char(&mut self) -> Option<char> {
        
		let cur = self.text.chars().nth((self.overall_pos) as usize);          // get_content() from Cstream will give me my file
		match cur {
			None => {       // NULL ??
				self.char_pos += 1;
				self.line_num += 1;
			},
			Some(x) => {    // don't understand
				if x == '\n' {                      // end of line
					self.char_pos = 0;              // restart to first char index
					self.line_num += 1;             // go to next line
				} else {
					self.char_pos += 1;             // increment char pos
					if self.overall_pos == -1 {     // WHAT DOES OVERALL_POS DO???
						self.line_num += 1;
					}
				}
			}
		}
		self.overall_pos += 1;
		self.text.chars().nth((self.overall_pos) as usize)  // this returns my character
	}

    fn check_TokenType(mut self, text) -> TokenType {
        // Keyword
        if keyword.contains(text) {
            return TokenType::Keyword;
        }

        // Identifier
        if text.chars().nth(0).unwrap() == '_' || isalpha(text.chars().nth(0).unwrap()) {
            if text.to_string.len() == 1 {
                return TokenType::Identifier;
            }
            for c in 1..text.to_string().len() {
                if text.chars().nth(c).unwrap() != '_' && !isalphanumeric(text.chars().nth(0).unwrap()) {
                    return TokenType::Invalid;
                }
            }
            return TokenType::Identifier;
        }


        // IntConstant
        let dotCount = 0;
        // checks if the first two characters are (-) digit
        if text.chars().nth(0).unwrap() == '-' || isnumeric(text.chars().nth(0).unwrap()) {
            if text.to_string.len() == 1 { // only digit
                if isnumeric(text.chars().nth(0).unwrap()) {
                    return TokenType::IntConstant;
                }
                return TokenType::Invalid;
            }
            // checks if the rest of the characters are digit.{digit}
            for c in 1..text.to_string().len() {
                if !isnumeric(text.chars().nth(c).unwrap()) {
                    if text.chars().nth(c).unwrap() == '.' && c !=  text.to_string.len() - 1{
                        dotCount += 1;
                    }
                    return TokenType::Invalid;
                }
            }
            if dotCount == 0 {
                return TokenType::IntConstant;
            }
            if dotCount == 1 {
                return TokenType::Float;
            }
            else {
                return TokenType::Invalid;
            }
        }
        }
    }
}

pub fn run() {
    // Token::new("hi", TokenType::Keyword);
    // println!("hi");
    // Cstream::run();
}