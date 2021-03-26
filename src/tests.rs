#[cfg(test)]
mod test {
    use crate::{
        lexer::{Lexer, TType, Token},
        parser::{Expr, Parser},
        Result,
    };

    #[test]
    fn lexing() -> Result<()> {
        let mut lexer = Lexer::new("()\"Foo Bar\"True False TrueFalse 4 9.1 lambda! λ match! def!");
        let tokens = lexer.proc_tokens()?;

        assert_eq!(
            tokens,
            vec![
            Token::new(TType::LParen, 1, 1),
            Token::new(TType::RParen, 2, 1),
            Token::new(TType::Str("Foo Bar".to_owned()), 11, 1),
            Token::new(TType::Bool(true), 15, 1),
            Token::new(TType::Bool(false), 21, 1),
            Token::new(TType::Ident("TrueFalse".to_owned()), 31, 1),
            Token::new(TType::Number(4), 33, 1),
            Token::new(TType::Float(9.1), 37, 1),
            Token::new(TType::Lambda, 45, 1),
            Token::new(TType::Lambda, 53, 1),
            Token::new(TType::Match, 60, 1),
            Token::new(TType::Def, 65, 1),
            ]
            );

        Ok(())
    }

    mod parsing {
        use super::*;

        #[test]
        fn currying() -> Result<()> {
            let code = "(λ (x y z) e)(foo a b c d)";
            let tokens = Lexer::new(code).proc_tokens()?;
            let expressions = Parser::new(tokens).parse()?;

            assert_eq!(format!("{:?}", expressions), r#"[Lambda("x", Lambda("y", Lambda("z", Var("e")))), Call(Call(Call(Call(Var("foo"), Var("a")), Var("b")), Var("c")), Var("d"))]"#.to_string());


            Ok(())
        }

        #[test]
        fn global_parsing() -> Result<()> {
            let code = "(defn! factorial (n) (if (< n 1) 1 (* n (factorial (- n 1)))))";
            let tokens = Lexer::new(code).proc_tokens()?;
            let expressions = Parser::new(tokens).parse()?;

            assert_eq!(format!("{:?}", expressions), r#"[Call(Call(Call(Var("defn!"), Var("factorial")), Var("n")), Call(Call(Call(Var("if"), Call(Call(Var("<"), Var("n")), Integer(1))), Integer(1)), Call(Call(Var("*"), Var("n")), Call(Var("factorial"), Call(Call(Var("-"), Var("n")), Integer(1))))))]"#.to_string());

            Ok(())

        }

    }
}
