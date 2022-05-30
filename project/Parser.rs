extern crate custom_error;
use custom_error::custom_error;

// #[derive(PartialEq)]
// custom_error!{MyError
//     General{n: i32, u:i32} = "Error at Line {n} Character {u}. The syntax should be: ",
    
// }

// match Result
// {
//     Ok(n) => println!("Input program is syntactacilly correct."),
//     Err(n) => eprintln!("Error at Line {} Character {}. The syntax should be: {}.", n, u) //, grammar_rule),
// }

use crate::Token::Token;
use crate::Token::TokenType;

// static index : i32 = 0;


pub struct Parser {
    allToken : Vec<Token>,
    index:i32,
}

impl Parser {
    pub fn new (all_tokens: Vec<Token>) -> Parser {
        Parser {
            allToken: all_tokens,
            index: 0,
        }
    }

    pub fn Program (&mut self) ->  bool {
        for i in 0..self.allToken.len() {
            println!("{}", self.allToken[i].text);
        }
        while self.Declaration() == true {
            self.Declaration();
        }
        if self.MainDeclaration() == true {
            while self.FunctionDefinition() == true {
                self.FunctionDefinition();
            }
            println!("true");
            return true;
        }
        println!("false");
        return false;
    }

    fn Declaration (&mut self) -> bool {
        if self.DeclarationType() == true {
            if self.VariableDeclaration() == true || self.FunctionDeclaration() == true {
                // return true;
                return true;
            }
        }
        // return false;
        return false;
    }

    fn MainDeclaration (&mut self) -> bool {
        let index_usize: usize = self.index as usize;
        if self.allToken[index_usize].text == "void" {
            self.index = self.index + 1;
            let index_usize: usize = self.index as usize;
            if self.allToken[index_usize].text == "main" {
                self.index = self.index + 1;
                let index_usize: usize = self.index as usize;
                if self.allToken[index_usize].text == "(" {
                    self.index = self.index + 1;
                    let index_usize: usize = self.index as usize;
                    if self.allToken[index_usize].text == ")" {
                        self.index = self.index + 1;
                        let index_usize: usize = self.index as usize;
                        if self.Block() == true {
                            return true;
                        }
                        // return custom_error(self.line_num, self.char_pos);
                    }
                }
            }
        }
        return false;
    }

    fn FunctionDefinition (&mut self) -> bool {
        if self.DeclarationType() == true {
            if self.ParameterBlock() == true {
                if self.Block() == true {
                    return true;
                }
            }
        }
        return false;
    }

    fn DeclarationType (&mut self) -> bool {
        let index_usize: usize = self.index as usize;
        if self.DataType() == true {
            let index_usize: usize = self.index as usize;
            if self.allToken[index_usize].token_type == TokenType::Identifier {
                self.index = self.index + 1;
                return true;
            }
        }
        return false;
    }

    fn VariableDeclaration (&mut self) -> bool {
        let index_usize: usize = self.index as usize;
        if self.allToken[index_usize].text == ";" {
            self.index = self.index + 1;
            return true;
        }

        if self.allToken[index_usize].text == "=" {
            let index_usize: usize = self.index as usize;
            self.index = self.index + 1;
            if self.Constant() == true {
                let index_usize: usize = self.index as usize;
                if self.allToken[index_usize].text == ";" {
                    self.index = self.index + 1;
                    return true;
                }
            }
        }
        return false;
    }

    fn FunctionDeclaration (&mut self) -> bool {
        let index_usize: usize = self.index as usize;
        if self.ParameterBlock() == true {
            let index_usize: usize = self.index as usize;
            if self.allToken[index_usize].text == ";" {
                self.index = self.index + 1;
                return true;
            }
        }
        return false;
    }

    fn Block(&mut self) -> bool {
        let index_usize: usize = self.index as usize;
        if self.allToken[index_usize].text == "{" {
            self.index = self.index + 1;
            while self.Declaration() == true {
                self.Declaration();
            }
            let index_usize: usize = self.index as usize;
            while self.Statement() == true {
                self.Statement();
            }
            let index_usize: usize = self.index as usize;
            while self.FunctionDeclaration() == true {
                self.FunctionDeclaration();
            }
            let index_usize: usize = self.index as usize;
            if self.allToken[index_usize].text == "}" {
                println!("end of block {}", self.allToken[index_usize].text);
                self.index = self.index + 1;
                return true;
            } 
        }
        return false;
    }

    fn ParameterBlock (&mut self) -> bool {
        let index_usize: usize = self.index as usize;
        if self.allToken[index_usize].text == "(" {
            self.index = self.index + 1;
            
            // [Parameter {, Parameter}]
            if self.Parameter() == true {
                // {, Parameter}
                while true {
                    let index_usize: usize = self.index as usize;
                    if self.allToken[index_usize].text == "," {
                        self.index = self.index + 1;
                        if self.Parameter() == true {
                            continue;
                        }
                        return false;
                    }
                    break;
                }
            }
        }
        let index_usize: usize = self.index as usize;
        if self.allToken[index_usize].text == ")" {
            self.index = self.index + 1;
            return true;
        }
        return false;
    }

    fn DataType (&mut self) -> bool {
        if self.FloatType() == true || self.IntegerType() == true {
            return true;
        }
        return false;
    }

    fn Constant (&mut self) -> bool {
        let index_usize: usize = self.index as usize;
        if self.allToken[index_usize].token_type == TokenType::IntConstant || self.allToken[index_usize].token_type == TokenType::FloatConstant {
            self.index = self.index + 1;
            return true;
        }
        return false;
    }

    fn Statement (&mut self) -> bool {
        let index_usize: usize = self.index as usize;
        if self.Assignment() == true || self.WhileLoop() == true || self.IfStatement() == true || self.ReturnStatement() == true || self.Expression() == true {
            let index_usize: usize = self.index as usize;
            if self.Expression() == true {
                let index_usize: usize = self.index as usize;
                if self.allToken[index_usize].text == ";" {
                    self.index = self.index + 1;
                    return true;
                }
            }
            return true;
        }
        return false;
    }

    fn Parameter (&mut self) -> bool {
        let index_usize: usize = self.index as usize;
        if self.DataType() == true {
            let index_usize: usize = self.index as usize;
            if self.allToken[index_usize].token_type == TokenType::Identifier {
                self.index = self.index + 1;
                return true;
            }
        }
        return false;
    }

    fn Assignment (&mut self) -> bool {
        let mut index_usize: usize = self.index as usize;
        if self.allToken[index_usize].token_type == TokenType::Identifier {
            self.index = self.index + 1;
            index_usize = self.index as usize;
            if self.allToken[index_usize].text == "=" {
                self.index = self.index + 1;
                index_usize = self.index as usize;
                while true {
                    index_usize = self.index as usize;
                    if self.allToken[index_usize].token_type == TokenType::Identifier {
                        if self.allToken[index_usize+1].text != "=" {
                            break;
                        }
                        self.index = self.index + 1;
                        index_usize = self.index as usize;
                        if self.allToken[index_usize].text == "=" {
                            self.index = self.index + 1;
                        }
                        return false;
                    }
                    break;
                }
                if self.Expression() == true {
                    let index_usize: usize = self.index as usize;
                    if self.allToken[index_usize].text == ";" {
                        self.index = self.index + 1;
                        return true;
                    }
                }
                return false;
            }   
        }
        return false;
    }

    fn WhileLoop (&mut self) -> bool {
        let index_usize: usize = self.index as usize;
        if self.allToken[index_usize].text == "while" {
            self.index = self.index + 1;
            let index_usize: usize = self.index as usize;
            if self.allToken[index_usize].text == "(" {
                self.index = self.index + 1;
                let index_usize: usize = self.index as usize;
                if self.Expression() == true {
                    if self.allToken[index_usize].text == ")"{
                        self.index = self.index + 1;
                        let index_usize: usize = self.index as usize;
                        if self.Block() == true {
                            return true;
                        }
                    }
                }
            }
        }
        return false;
    }

    fn IfStatement (&mut self) -> bool {
        let index_usize: usize = self.index as usize;
        if self.allToken[index_usize].text == "if" {
            self.index = self.index + 1;
            let index_usize: usize = self.index as usize;
            if self.allToken[index_usize].text == "(" {
                self.index = self.index + 1;
                let index_usize: usize = self.index as usize;
                if self.Expression() == true {
                    if self.allToken[index_usize].text == ")"{
                        self.index = self.index + 1;
                        let index_usize: usize = self.index as usize;
                        if self.Block() == true {
                            return true;
                        }
                    }
                }
            }
        }
        return false;
    }

    fn ReturnStatement (&mut self) -> bool {
        let index_usize: usize = self.index as usize;
        if self.allToken[index_usize].text == "return" {
            self.index = self.index + 1;
            let index_usize: usize = self.index as usize;
            if self.Expression() == true {
                let index_usize: usize = self.index as usize;
                if self.allToken[index_usize].text == ";" {
                    self.index = self.index + 1;
                    return true;
                }
            }
        }
        return false;
    }

    fn Expression (&mut self) -> bool {
        if self.SimpleExpression() == true {
            if self.RelationOperator() == true {
                if self.SimpleExpression() == true {
                    return true;
                }
                return false;
            }
            return true;
        }
        return false;
    }

    fn SimpleExpression (&mut self) -> bool {
        if self.Term() == true {
            while true {
                if self.AddOperator() == true {
                    if self.Term() == true {
                        continue;
                    }
                    return false;
                }
                break;
            }
            return true;
        }
        return false;
    }

    fn Term (&mut self) -> bool {
        if self.Factor() == true {
            while true {
                if self.MultOperator() == true {
                    if self.Factor() == true {
                        continue;
                    }
                    return false;
                }
                break;
            }
            return true;
        }
        return false;
    }

    fn Factor (&mut self) -> bool {
        let index_usize: usize = self.index as usize;
        if self.allToken[index_usize].text == "(" {
            self.index = self.index + 1;
            let index_usize: usize = self.index as usize;
            if self.Expression() == true {
                let index_usize: usize = self.index as usize;
                if self.allToken[index_usize].text == ")" {
                    self.index = self.index + 1;
                    return true;
                }   
            }
        }
        if self.Constant() == true {
            return true;
        }
        // Foo () or Foo (e, f, g, h) or    CHECK ABT Foo &
        if self.allToken[index_usize].token_type == TokenType::Identifier {
            self.index = self.index + 1;
            let index_usize: usize = self.index as usize;
            if self.allToken[index_usize].text == "(" {
                self.index = self.index + 1;
                let index_usize: usize = self.index as usize;
                if self.Expression() == true {
                    // {, Expression}
                    while true {
                        if self.allToken[index_usize].text == "," {
                            self.index = self.index + 1;
                            let index_usize: usize = self.index as usize;
                            if self.Expression() == true {
                                continue;
                            }
                            return false;
                        }
                        break;
                    }
                    if self.allToken[index_usize].text == ")" {
                        self.index = self.index + 1;
                        return true;
                    }
                    return false;
                }
                if self.allToken[index_usize].text == ")" {
                    self.index = self.index + 1;
                    return true;
                }
                return false;
            }
            return true;
        }
        return false;
    }

    fn IntegerType (&mut self) -> bool {
        let index_usize: usize = self.index as usize;
        // char | short | int | long
        if self.allToken[index_usize].text == "unsigned" {
            self.index = self.index + 1;
            let index_usize: usize = self.index as usize;
            if self.allToken[index_usize].text == "char" || self.allToken[index_usize].text == "short" || self.allToken[index_usize].text == "int" || self.allToken[index_usize].text == "long" {
                self.index = self.index + 1;
                return true;
            }
            return false;
        }
        let index_usize: usize = self.index as usize;
        if self.allToken[index_usize].text == "char" || self.allToken[index_usize].text == "short" || self.allToken[index_usize].text == "int" || self.allToken[index_usize].text == "long" {
            self.index = self.index + 1;
            return true;
        }
        return false;
    }

    fn FloatType(&mut self) -> bool {
        
        let index_usize: usize = self.index as usize;
        println!("index num {}", self.index);
        println!("index num {}", self.allToken[index_usize].text);
        if self.allToken[index_usize].text == "float" || self.allToken[index_usize].text == "double" {
            
            self.index = self.index + 1;
            return true;
        }
        return false;
    }

    fn RelationOperator(&mut self) -> bool {
        let index_usize: usize = self.index as usize;
        if self.allToken[index_usize].text == "==" || self.allToken[index_usize].text == "<" || self.allToken[index_usize].text == ">" || self.allToken[index_usize].text == "<=" || self.allToken[index_usize].text == ">=" || self.allToken[index_usize].text == "!=" {
            self.index = self.index + 1;
            return true;
        }
        return false;
    }

    fn AddOperator(&mut self) -> bool {
        let index_usize: usize = self.index as usize;
        if self.allToken[index_usize].text == "+" || self.allToken[index_usize].text == "-" { // do we need to create a funciton that gets text
            self.index = self.index + 1;
            return true;
        }
        return false;
    }

    fn MultOperator (&mut self) -> bool {
        let index_usize: usize = self.index as usize;
        if self.allToken[index_usize].text == "*" || self.allToken[index_usize].text == "/" {
            self.index = self.index + 1;
            return true;
        }
        return false;
    }
}