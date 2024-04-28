use std::io::{stdin, stdout, Write};

use anyhow::Result;

use nac::Expression;

fn main() -> Result<()> {
    let mut args = std::env::args().collect::<Vec<String>>();

    let _program_name = args.remove(0);

    if !args.is_empty() {
        let root = Expression::root(args.join("").as_str());

        match root {
            Ok(mut root) => {
                let res = root.eval();

                match res {
                    Ok(res) => {
                        println!("{}", res);
                    }
                    Err(e) => {
                        eprintln!("{}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("{}", e);
            }
        }
    } else {
        let mut input = String::new();

        loop {
            input.clear();
            print!("> ");
            stdout().flush()?;

            stdin().read_line(&mut input)?;
            if input.starts_with("q") {
                break;
            }
            let root = Expression::root(input.as_str());

            match root {
                Ok(mut root) => {
                    let res = root.eval();

                    match res {
                        Ok(res) => {
                            println!("{}", res);
                        }
                        Err(e) => {
                            eprintln!("{}", e);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("{}", e);
                }
            }
        }
    }

    Ok(())
}
