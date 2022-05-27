use crate::Token::Token;

let mut index : i32 = 0;

struct Parser {
    allToken : Vec<Token>,

}

impl Parser {
    fn lookAhead (&mut self, index:i32) -> Token{
        return allToken[index + 1];
    }

    fn Program (&mut self){
        while self.Declaration() == true {
            self.Declaration();
        }
        if self.MainDeclaration() == true {
            while self.FunctionDefinition() == true {
                self.FunctionDefinition();
            }
            return true;
        }
        return false;
    }

    fn Declaration (&mut self) -> bool {
        if self.DeclarationType() == true {
            if self.VariableDeclaration() == true || self.FunctionDeclaration() == true {
                return true;
            }
        }
        return false;
    }

    fn MainDeclaration (&mut self) -> bool {
        if self.allTokens[index].text == "void" {
            index = index + 1;
            if self.allTokens[index].text == "main" {
                index = index + 1;
                if self.allTokens[index].text == "(" {
                    index = index + 1;
                    if self.allTokens[index].text == ")" {
                        index = index + 1;
                        if self.Block() == true {
                            return true;
                        }
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
        if self.DataType() == true {
            if self.allTokens[index].token_type == TokenType::Identifier {
                index = index + 1;
                return true;
            }
        }
        return false;
    }

    fn VariableDeclaration (&mut self) -> bool {
        if self.allTokens[index].text == ";" {
            index = index + 1;
            return true;
        }

        if self.allTokens[index].text == "=" {
            index = index + 1;
            if self.Constant() == true {
                if self.allTokens[index].text == ";" {
                    index = index + 1;
                    return true;
                }
            }
        }
        return false;
    }

    fn FunctionDeclaration (&mut self) -> bool {
        if self.ParameterBlock() == true {
            if self.allTokens[index].text == ";" {
                index = index + 1;
                return true;
            }
        }
        return false;
    }

    fn Block(&mut self) -> bool {
        if self.allTokens[index].text == "{" {
            index = index + 1;
            while self.Declaration() == true {
                self.Declaration()
            }
            while self.Statement() == true {
                self.Statement()
            }
            while self.FunctionDeclaration() == true {
                self.FunctionDeclaration()
            }
            if self.allTokens[index].text == "}" {
                index = index + 1
                return true;
            } 
        }
        return false;
    }

    fn ParameterBlock (&mut self) -> bool {
        if self.allTokens[index].text == "(" {
            index = index + 1
            // [Parameter {, Parameter}]
            if self.Parameter() == true {
                // {, Parameter}
                while true {
                    if self.allTokens[index].text == "," {
                        index = index + 1;
                        if self.Parameter() == true {
                            continue;
                        }
                        return false;
                    }
                    break;
                }
            }
        }
        if self.allTokens[index].text == ")" {
            index = index + 1;
            return true;
        }
        return false;
    }

    fn DataType (&mut self) -> bool{
        if self.FloatType() == true || self.IntegerType() == true {
            return true;
        }
        return false;
    }

    fn Constant (&mut self) -> bool {
        if self.allTokens[index].token_type == TokenType::IntConstant || self.allTokens[index].token_type == TokenType::FloatConstant {
            index = index + 1;
            return true;
        }
        return false;
    }

    fn Statement (&mut self) -> bool {
        if self.Assignment() == true || self.WhileLoop() == true || self.IfStatement() == true || self.ReturnStatement() == true || self.Expression == true {
            if self.Expression() == true {
                if self.allTokens[index].text == ";" {
                    index = index + 1;
                    return true;
                }
                return false;
            }
            return true;
        }
        return false;
    }

    fn Parameter (&mut self) -> bool {
        if self.DataType() == true {
            if self.allTokens[index].token_type == TokenType::Identifier {
                index = index + 1;
                return true;
            }
        }
        return false;
    }

    fn Assignment (&mut self) -> bool {
        if self.allTokens[index].token_type == TokenType::Identifier {
            index = index + 1;
            if self.allTokens[index].text == "=" {
                index = index + 1;
                while true {
                    if self.allTokens[index].token_type == TokenType::Identifier {
                        index = index + 1;
                        if self.allTokens[index].text == "=" {
                            index = index + 1;
                        }
                        return false;
                    }
                }
                if self.Expression() == true {
                    if self.allTokens[index].text == ";" {
                        index = index + 1;
                        return true;
                    }
                }
            }
        }
        return false;
    }

    fn WhileLoop (&mut self) -> bool {
        if self.allTokens[index].text == "while" {
            index = index + 1;
            if self.allTokens[index].text == "(" {
                index = index + 1;
                if self.Expression() == true {
                    if self.allTokens[index].text == ")"{
                        index = index + 1;
                        if self.Block() == true {
                            return true;
                        }
                    }
                }
            }
        }
        return false;
    }

    fn IfStatement (&mut self, token:Token){
        if self.allTokens[index].text == "if" {
            index = index + 1;
            if self.allTokens[index].text == "(" {
                index = index + 1;
                if self.Expression() == true {
                    if self.allTokens[index].text == ")"{
                        index = index + 1;
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
        if self.allTokens[index].text == "return" {
            index = index + 1;
            if self.Expression() == true {
                if self.allTokens[index].text == ";" {
                    index = index + 1;
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
        if self.allTokens[index].text == "(" {
            index = index + 1;
            if self.Expression() == true {
                if self.allTokens[index].text == ")" {
                    index = index + 1;
                    return true;
                }   
            }
        }
        if self.Constant() == true {
            return true;
        }
        // Foo () or Foo (e, f, g, h) or    CHECK ABT Foo &
        if self.Identifier() == true {
            if self.allTokens[index].text == "(" {
                index = index + 1;
                if self.Expression() == true {
                    // {, Expression}
                    while true {
                        if self.allTokens[index].text == "," {
                            index = index + 1;
                            if self.Expression() == true {
                                continue;
                            }
                            return false;
                        }
                        break;
                    }
                    if self.allTokens[index].text == ")" {
                        index = index + 1;
                        return true;
                    }
                    return false;
                }
                if self.allTokens[index].text == ")" {
                    index = index + 1;
                    return true;
                }
                return false;
            }
            return true;
        }
        return false;
    }

    fn IntegerType (&mut self) -> bool {
        // char | short | int | long
        if self.allToken[index].text == "unsigned" {
            index = index + 1;
            if self.allToken[index].text == "char" || self.allToken[index].text == "short" || self.allToken[index].text == "int" || self.allToken[index].text == "long" {
                index = index + 1;
                return true;
            }
            return false;
        }
        if self.allToken[index].text == "char" || self.allToken[index].text == "short" || self.allToken[index].text == "int" || self.allToken[index].text == "long" {
            index = index + 1;
            return true;
        }
        return false;
    }

    fn FloatType(&mut self) -> bool{
        if self.allToken[index].text == "float" || self.allToken[index].text == "double" {
            index = index + 1;
            return true
        }
        return false
    }

    fn RelationOperator(&mut self) -> bool {
        if self.allToken[index].text == "==" || self.allToken[index].text == "<" || self.allToken[index].text == ">" || self.allToken[index].text == "<=" || self.allToken[index].text == ">=" || self.allToken[index].text == "!=" {
            index = index + 1;
            return true;
        }
        return false;
    }

    fn AddOperator(&mut self) -> bool {
        if self.allToken[index].text == "+" || self.allToken[index].text == "-" { // do we need to create a funciton that gets text
            index = index + 1;
            return true;
        }
        return false;
    }

    fn MultOperator (&mut self) -> bool {
        if self.allToken[index].text == "*" || self.allToken[index].text == "/" {
            index = index + 1;
            return true;
        }
        return false;
    }
}
