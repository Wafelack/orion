use std::collections::BTreeMap;
use crate::interpreter::value::Value;
use crate::interpreter::interpreter::interpreter::Interpreter;
use crate::*;
use std::io;

impl Interpreter {
    pub fn exit(&mut self, args: &Vec<Value>) -> crate::Result<Value> {
        if args.len() == 1 {
            if let Value::Int(i) = &args[0] {
                std::process::exit(*i);
                Ok(
                    Value::Nil
                )
            } else {
                Err(
                    error!("Invalid argument, expected integer, found", (&args[0].get_type()))
                )
            }
        } else {
            Err(
                error!("Invalid number of arguments, expected 1, found", (args.len()))
            )
        }
    }
    pub fn exec(&mut self, args: &Vec<Value>) -> crate::Result<Value> {
        if args.len() == 2 {
            if let Value::String(cmd) = &args[0] {
                if let Value::List(cmdargs) = &args[1] {
                    let mut s_args = vec![];
                    for arg in cmdargs {
                        if let Value::String(s) = arg {
                            s_args.push(s);
                        } else {
                            return Err(
                                error!("Invalid argument, expected string, found", (arg.get_type()))
                            );
                        }
                    }
                    use std::process::Command;

                    let out = match Command::new(cmd).args(s_args).output() {
                        Ok(o) => o,
                        Err(e) => return Err(
                            error!(e),
                        )
                    };

                    let stdout = std::str::from_utf8(&out.stdout).unwrap_or("");
                    let stderr = std::str::from_utf8(&out.stderr).unwrap_or("");
                    let status = out.status.code().unwrap_or(0);

                    let mut toret: BTreeMap<String, Value> = BTreeMap::new();
                    toret.insert("stdout".to_owned(), Value::String(stdout.to_owned()));
                    toret.insert("stderr".to_owned(), Value::String(stderr.to_owned()));
                    toret.insert("status".to_owned(), Value::Int(status));
                    Ok(
                        Value::Object(
                            toret
                        )
                    )


                } else {
                    Err(
                        error!("Invalid argument, expected list, found", (&args[1].get_type()))
                    )
                }
            } else {
                Err(
                    error!("Invalid argument, expected string, found", (&args[0].get_type()))
                )
            }
        } else {
            Err(
                error!("Invalid number of arguments, expected 2, found", (args.len()))
            )
        }
    }
    pub fn breakpoint(&mut self, args: &Vec<Value>) -> crate::Result<Value> {
        if args.len() == 1 {
            if let Value::String(s) = &args[0] {
                self.breakpnt(&s);
                Ok(
                    Value::Nil
                )
            } else {
                Err(
                    error!("Invalid argument, expected string, found", (&args[0].get_type()))
                )
            }
        } else if args.len() == 0 {
            self.breakpnt("UNNAMED_BREAKPOINT");
            Ok(Value::Nil)
        } else {
            Err(
                error!("Invalid number of arguments, expected 1|0, found", (args.len()))
            )
        }
    }
    fn breakpnt(&mut self, name: &str) {
        println!("\x1b[0;31m\x1b[1m== Program hit breakpoint '{}' ==\x1b[0m", name);

        println!("\nColour codes:\n- \x1b[0;31mred: immutable variables\x1b[0;32m\n- green: mutable variables\n\x1b[0m");

        println!("\nScopes: {}", self.print_scopes());
        println!("\x1b[0;31m\x1b[1m== End of scopes ==\x1b[0m");
        println!("\nPress enter key to continue ...");
        io::stdin().read_line(&mut String::new()).unwrap();
    }
    fn print_scopes(&mut self) -> String {
        let mut toret = String::new();
        for scope in &self.scopes {
            toret.push_str("{\n");
            for (key, (value, mutable)) in scope {
                if *mutable {
                    toret.push_str(
                        format!("\t\x1b[0;32m{} => {} \x1b[0;34m({})\x1b[0m\n", key, value, value.get_type()).as_str()
                    )
                } else {
                    toret.push_str(
                        format!("\t\x1b[0;31m{} => {} \x1b[0;34m({})\x1b[0m\n", key, value, value.get_type()).as_str()
                    )
                }
            }
            toret.push_str("},");
        }
        toret.pop();
        toret.push('\n');
        toret
    }
}