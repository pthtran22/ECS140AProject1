use crate::Cstream::CStream;

enum TokenType {
    IntConstant,
    FloatConstant,
    Keyword,
    Operator,
    Identifier,
    Invalid,
}

struct Token {
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
		let cur = CStream::get_content().chars().nth((self.overall_pos) as usize);          // get_content() from Cstream will give me my file
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
		CStream::get_content().chars().nth((self.overall_pos) as usize)  // this returns my character
	}
}

pub fn run() {
    println!("hi");
    // Cstream::run();
}