#[cfg(test)]
mod tests {
    use crate::lexer::lexer::Lexer;
    use crate::lexer::tokens::Token;
    use crate::parser::node::*;
    use crate::parser::parser::Parser;
    use crate::interpreter::interpreter::interpreter::Interpreter;

    mod tokenizing {
        use super::*;

        #[test]
        fn string() {
            let mut lexer = Lexer::new("\"foo\"".to_owned());
            assert_eq!(lexer.scan_tokens(), vec![Token::String("foo".to_owned())]);
        }

        #[test]
        fn float() {
            let mut lexer = Lexer::new("43.50".to_owned());
            assert_eq!(lexer.scan_tokens(), vec![Token::Float(43.50)]);
        }

        #[test]
        fn int() {
            let mut lexer = Lexer::new("43".to_owned());
            assert_eq!(lexer.scan_tokens(), vec![Token::Int(43)]);
        }

        #[test]
        fn boolean() {
            let mut lexer = Lexer::new("true false".to_owned());
            assert_eq!(lexer.scan_tokens(), vec![Token::Bool(true), Token::Bool(false)]);
        }

        #[test]
        fn nil() {
            let mut lexer = Lexer::new("n ni nil".to_owned());
            assert_eq!(lexer.scan_tokens(), vec![Token::Identifier("n".to_owned()), Token::Identifier("ni".to_owned()), Token::Nil]); 
        }

        #[test]
        fn comment() {
            let mut lexer = Lexer::new("# a looooooooooooooooooong comment\n45".to_owned());
            assert_eq!(lexer.scan_tokens(), vec![Token::Int(45)]);
        }

        #[test]
        fn paren() {
            let mut lexer = Lexer::new("()".to_owned());
            assert_eq!(lexer.scan_tokens(), vec![Token::LeftParen, Token::RightParen]);
        }

        #[test]
        fn brace() {
            let mut lexer = Lexer::new("{}".to_owned());
            assert_eq!(lexer.scan_tokens(), vec![Token::LeftBrace, Token::RightBrace]);
        }

        #[test]
        fn identifier() {
            let mut lexer = Lexer::new("def".to_owned());
            assert_eq!(lexer.scan_tokens(), vec![Token::Identifier("def".to_owned())]);
        }
    }

    mod parsing {

        use super::*;

        #[test]
        fn multiple_args() -> crate::Result<()> {
            let mut lexer = Lexer::new("(print \"foo\" 4 nil true)".to_owned());
            let mut parser = Parser::new( lexer.scan_tokens());
            let ast = parser.parse_tokens()?;

            assert_eq!(ast, Node {
                ntype: NodeType::Scope,
                children: vec![
                    Node {
                        ntype: NodeType::FunctionCall(
                            "print".to_owned(),
                        ),
                        children: vec![
                            Node {
                                ntype: NodeType::String(
                                    "foo".to_owned(),
                                ),
                                children: vec![],
                            },
                            Node {
                                ntype: NodeType::Int(
                                    4
                                ),
                                children: vec![],
                            },
                            Node {
                                ntype: NodeType::Nil,
                                children: vec![],
                            },
                            Node {
                                ntype: NodeType::Bool(
                                    true
                                ),
                                children: vec![],
                            },
                        ],
                    },
                ],
            });

            Ok(())
        }

        fn standardize(ast: &str) -> String {
            ast.replace("\r", "")
                .replace("\n", "")
                .replace(" ", "")
                .replace("\t", "")
        }

        #[test]
        fn condition() -> crate::Result<()> {
            let code = "(if (= a b) {(print \"equal\")} {(print \"not equal\")})";
            let mut lexer = Lexer::new(code.to_owned());
            let mut parser = Parser::new( lexer.scan_tokens());
            let ast = parser.parse_tokens()?;

            assert_eq!(standardize(&stringify(&ast,0)), standardize(r#"{
                @type : FunctionCall("if")
                @children : {
                  @type : FunctionCall("=")
                  @children : {
                    @type : Identifier("a")
                    @children : {
                      }
                    @type : Identifier("b")
                    @children : {
                      }
                    }
                  @type : Scope
                  @children : {
                    @type : FunctionCall("print")
                    @children : {
                      @type : String("equal")
                      @children : {
                        }
                      }
                    }
                  @type : Scope
                  @children : {
                    @type : FunctionCall("print")
                    @children : {
                      @type : String("not equal")
                      @children : {
                        }
                      }
                    }
                  }
                }"#));

            Ok(())
        }

    }

    mod interpreter {
        use super::*;

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

        mod benches {
            use super::*;

            #[test]
            fn ackermann() -> crate::Result<()> {
                let code = r#"
                (define ack (lambda (m n) {
                    (if (= m 0) {
                        (+ n 1)
                    } {
                        (if (= n 0) {
                            (ack (- m 1) 1)
                        } {
                            (ack (- m 1) (ack m (- n 1)))
                        })
                    })
                }))
                (ack 3 3)"#;
                    
                let mut lexer = Lexer::new(code.to_owned());
                let toks = lexer.scan_tokens();
                let ast = Parser::new(toks).parse_tokens()?;
                let mut interpreter = Interpreter::new(ast);
        
                use std::time::Instant;

                let mut vals = vec![];

                for _ in 0..500 {
                    let start = Instant::now();
                    interpreter.eval()?;
                    let elapsed = start.elapsed();
                    vals.push(elapsed.as_millis());
                }

                vals.sort();
                let total = vals.iter().sum::<u128>();
                let average = total / vals.len() as u128;
                let median = vals[vals.len() / 2];
                let amplitude = vals[vals.len() - 1] - vals[0];
                println!("Total: {}ms ; Average: {}ms ; Median: {}ms; Amplitude: {}ms", total, average, median, amplitude);

                Ok(())
            }

            #[test]
            fn push() -> crate::Result<()> {
                let code = r#"
                (var list (list))
                (var i 0)
                (while (< i 1000) {
                    (set list (push list i))
                    (set i (+ i 1))
                })"#;
                    
                let mut lexer = Lexer::new(code.to_owned());
                let toks = lexer.scan_tokens();
                let ast = Parser::new(toks).parse_tokens()?;
                let mut interpreter = Interpreter::new(ast);
        
                use std::time::Instant;

                let mut vals = vec![];

                for _ in 0..500 {
                    let start = Instant::now();
                    interpreter.eval()?;
                    let elapsed = start.elapsed();
                    vals.push(elapsed.as_millis());
                }

                vals.sort();
                let total = vals.iter().sum::<u128>();
                let average = total / vals.len() as u128;
                let median = vals[vals.len() / 2];
                let amplitude = vals[vals.len() - 1] - vals[0];
                println!("Total: {}ms ; Average: {}ms ; Median: {}ms; Amplitude: {}ms", total, average, median, amplitude);

                Ok(())
            }
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
        }
    }


}