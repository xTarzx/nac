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

        let mut prev_result: f64 = 0.0;
        loop {
            input.clear();

            print!("> ");
            stdout().flush().unwrap();
            stdin().read_line(&mut input)?;
            if input.starts_with("q") {
                break;
            }
            let root = Expression::root_with_prev(input.as_str(), prev_result);

            match root {
                Ok(mut root) => {
                    let res = root.eval();

                    match res {
                        Ok(res) => {
                            prev_result = res;
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
