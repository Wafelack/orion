#[cfg(test)]
mod test {
    use crate::{
        interpreter::Interpreter,
        lexer::{Lexer, TType, Token},
        parser::Parser,
        Result,
    };
    use std::time::Instant;

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
            let code = r#"(load "lib/core/bool.orn")(def foo 99)(def bar foo)(assert_eq foo bar)(assert_eq foo 99)"#;
            let tokens = Lexer::new(code).proc_tokens()?;
            let expressions = Parser::new(tokens).parse()?;
            Interpreter::new(expressions).interpret(false)?;
            Ok(())
        }

        #[test]
        fn functions() -> Result<()> {
            let code = r#"(load "lib/core/bool.orn")(def foo (lambda (x y) (_add x y)))(assert_eq (foo 5 6) 11)"#;
            let tokens = Lexer::new(code).proc_tokens()?;
            let expressions = Parser::new(tokens).parse()?;
            Interpreter::new(expressions).interpret(false)?;
            Ok(())
        }
        #[test]
        fn pattern_matching() -> Result<()> {
            let code = r#"(load "lib/core/bool.orn")(enum List (Cons x next) Nil) (def foo (Cons 9 Nil)) (assert_eq (match foo ((Cons x y) (and (= x 9) (= y Nil)))) True)"#;
            let tokens = Lexer::new(code).proc_tokens()?;
            let expressions = Parser::new(tokens).parse()?;
            Interpreter::new(expressions).interpret(false)?;
            Ok(())
        }

        #[test]
        fn ackermann() -> Result<()> {
            let code = r#"
(def ackermann (lambda (m n)
  (match (, m n)
    ((, 0 _) (+ n 1))
    ((, _ 0) (ackermann (- m 1) 1))
    (_ (ackermann (- m 1) (ackermann m (- n 1)))))))"#;
            let tokens = Lexer::new(code).proc_tokens()?;
            let expressions = Parser::new(tokens).parse()?;
            let mut interpreter = Interpreter::new(expressions);
            interpreter.interpret(false)?;
            interpreter.update_ast(Parser::new(Lexer::new("(ackermann 3 3)").proc_tokens()?).parse()?);
            let start = Instant::now();
            interpreter.interpret(false)?;
            println!("Done in {}ms", start.elapsed().as_millis());
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
            assert_eq!(format!("{:?}", expressions), "[Constr(\"Just\", [Literal(Integer(9))]), Call(Call(Call(Var(\"defn\"), Var(\"factorial\")), Var(\"n\")), Call(Call(Call(Var(\"if\"), Call(Call(Var(\"<\"), Var(\"n\")), Literal(Integer(1)))), Literal(Integer(1))), Call(Call(Var(\"*\"), Var(\"n\")), Call(Var(\"factorial\"), Call(Call(Var(\"-\"), Var(\"n\")), Literal(Integer(1)))))))]".to_string());

            Ok(())
        }
    }
}
