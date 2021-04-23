/*
 *  Copyright (C) 2021, Wafelack <wafelack@protonmail.com>
 *
 *  ------------------------------------------------------
 *
 *     This file is part of Orion.
 *
 *  Orion is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  Orion is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with Orion.  If not, see <https://www.gnu.org/licenses/>.
 */
#[cfg(test)]
mod test {
    use crate::{
        interpreter::Interpreter,
        lexer::{Lexer, TType, Token},
        parser::Parser,
        Result,
    };
    use std::time::Instant;

    
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

            let mut vals = vec![];

            for _ in 0..500 {
                let start = Instant::now();
                interpreter.interpret(false)?;
                let elapsed = start.elapsed();
                vals.push(elapsed.as_millis());
            }
            let summed = vals.iter().sum::<u128>() as u32;
            vals.sort();
            println!("Total: {}ms ; Average: {}ms ; Median : {}ms ; Amplitude: {}ms", summed, summed as f32 / vals.len() as f32, vals[vals.len() / 2], vals[vals.len() - 1] - vals[0]);

            Ok(())

        }
    }
}
