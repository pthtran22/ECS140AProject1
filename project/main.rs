// use crate::Cstream::CStream;
// use crate::Token::Token;
// use crate::Scanner::Scanner;

mod Token;
mod Cstream;
mod Scanner;
mod Parser;

// custom_error!{MyError
//     General{n: bool} = "Function return {n}.",
// }


fn main (){
    let file = Cstream::run();
    let mut ex = Scanner::Scanner::new(file.to_string());
    let all_tokens = ex.get_all_tokens();

    for i in 0..all_tokens.len()-1 {
        // check ==, >=, <=, !=
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
    let result = parser.Program();
    // if result == true {
    //     println!("Input program is syntactacilly correct.");
    // }
    // match result
    // {
    //     Ok(n) => println!("Input program is syntactacilly correct."),
    //     Err(n) => eprintln!("Syntax error at character position"),
    // }

    // for i in 0..all_tokens.len() {
    //     println!("--------------------------");
    //     println!("token: {:?}", all_tokens[i].text);
    //     println!("token_type: {:?}", all_tokens[i].token_type);
    //     println!("line_num: {:?}", all_tokens[i].line_num);
    //     println!("char_pos: {:?}", all_tokens[i].char_pos);
        
    // }
    // println!("--------------------------");
    

}





// match result
// {
//     Ok(n) => println!("Input program is syntactacilly correct."),
//     Err(n) => eprintln!("Error at Line {} Character {}. The syntax should be: {}.", line_num, char_pos, grammar_rule),
// }
