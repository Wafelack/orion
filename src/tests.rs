#[cfg(test)]
mod test {
    use crate::{Result, parser::{Parser, NType, Node}, lexer::{Lexer, TType, Token}};

#[test]
    fn lexing() -> Result<()> {
        let mut lexer = Lexer::new("()\"Foo Bar\"True False TrueFalse 4 9.1");
        let tokens = lexer.proc_tokens()?;

        assert_eq!(tokens,
                   vec![Token::new(TType::LParen, 1, 1), Token::new(TType::RParen, 2, 1), Token::new(TType::Str("Foo Bar".to_owned()), 11, 1), Token::new(TType::Bool(true), 15, 1), Token::new(TType::Bool(false), 21, 1), Token::new(TType::Ident("TrueFalse".to_owned()), 31, 1), Token::new(TType::Number(4), 33, 1), Token::new(TType::Float(9.1), 37, 1)]);

        Ok(())
    }

    #[test]
    fn parsing() -> Result<()> {
        let code = "(let foo (lambda (x y) (+ x y)))(while x y)";
        let tokens = Lexer::new(code).proc_tokens()?;
        let node = Parser::new(tokens).parse()?;

        let mut master = Node::new(NType::Ident("begin".to_owned()));
        let mut let_func = Node::new(NType::Ident("let".to_owned()));
        let mut lambda_func = Node::new(NType::Ident("lambda".to_string()));
        let mut x = Node::new(NType::Ident("x".to_string()));
        x.add_child(Node::new(NType::Ident("y".to_string())));
        let mut plus = Node::new(NType::Ident("+".to_string()));
        plus.add_child(Node::new(NType::Ident("x".to_string())));
        plus.add_child(Node::new(NType::Ident("y".to_string())));

        lambda_func.add_child(x);
        lambda_func.add_child(plus);
        let_func.add_child(Node::new(NType::Ident("foo".to_owned())));
        let_func.add_child(lambda_func);

        let mut while_func = Node::new(NType::Ident("while".to_owned()));
        while_func.add_child(Node::new(NType::Ident("x".to_string())));
        while_func.add_child(Node::new(NType::Ident("y".to_string())));
        
        master.add_child(let_func);
        master.add_child(while_func);

        assert_eq!(node, master);

        Ok(())
    }

}
