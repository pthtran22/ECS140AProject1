// use crate::Cstream::CStream;
// use crate::Token::Token;
// use crate::Scanner::Scanner;

mod Token;
mod Cstream;
mod Scanner;
mod Parser;
//use Scanner::Scanner;
// pub use new

fn main (){
    let file = Cstream::run();
    let mut ex = Scanner::Scanner::new(file.to_string());
    let all_tokens = ex.get_all_tokens();
        
    // check if there's ==, >=, <=, !=
    for i in 0..all_tokens.len()-1 {
        if i > 0 && all_tokens[i].text == "=" {
            if all_tokens[i-1].text == "=" {
                all_tokens.remove(i);
                all_tokens[i-1].text = "==".to_string();
            }
            if all_tokens[i-1].text == ">" {
                all_tokens.remove(i);
                all_tokens[i-1].text = ">=".to_string();
            }
            if all_tokens[i-1].text == "<" {
                all_tokens.remove(i);
                all_tokens[i-1].text = "<=".to_string();
            }
            if all_tokens[i-1].text == "!" {
                all_tokens.remove(i);
                all_tokens[i-1].text = "!=".to_string();
            }
        }
    }
    
    
    let mut parser = Parser::Parser::new((&all_tokens).to_vec());
    let boolean = parser.Program();
    // for i in 0..all_tokens.len() {
    //     println!("--------------------------");
    //     println!("token: {:?}", all_tokens[i].text);
    //     println!("token_type: {:?}", all_tokens[i].token_type);
    //     println!("line_num: {:?}", all_tokens[i].line_num);
    //     println!("char_pos: {:?}", all_tokens[i].char_pos);
        
    // }
    // println!("--------------------------");
    

}

