#[cfg(test)]
mod test {
    use crate::lexer::lexer::Lexer;
    use crate::parser::node::*;
    use crate::parser::parser::Parser;

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
}