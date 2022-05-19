// use crate::Cstream::CStream;
// use crate::Token::Token;
// use crate::Scanner::Scanner;

mod Token;
mod Cstream;
mod Scanner;
// use Scanner::new;
// pub use new

fn main (){
    //let file = CStream::run();
    let file = Cstream::run();
    Scanner::run(&file);

}

