#[cfg(test)]
mod test {
    mod interpreter {
        use crate::*;

        #[test]
        fn definition() -> crate::Result<()> {
            let mut lexer = Lexer::new("(define a 4)".to_owned());
            let ast = Parser::new(lexer.scan_tokens()).parse_tokens()?;
            let mut interpreter = Interpreter::new(ast, vec![]);
            interpreter.eval()?;

            Ok(())
        }

        #[test]
        fn redefinition() -> crate::Result<()> {
            let mut lexer = Lexer::new("(var a 4)(set a 5)(assert (= a 5))".to_owned());
            let ast = Parser::new(lexer.scan_tokens()).parse_tokens()?;
            let mut interpreter = Interpreter::new(ast, vec![]);
            interpreter.eval()?;

            Ok(())
        }

        #[test]
        fn adding() -> crate::Result<()> {
            let mut lexer = Lexer::new("(define a (+ \"a\" \"b\"))(define b (+ 4 5))(define c (+ 4. 9))(assert (= a \"ab\"))(assert (= b 9))(assert (= c 13.))".to_owned());
            let ast = Parser::new(lexer.scan_tokens()).parse_tokens()?;
            let mut interpreter = Interpreter::new(ast, vec![]);
            interpreter.eval()?;

            Ok(())
        }

        #[test]
        fn sub() -> crate::Result<()> {
            let mut lexer = Lexer::new("(assert (= (- 4 5) -1))(assert (= (- 4. 5.) -1.))".to_owned());
            let toks = lexer.scan_tokens();
            let ast = Parser::new(toks).parse_tokens()?;
            let mut interpreter = Interpreter::new(ast, vec![]);
            interpreter.eval()?;

            Ok(())
        }

        #[test]
        fn mul() -> crate::Result<()> {
            let mut lexer = Lexer::new("(assert (= (* 4 5) 20))(assert (= (* 4. 5.) 20.))".to_owned());
            let toks = lexer.scan_tokens();
            let ast = Parser::new(toks).parse_tokens()?;
            let mut interpreter = Interpreter::new(ast, vec![]);
            interpreter.eval()?;

            Ok(())
        }


        #[test]
        fn div() -> crate::Result<()> {
            let mut lexer = Lexer::new("(assert (= (/ 4 5) 0 ))(assert (= (/ 4. 5.) 0.8))".to_owned());
            let toks = lexer.scan_tokens();
            let ast = Parser::new(toks).parse_tokens()?;
            let mut interpreter = Interpreter::new(ast, vec![]);
            interpreter.eval()?;

            Ok(())
        }

        #[test]
        fn modulo() -> crate::Result<()> {
            let mut lexer = Lexer::new("(assert (= (% 4 5) 4 ))(assert (= (% 4. 5.) 4.))".to_owned());
            let toks = lexer.scan_tokens();
            let ast = Parser::new(toks).parse_tokens()?;
            let mut interpreter = Interpreter::new(ast, vec![]);
            interpreter.eval()?;

            Ok(())
        }

        #[test]
        fn conditions() -> crate::Result<()> {
            let mut lexer = Lexer::new("(define a 5)(define b (if (= a 6) {(return -6)} {(return a)}))(assert (= b a))".to_owned());
            let toks = lexer.scan_tokens();
            let ast = Parser::new(toks).parse_tokens()?;
            let mut interpreter = Interpreter::new(ast, vec![]);
            interpreter.eval()?;

            Ok(())
        }

        #[test]
        fn boolean_algebra() -> crate::Result<()> {
            let mut lexer = Lexer::new(
                "(assert (! false))(assert (!= 4 5))(assert (= 3. 3.))".to_owned());
            let toks = lexer.scan_tokens();
            let ast = Parser::new(toks).parse_tokens()?;
            let mut interpreter = Interpreter::new(ast, vec![]);
            interpreter.eval()?;

            Ok(())
        }

        #[test]
        fn looop() -> crate::Result<()> {
            let mut lexer = Lexer::new("(var i 0) (while (< i 5) { (set i (+ i 1))})(assert (= i 5))".to_owned());
            let toks = lexer.scan_tokens();
            let ast = Parser::new(toks).parse_tokens()?;
            let mut interpreter = Interpreter::new(ast, vec![]);
            interpreter.eval()?;

            Ok(())
        }

        #[test]
        fn func() -> crate::Result<()> {
            let mut lexer = Lexer::new("(define add (lambda (a b) {(+ a b)}))(assert (= (add 5 6) 11))".to_owned());
            let toks = lexer.scan_tokens();
            let ast = Parser::new(toks).parse_tokens()?;
            let mut interpreter = Interpreter::new(ast, vec![]);
            interpreter.eval()?;

            Ok(())
        }

        #[test]
        fn list() -> crate::Result<()> {
            let mut lexer = Lexer::new("(define a (list 5 6 8))(assert (= (@ a 2) 8))".to_owned());
            let toks = lexer.scan_tokens();
            let ast = Parser::new(toks).parse_tokens()?;
            let mut interpreter = Interpreter::new(ast, vec![]);
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
            let mut interpreter = Interpreter::new(ast, vec![]);

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
                let mut interpreter = Interpreter::new(ast, vec![]);

                interpreter.eval()?;
                Ok(())
            }

            #[test]
            fn objects() -> crate::Result<()> {
                let code = "(var a (object \"a\" 5))(set a (push a \"b\" false))(assert (= (@ a \"b\") false))";

                let mut lexer = Lexer::new(code.to_owned());
                let toks = lexer.scan_tokens();
                let ast = Parser::new(toks).parse_tokens()?;
                let mut interpreter = Interpreter::new(ast, vec![]);

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
            let mut interpreter = Interpreter::new(ast, vec![]);

            interpreter.eval()?;
            Ok(())
        }

        #[test]
        fn pop() -> crate::Result<()> {
            let code = "(var a (list 4 5))(set a (pop a))(assert (= (length a) 1))";

            let mut lexer = Lexer::new(code.to_owned());
            let toks = lexer.scan_tokens();
            let ast = Parser::new(toks).parse_tokens()?;
            let mut interpreter = Interpreter::new(ast, vec![]);

            interpreter.eval()?;
            Ok(())
        }

        #[test]
        fn foreach() -> crate::Result<()> {
            let code = r#"
            (var acc (list))
            (define obj (object "a" 0 "b" 1 "c" 2))
            (foreach obj (lambda (_ v) {
                (set acc (push acc v))
            }))
            (assert (= (list 0 1 2) acc))"#;

            let mut lexer = Lexer::new(code.to_owned());
            let toks = lexer.scan_tokens();
            let ast = Parser::new(toks).parse_tokens()?;
            let mut interpreter = Interpreter::new(ast, vec![]);

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
                let mut interpreter = Interpreter::new(ast, vec![]);

                interpreter.eval()?;
                Ok(())
            }

            #[test]
            fn create_file() -> crate::Result<()> {
                let code = "(fs:createFile \"test.txt\")(assert (fs:exists? \"test.txt\"))(fs:removeFile \"test.txt\")";

                let mut lexer = Lexer::new(code.to_owned());
                let toks = lexer.scan_tokens();
                let ast = Parser::new(toks).parse_tokens()?;
                let mut interpreter = Interpreter::new(ast, vec![]);

                interpreter.eval()?;
                Ok(())
            }

            #[test]
            fn exec() -> crate::Result<()> {
                let code = "(define out (sys:exec \"echo\" (list)))(assert (= (@ out \"status\") 0))";

                let mut lexer = Lexer::new(code.to_owned());
                let toks = lexer.scan_tokens();
                let ast = Parser::new(toks).parse_tokens()?;
                let mut interpreter = Interpreter::new(ast, vec![]);

                interpreter.eval()?;
                Ok(())
            }

            #[test]
            fn slice() -> crate::Result<()> {
                let code = "(define a (list 0 1 2 3))(define expected (list 1 2 3))(assert (= (slice a 1 4) expected))";

                let mut lexer = Lexer::new(code.to_owned());
                let toks = lexer.scan_tokens();
                let ast = Parser::new(toks).parse_tokens()?;
                let mut interpreter = Interpreter::new(ast, vec![]);

                interpreter.eval()?;
                Ok(())
            }

            #[test]
            fn static_cast() -> crate::Result<()> {
                let code = "(define a \"true\"); string\n(define casted (static_cast \"bool\" a))(assert casted)";

                let mut lexer = Lexer::new(code.to_owned());
                let toks = lexer.scan_tokens();
                let ast = Parser::new(toks).parse_tokens()?;
                let mut interpreter = Interpreter::new(ast, vec![]);

                interpreter.eval()?;
                Ok(())
            }

            mod math {
                use super::*;
                use std::time::Instant;

                #[test]
                fn cos() -> crate::Result<()> {
                    let code = "(define a (math:cos (/ math:PI 3)))(assert (< a 0.50))";

                    let mut lexer = Lexer::new(code.to_owned());
                    let toks = lexer.scan_tokens();
                    let ast = Parser::new(toks).parse_tokens()?;
                    let mut interpreter = Interpreter::new(ast, vec![]);

                    interpreter.eval()?;
                    Ok(())
                }
                #[test]
                fn sin() -> crate::Result<()> {
                    let code = "(define a (math:sin (/ math:PI 6)))(print a)(assert (= a 0.5))";

                    let mut lexer = Lexer::new(code.to_owned());
                    let toks = lexer.scan_tokens();
                    let ast = Parser::new(toks).parse_tokens()?;
                    let mut interpreter = Interpreter::new(ast, vec![]);

                    interpreter.eval()?;
                    Ok(())
                }
                #[test]
                fn tan() -> crate::Result<()> {
                    let code = "(define a (math:tan (/ math:PI 4)))(print a)(assert (= a 1.))";

                    let mut lexer = Lexer::new(code.to_owned());
                    let toks = lexer.scan_tokens();
                    let ast = Parser::new(toks).parse_tokens()?;
                    let mut interpreter = Interpreter::new(ast, vec![]);

                    interpreter.eval()?;
                    Ok(())
                }

                #[test]
                fn acos() -> crate::Result<()> {

                    let code = "(define a (math:acos 0.5))(print a)(assert (= a (/ math:PI 3)))";

                    let mut lexer = Lexer::new(code.to_owned());
                    let toks = lexer.scan_tokens();
                    let ast = Parser::new(toks).parse_tokens()?;
                    let mut interpreter = Interpreter::new(ast, vec![]);

                    interpreter.eval()?;
                    Ok(())
                }

                #[test]
                fn asin() -> crate::Result<()> {

                    let code = "(define a (math:asin 0.5))(print a)(assert (= a (/ math:PI 6)))";

                    let mut lexer = Lexer::new(code.to_owned());
                    let toks = lexer.scan_tokens();
                    let ast = Parser::new(toks).parse_tokens()?;
                    let mut interpreter = Interpreter::new(ast, vec![]);

                    interpreter.eval()?;
                    Ok(())
                }

                #[test]
                fn atan() -> crate::Result<()> {

                    let code = "(define a (math:atan 1.))(print a)(assert (= a  (/ math:PI 4)))";

                    let mut lexer = Lexer::new(code.to_owned());
                    let toks = lexer.scan_tokens();
                    let ast = Parser::new(toks).parse_tokens()?;
                    let mut interpreter = Interpreter::new(ast, vec![]);

                    interpreter.eval()?;
                    Ok(())
                }

                #[test]
                fn max() -> crate::Result<()> {

                    let code = "(assert (= (math:max 4. 5.) 5.))";

                    let mut lexer = Lexer::new(code.to_owned());
                    let toks = lexer.scan_tokens();
                    let ast = Parser::new(toks).parse_tokens()?;
                    let mut interpreter = Interpreter::new(ast, vec![]);

                    interpreter.eval()?;
                    Ok(())
                }
                #[test]
                fn min() -> crate::Result<()> {

                    let code = "(assert (= (math:min 4. 5.) 4.))";

                    let mut lexer = Lexer::new(code.to_owned());
                    let toks = lexer.scan_tokens();
                    let ast = Parser::new(toks).parse_tokens()?;
                    let mut interpreter = Interpreter::new(ast, vec![]);

                    interpreter.eval()?;
                    Ok(())
                }
                #[test]
                fn sqrt() -> crate::Result<()> {

                    let code = "(assert (= (math:sqrt 4.) 2.))";

                    let mut lexer = Lexer::new(code.to_owned());
                    let toks = lexer.scan_tokens();
                    let ast = Parser::new(toks).parse_tokens()?;
                    let mut interpreter = Interpreter::new(ast, vec![]);

                    interpreter.eval()?;
                    Ok(())
                }
                #[test]
                fn pow() -> crate::Result<()> {

                    let code = "(assert (= (math:pow 2. 8) 256.))";

                    let mut lexer = Lexer::new(code.to_owned());
                    let toks = lexer.scan_tokens();
                    let ast = Parser::new(toks).parse_tokens()?;
                    let mut interpreter = Interpreter::new(ast, vec![]);

                    interpreter.eval()?;
                    Ok(())
                }

                #[test]
                fn range() -> crate::Result<()> {

                    let code = "(assert (= (math:range 2 8) (list 2 3 4 5 6 7)))";

                    let mut lexer = Lexer::new(code.to_owned());
                    let toks = lexer.scan_tokens();
                    let ast = Parser::new(toks).parse_tokens()?;
                    let mut interpreter = Interpreter::new(ast, vec![]);

                    interpreter.eval()?;
                    Ok(())
                }

                #[test]
                fn clamp() -> crate::Result<()> {

                    let code = "(assert (= (math:clamp 2. 3. 8.) 3.))";

                    let mut lexer = Lexer::new(code.to_owned());
                    let toks = lexer.scan_tokens();
                    let ast = Parser::new(toks).parse_tokens()?;
                    let mut interpreter = Interpreter::new(ast, vec![]);

                    interpreter.eval()?;
                    Ok(())
                }
                #[test]
                fn odd() -> crate::Result<()> {

                    let code = "(assert (math:odd 5))";

                    let mut lexer = Lexer::new(code.to_owned());
                    let toks = lexer.scan_tokens();
                    let ast = Parser::new(toks).parse_tokens()?;
                    let mut interpreter = Interpreter::new(ast, vec![]);

                    interpreter.eval()?;
                    Ok(())
                }

                #[test]
                fn ack() -> crate::Result<()> {
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
                    let start = Instant::now();
                    let mut lexer = Lexer::new(code.to_owned());
                    let toks = lexer.scan_tokens();
                    let ast = Parser::new(toks).parse_tokens()?;
                    let mut interpreter = Interpreter::new(ast, vec![]);
                    interpreter.eval()?;
                    let elapsed = start.elapsed().as_millis();
                    println!("Ack(3, 3) done in {}ms", elapsed);
                    Ok(())
                }

                #[test]
                fn matching() -> crate::Result<()> {

                    let code = r#"
(define a "foo")
(define result (match a {
    (=> "bar" {
        (return "Of course, it is bar !")
    })
    (_ {
        (+ "It is not bar, but it is '" (+ a "'"))
    })
}))
(assert (= result "It is not bar, but it is 'foo'"))"#;

                    let mut lexer = Lexer::new(code.to_owned());
                    let toks = lexer.scan_tokens();
                    let ast = Parser::new(toks).parse_tokens()?;
                    let mut interpreter = Interpreter::new(ast, vec![]);

                    interpreter.eval()?;
                    Ok(())
                }

                #[test]
                fn some() -> crate::Result<()> {
                    let code = "(define a (some 4))(assert (= (typeof a) \"option\"))";

                    let mut lexer = Lexer::new(code.to_owned());
                    let toks = lexer.scan_tokens();
                    let ast = Parser::new(toks).parse_tokens()?;
                    let mut interpreter = Interpreter::new(ast, vec![]);

                    interpreter.eval()?;
                    Ok(())
                }

                #[test]
                fn none() -> crate::Result<()> {
                    let code = "(define a (none))(assert (= (typeof a) \"option\"))";

                    let mut lexer = Lexer::new(code.to_owned());
                    let toks = lexer.scan_tokens();
                    let ast = Parser::new(toks).parse_tokens()?;
                    let mut interpreter = Interpreter::new(ast, vec![]);

                    interpreter.eval()?;
                    Ok(())
                }

                #[test]
                fn unwrap() -> crate::Result<()> {
                    let code = "(define a (some 4))(assert (= (typeof (? a)) \"int\"))";

                    let mut lexer = Lexer::new(code.to_owned());
                    let toks = lexer.scan_tokens();
                    let ast = Parser::new(toks).parse_tokens()?;
                    let mut interpreter = Interpreter::new(ast, vec![]);

                    interpreter.eval()?;
                    Ok(())
                }

                #[test]
                #[should_panic]
                fn unwrap_none() {
                    let code = "(define a (none))(assert (= (typeof (? a)) \"int\"))";

                    let mut lexer = Lexer::new(code.to_owned());
                    let toks = lexer.scan_tokens();
                    let ast = Parser::new(toks).parse_tokens().unwrap();
                    let mut interpreter = Interpreter::new(ast, vec![]);

                    interpreter.eval().unwrap();
                }

                #[test]
                fn unwrap_or() {
                    let code = "(define a (none))(assert (= (typeof (?=> a 4)) \"int\"))";

                    let mut lexer = Lexer::new(code.to_owned());
                    let toks = lexer.scan_tokens();
                    let ast = Parser::new(toks).parse_tokens().unwrap();
                    let mut interpreter = Interpreter::new(ast, vec![]);

                    interpreter.eval().unwrap();
                }

            }
        }
    }
}