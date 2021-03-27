#[cfg(test)]
mod test {
    use crate::{
        interpreter::{Interpreter, Value},
        lexer::{Lexer, TType, Token},
        parser::{Expr, Parser},
        Result,
    };

    #[test]
    fn lexing() -> Result<()> {
        let mut lexer = Lexer::new("()\"Foo Bar\"TrueFalse 4 9.1 lambda λ def");
        let tokens = lexer.proc_tokens()?;

        assert_eq!(
            tokens,
            vec![
                Token::new(TType::LParen, 1, 1),
                Token::new(TType::RParen, 2, 1),
                Token::new(TType::Str("Foo Bar".to_owned()), 11, 1),
                Token::new(TType::Ident("TrueFalse".to_owned()), 20, 1),
                Token::new(TType::Number(4), 22, 1),
                Token::new(TType::Float(9.1), 26, 1),
                Token::new(TType::Lambda, 33, 1),
                Token::new(TType::Lambda, 40, 1),
                Token::new(TType::Def, 44, 1),
            ]
        );

        Ok(())
    }

    mod interpreting {
        use super::*;

        #[test]
        fn declaration() -> Result<()> {
            let code = "(def foo 99)(def bar foo)";
            let tokens = Lexer::new(code).proc_tokens()?;
            let expressions = Parser::new(tokens).parse()?;
            let scopes = Interpreter::new(expressions).interpret()?;
            assert_eq!(scopes[0]["foo"], Value::Integer(99));
            assert_eq!(scopes[0]["bar"], scopes[0]["foo"]);

            Ok(())
        }
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
            let code = "(Just 9)(defn factorial (n) (if (< n 1) 1 (* n (factorial (- n 1)))))";
            let tokens = Lexer::new(code).proc_tokens()?;
            let expressions = Parser::new(tokens).parse()?;
            assert_eq!(format!("{:?}", expressions), r#"[Constr("Just", Integer(9)), Call(Call(Call(Var("defn"), Var("factorial")), Var("n")), Call(Call(Call(Var("if"), Call(Call(Var("<"), Var("n")), Integer(1))), Integer(1)), Call(Call(Var("*"), Var("n")), Call(Var("factorial"), Call(Call(Var("-"), Var("n")), Integer(1))))))]"#.to_string());

            Ok(())
        }
    }
}
