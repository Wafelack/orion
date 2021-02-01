#[cfg(test)]
mod test {
    use crate::lexer::lexer::Lexer;
    use crate::lexer::tokens::Token;
    use crate::parser::node::*;
    use crate::parser::parser::Parser;
    use crate::interpreter::interpreter::interpreter::Interpreter;

    mod interpreter {
        use super::*;
        use crate::*;

        #[test]
        fn definition() -> crate::Result<()> {
            let mut lexer = Lexer::new("(define a 4)".to_owned());
            let ast = Parser::new(lexer.scan_tokens()).parse_tokens()?;
            let mut interpreter = Interpreter::new(ast);
            interpreter.eval()?;

            Ok(())
        }

        #[test]
        fn redefinition() -> crate::Result<()> {
            let mut lexer = Lexer::new("(var a 4)(set a 5)(assert (= a 5))".to_owned());
            let ast = Parser::new(lexer.scan_tokens()).parse_tokens()?;
            let mut interpreter = Interpreter::new(ast);
            interpreter.eval()?;

            Ok(())
        }

        #[test]
        fn adding() -> crate::Result<()> {
            let mut lexer = Lexer::new("(define a (+ \"a\" \"b\"))(define b (+ 4 5))(define c (+ 4. 9))(assert (= a \"ab\"))(assert (= b 9))(assert (= c 13.))".to_owned());
            let ast = Parser::new(lexer.scan_tokens()).parse_tokens()?;
            let mut interpreter = Interpreter::new(ast);
            interpreter.eval()?;

            Ok(())
        }

        #[test]
        fn sub() -> crate::Result<()> {
            let mut lexer = Lexer::new("(assert (= (- 4 5) -1))(assert (= (- 4. 5.) -1.))".to_owned());
            let toks = lexer.scan_tokens();
            let ast = Parser::new(toks).parse_tokens()?;
            let mut interpreter = Interpreter::new(ast);
            interpreter.eval()?;

            Ok(())
        }

        #[test]
        fn mul() -> crate::Result<()> {
            let mut lexer = Lexer::new("(assert (= (* 4 5) 20))(assert (= (* 4. 5.) 20.))".to_owned());
            let toks = lexer.scan_tokens();
            let ast = Parser::new(toks).parse_tokens()?;
            let mut interpreter = Interpreter::new(ast);
            interpreter.eval()?;

            Ok(())
        }


        #[test]
        fn div() -> crate::Result<()> {
            let mut lexer = Lexer::new("(assert (= (/ 4 5) 0 ))(assert (= (/ 4. 5.) 0.8))".to_owned());
            let toks = lexer.scan_tokens();
            let ast = Parser::new(toks).parse_tokens()?;
            let mut interpreter = Interpreter::new(ast);
            interpreter.eval()?;

            Ok(())
        }

        #[test]
        fn modulo() -> crate::Result<()> {
            let mut lexer = Lexer::new("(assert (= (% 4 5) 4 ))(assert (= (% 4. 5.) 4.))".to_owned());
            let toks = lexer.scan_tokens();
            let ast = Parser::new(toks).parse_tokens()?;
            let mut interpreter = Interpreter::new(ast);
            interpreter.eval()?;

            Ok(())
        }

        #[test]
        fn conditions() -> crate::Result<()> {
            let mut lexer = Lexer::new("(define a 5)(define b (if (= a 6) {(return -6)} {(return a)}))(assert (= b a))".to_owned());
            let toks = lexer.scan_tokens();
            let ast = Parser::new(toks).parse_tokens()?;
            let mut interpreter = Interpreter::new(ast);
            interpreter.eval()?;

            Ok(())
        }

        #[test]
        fn boolean_algebra() -> crate::Result<()> {
            let mut lexer = Lexer::new(
                "(assert (! false))(assert (!= 4 5))(assert (= 3. 3.))".to_owned());
            let toks = lexer.scan_tokens();
            let ast = Parser::new(toks).parse_tokens()?;
            let mut interpreter = Interpreter::new(ast);
            interpreter.eval()?;

            Ok(())
        }

        #[test]
        fn looop() -> crate::Result<()> {
            let mut lexer = Lexer::new("(var i 0) (while (< i 5) { (set i (+ i 1))})(assert (= i 5))".to_owned());
            let toks = lexer.scan_tokens();
            let ast = Parser::new(toks).parse_tokens()?;
            let mut interpreter = Interpreter::new(ast);
            interpreter.eval()?;

            Ok(())
        }

        #[test]
        fn func() -> crate::Result<()> {
            let mut lexer = Lexer::new("(define add (lambda (a b) {(+ a b)}))(assert (= (add 5 6) 11))".to_owned());
            let toks = lexer.scan_tokens();
            let ast = Parser::new(toks).parse_tokens()?;
            let mut interpreter = Interpreter::new(ast);
            interpreter.eval()?;

            Ok(())
        }

        #[test]
        fn list() -> crate::Result<()> {
            let mut lexer = Lexer::new("(define a (list 5 6 8))(assert (= (@ a 2) 8))".to_owned());
            let toks = lexer.scan_tokens();
            let ast = Parser::new(toks).parse_tokens()?;
            let mut interpreter = Interpreter::new(ast);
            interpreter.eval()?;

            Ok(())
        }
        #[test]
        fn object() -> crate::Result<()> {

            let code = r#"
    (define foo (object "a" "b"
                        "c" 5
                        "d" false
                        "e" nil
                        "f" 5.5))
    (assert (= (@ foo "f") 5.5))"#;

            let mut lexer = Lexer::new(code.to_owned());
            let ast = Parser::new(lexer.scan_tokens()).parse_tokens()?;
            let mut interpreter = Interpreter::new(ast);

            interpreter.eval()?;
            Ok(())
        }

        mod push {
            use super::*;

            #[test]
            fn list() -> crate::Result<()> {
                let code = "(var a (list 4 5))(set a (push a 4))(assert (= (@ a 2) 4))";

                let mut lexer = Lexer::new(code.to_owned());
                let toks = lexer.scan_tokens();
                let ast = Parser::new(toks).parse_tokens()?;
                let mut interpreter = Interpreter::new(ast);

                interpreter.eval()?;
                Ok(())
            }

            #[test]
            fn objects() -> crate::Result<()> {
                let code = "(var a (object \"a\" 5))(set a (push a \"b\" false))(print a)(assert (= (@ a \"b\") false))";

                let mut lexer = Lexer::new(code.to_owned());
                let toks = lexer.scan_tokens();
                let ast = Parser::new(toks).parse_tokens()?;
                let mut interpreter = Interpreter::new(ast);

                interpreter.eval()?;
                Ok(())
            }
        }

        #[test]
        fn len() -> crate::Result<()> {
            let code = "(var a (list 4 5))(assert (= (length a) 2))";

            let mut lexer = Lexer::new(code.to_owned());
            let toks = lexer.scan_tokens();
            let ast = Parser::new(toks).parse_tokens()?;
            let mut interpreter = Interpreter::new(ast);

            interpreter.eval()?;
            Ok(())
        }

        #[test]
        fn pop() -> crate::Result<()> {
            let code = "(var a (list 4 5))(set a (pop a))(assert (= (length a) 1))";

            let mut lexer = Lexer::new(code.to_owned());
            let toks = lexer.scan_tokens();
            let ast = Parser::new(toks).parse_tokens()?;
            let mut interpreter = Interpreter::new(ast);

            interpreter.eval()?;
            Ok(())
        }


        mod fs{
            use super::*;

            #[test]
            fn exists() -> crate::Result<()> {
                let code = "(assert (fs:exists? \".\"))";

                let mut lexer = Lexer::new(code.to_owned());
                let toks = lexer.scan_tokens();
                let ast = Parser::new(toks).parse_tokens()?;
                let mut interpreter = Interpreter::new(ast);

                interpreter.eval()?;
                Ok(())
            }

            #[test]
            fn read_dir() -> crate::Result<()> {
                let code = "(assert (= (@ (fs:readDir \"src\") 0) \"src/main.rs\"))";

                let mut lexer = Lexer::new(code.to_owned());
                let toks = lexer.scan_tokens();
                let ast = Parser::new(toks).parse_tokens()?;
                let mut interpreter = Interpreter::new(ast);

                interpreter.eval()?;
                Ok(())
            }
        }
    }
}