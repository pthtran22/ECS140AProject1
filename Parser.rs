use crate::Token::Token;

struct Parser {
    allToken : Vec<Token>,
}

impl Parser {

    fn Program (&mut self, token:Token){

    }

    fn Declaration (&mut self, token:Token){

    }

    fn MainDeclaration (&mut self, token:Token){

    }

    fn FunctionDefinition (&mut self, token:Token){

    }

    fn DeclarationType (&mut self, token:Token){

    }

    fn VariableDeclaration (&mut self, token:Token){

    }

    fn FunctionDeclaration (&mut self, token:Token){

    }

    fn Block (&mut self, token:Token){

    }

    fn ParameterBlock (&mut self, token:Token){

    }

    fn DataType (&mut self, token:Token){

    }

    fn Constant (&mut self, token:Token){

    }

    fn Statement (&mut self, token:Token){

    }

    fn Parameter (&mut self, token:Token){

    }

    fn Assignment (&mut self, token:Token){

    }

    fn WhileLoop (&mut self, token:Token){

    }

    fn IfStatement (&mut self, token:Token){

    }

    fn ReturnStatement (&mut self, token:Token){

    }

    fn Expression (&mut self, token:Token){

    }

    fn SimpleExpression (&mut self, token:Token){

    }

    fn Term(&mut self, token:Token){

    }

    fn Factor(&mut self, token:Token){
        
    }

    // terminal
    fn IntegerType(&mut self, token:Token){
        // TO DO
        // char | short | int | long
        if token.text.length() == 1 {
            i
        }
         // unsigned (char | short | int | long)
        if token.text.length() == 2 {
            token.text == "unsigned"

        }
    }

    fn FloatType(&mut self, token:Token) {
        
    }

    fn RelationOperator(&mut self, token:Token) {
        if token.text == "==" || token.text == "<" || token.text == ">" || token.text == "<=" || token.text == ">=" || token.text == "!=" {
            return "RelationOperator";
        }
    }

    fn AddOperator(&mut self, token:Token) {
        if token.text == "+" || token.text == "-" { // do we need to create a funciton that gets text
            return "AddOperator";
        }
    }

    fn MultOperator (&mut self, token:Token){
        if token.text == "*" || token.text == "/" {
            // not done
            Term(token);
        }
    }

    fn look


}