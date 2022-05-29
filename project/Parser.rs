
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
    // fn lookAhead (&mut self, index:i32) -> Token{
    //     return allToken[index + 1];
    // }

    pub fn Program (&mut self) ->  bool {
        for i in 0..self.allToken.len() {
            println!("{}", self.allToken[i].text);
        }
        println!("\nInside program()");
        while self.Declaration() == true {
            self.Declaration();
        }
        if self.MainDeclaration() == true {
            println!("main dec pased");
            while self.FunctionDefinition() == true {
                self.FunctionDefinition();
            }
            return true;
        }
        println!("main dec failed");
        return false;
    }

    fn Declaration (&mut self) -> bool {
        println!("\nInside declaration()");
        println!("Index in Declaration Before: {}", self.index);
        if self.DeclarationType() == true {
            if self.VariableDeclaration() == true || self.FunctionDeclaration() == true {
                return true;
            }
        }
        println!("Index in Declaration After: {}", self.index);
        return false;
    }

    fn MainDeclaration (&mut self) -> bool {
        println!("\nInside mainDeclaration()");
        let index_usize: usize = self.index as usize;
        println!("{}", self.allToken[index_usize].text);
        println!("Index in Main Before: {}", self.index);
        if self.allToken[index_usize].text == "void" {
            println!("passed void");
            self.index = self.index + 1;
            let index_usize: usize = self.index as usize;
            if self.allToken[index_usize].text == "main" {
                println!("passed main");
                self.index = self.index + 1;
                let index_usize: usize = self.index as usize;
                if self.allToken[index_usize].text == "(" {
                    println!("passed (");
                    self.index = self.index + 1;
                    let index_usize: usize = self.index as usize;
                    if self.allToken[index_usize].text == ")" {
                        println!("passed )");
                        self.index = self.index + 1;
                        let index_usize: usize = self.index as usize;
                        if self.Block() == true {
                            println!("passed Block()");
                            return true;
                        }
                        println!("failed Block()");
                    }
                    println!("failed )");
                }
                println!("failed (");
            }
            println!("failed main");
        }
        println!("failed void");
        println!("Index in Main After: {}", self.index);
        return false;
    }

    fn FunctionDefinition (&mut self) -> bool {
        println!("\nInside functionDefinition()");
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
        println!("\nInside DeclarationType()");
        println!("Index in DeclarationType Before: {}", self.index);
        let index_usize: usize = self.index as usize;
        if self.DataType() == true {
            let index_usize: usize = self.index as usize;
            if self.allToken[index_usize].token_type == TokenType::Identifier {
                self.index = self.index + 1;
                
                println!("Index in DeclarationType After: {}", self.index);
                return true;
            }
        }
        println!("Index in DeclarationType After: {}", self.index);
        return false;
    }

    fn VariableDeclaration (&mut self) -> bool {
        println!("\nInside VariableDeclaration()");
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
        println!("\nInside FunctionDeclaration()");
        let index_usize: usize = self.index as usize;
        if self.ParameterBlock() == true {
            let index_usize: usize = self.index as usize;
            if self.allToken[index_usize].text == ";" {
                self.index = self.index + 1;
                return true;
            }
            println!("Failed ParameterBlock()")
        }
        return false;
    }

    fn Block(&mut self) -> bool {
        println!("\nInside Block()");
        let index_usize: usize = self.index as usize;
        if self.allToken[index_usize].text == "{" {
            self.index = self.index + 1;
            while self.Declaration() == true {
                println!("Declaration() true");
                self.Declaration();
            }
            while self.Statement() == true {
                println!("Statement() true");
                self.Statement();
            }
            while self.FunctionDeclaration() == true {
                println!("FunctionDeclaration() true");
                self.FunctionDeclaration();
            }
            if self.allToken[index_usize].text == "}" {
                self.index = self.index + 1;
                return true;
            } 
        }
        return false;
    }

    fn ParameterBlock (&mut self) -> bool {
        println!("\nInside ParameterBlock()");
        let index_usize: usize = self.index as usize;
        println!("{}", self.allToken[index_usize].text);
        println!("{}", self.allToken[index_usize+1].text);
        if self.allToken[index_usize].text == "(" {
            self.index = self.index + 1;
            
            // [Parameter {, Parameter}]
            if self.Parameter() == true {
                println!("\nInside Parameter() true");
                // {, Parameter}
                while true {
                    println!("\nInside Parameter optional()");
                    let index_usize: usize = self.index as usize;
                    if self.allToken[index_usize].text == "," {
                        println!("\nInside Parameter() para ,");
                        self.index = self.index + 1;
                        if self.Parameter() == true {
                            println!("\nInside Parameter() para, para");
                            continue;
                        }
                        println!("\nInside Parameter() para, but no para");
                        return false;
                        
                    }
                    println!("\nInside Parameter() para");
                    break;
                }
            }
        }
        println!("not match (");
        let index_usize: usize = self.index as usize;
        if self.allToken[index_usize].text == ")" {
            self.index = self.index + 1;
            return true;
        }
        return false;
    }

    fn DataType (&mut self) -> bool{
        println!("\nInside DataType()");
        println!("Index in DataType Before: {}", self.index);
        if self.FloatType() == true || self.IntegerType() == true {
            return true;
        }
        println!("Index in DataType After: {}", self.index);
        return false;
    }

    fn Constant (&mut self) -> bool {
        println!("\nInside Constant()");
        let index_usize: usize = self.index as usize;
        if self.allToken[index_usize].token_type == TokenType::IntConstant || self.allToken[index_usize].token_type == TokenType::FloatConstant {
            self.index = self.index + 1;
            return true;
        }
        return false;
    }

    fn Statement (&mut self) -> bool {
        println!("\nsInside Statement()");
        let index_usize: usize = self.index as usize;
        if self.Assignment() == true || self.WhileLoop() == true || self.IfStatement() == true || self.ReturnStatement() == true || self.Expression() == true {
            let index_usize: usize = self.index as usize;
            if self.Expression() == true {
                let index_usize: usize = self.index as usize;
                if self.allToken[index_usize].text == ";" {
                    self.index = self.index + 1;
                    return true;
                }
                return false;
            }
            return true;
        }
        return false;
    }

    fn Parameter (&mut self) -> bool {
        println!("\nInside Parameter()");
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
        println!("\nInside Assignment()");
        let index_usize: usize = self.index as usize;
        if self.allToken[index_usize].token_type == TokenType::Identifier {
            self.index = self.index + 1;
            let index_usize: usize = self.index as usize;
            if self.allToken[index_usize].text == "=" {
                self.index = self.index + 1;
                while true {
                    let index_usize: usize = self.index as usize;
                    if self.allToken[index_usize].token_type == TokenType::Identifier {
                        self.index = self.index + 1;
                        let index_usize: usize = self.index as usize;
                        if self.allToken[index_usize].text == "=" {
                            self.index = self.index + 1;
                        }
                        return false;
                    }
                }
                if self.Expression() == true {
                    if self.allToken[index_usize].text == ";" {
                        self.index = self.index + 1;
                        return true;
                    }
                }
            }
        }
        return false;
    }

    fn WhileLoop (&mut self) -> bool {
        println!("\nInside WhileLoop()");
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
        println!("\nInside IfStatement()");
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
        println!("\nInside ReturnStatement()");
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
        println!("\nInside Expression()");
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
        println!("\nInside SimpleExpression()");
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
        println!("\nInside Term()");
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
        println!("\nInside Factor()");
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
        println!("\nInside IntegerType()");
        println!("Index in IndexType Before: {}", self.index);
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
            println!("Index in IndexType After: {}", self.index);
            return true;
        }
        return false;
    }

    fn FloatType(&mut self) -> bool{
        println!("\nInside FloatType()");
        println!("Index in FloatType Before: {}", self.index);
        let index_usize: usize = self.index as usize;
        if self.allToken[index_usize].text == "float" || self.allToken[index_usize].text == "double" {
            self.index = self.index + 1;
            println!("Index in FloatType After: {}", self.index);
            return true
        }
        println!("Index in FloatType After: {}", self.index);
        return false;
    }

    fn RelationOperator(&mut self) -> bool {
        println!("\nInside RelationOperator()");
        let index_usize: usize = self.index as usize;
        if self.allToken[index_usize].text == "==" || self.allToken[index_usize].text == "<" || self.allToken[index_usize].text == ">" || self.allToken[index_usize].text == "<=" || self.allToken[index_usize].text == ">=" || self.allToken[index_usize].text == "!=" {
            self.index = self.index + 1;
            return true;
        }
        return false;
    }

    fn AddOperator(&mut self) -> bool {
        println!("\nInside AddOperator()");
        let index_usize: usize = self.index as usize;
        if self.allToken[index_usize].text == "+" || self.allToken[index_usize].text == "-" { // do we need to create a funciton that gets text
            self.index = self.index + 1;
            return true;
        }
        return false;
    }

    fn MultOperator (&mut self) -> bool {
        println!("\nInside MultOperator()");
        let index_usize: usize = self.index as usize;
        if self.allToken[index_usize].text == "*" || self.allToken[index_usize].text == "/" {
            self.index = self.index + 1;
            return true;
        }
        return false;
    }
}
