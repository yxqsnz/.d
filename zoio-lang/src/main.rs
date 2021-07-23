use std::env::args;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Error;
use std::path::Path;
use std::process::exit;
use zoio::util::*;
use zoio::Zoio;
fn main() -> Result<(), Error> {
    let args = args().collect::<Vec<String>>();
    let mut zoio = Zoio::new();
    if args.len() < 2 {
        println!("zoio <FILE>");
        println!("zoio -i");
        exit(1);
    }
    if let Some(arg) = args.get(1) {
        if arg == "-i" {
            println!("type `/help` for help.");
            loop {
                let line = input(Some(">>> "))?;
                match line.as_str() {
                    "/help" => {
                        println!("/help show this message\n/exit leave this program")
                    }
                    "/exit" => {
                        exit(0);
                    }
                    "/vars" => {
                        println!("{:#?}", &zoio.vars);
                    }
                    _ => {
                        let res = zoio.run(&line);
                        if let Ok(res) = res {
                            println!("{}", res);
                        } else {
                            println!("err: {}", res.unwrap_err())
                        }
                    }
                }
            }
        } else {
            if !Path::new(&arg).exists() {
                println!("error: {} is NULL.", arg);
                exit(1);
            }
            let file = File::open(&arg)?;
            let reader = BufReader::new(file);
            for line in reader.lines() {
                let line = line?;
                if let Err(err) = zoio.run(&line) {
                    println!("err: {} at line {}", err, zoio.current_line);
                }
            }
        }
    }

    Ok(())
}
