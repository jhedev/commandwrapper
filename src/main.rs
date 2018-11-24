use std::env;
use std::process::Command;

#[derive(Debug)]
struct Execute {
    cmd: String,
    args: Vec<String>,
}

impl Execute {
    fn from_string(cmd: String) -> Result<Execute,String> {
        let mut iter = cmd.split_whitespace();
        match iter.next() {
            None => return Err("no command given".to_string()),
            Some(cmd) => {
                return Ok(Execute{
                    cmd: cmd.to_string(),
                    args: iter.map(|x| x.to_string()).collect(),
                });
            },
        };
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut execs: Vec<Execute> = vec![];
    for arg in args {
        if !arg.starts_with("-execute=") {
            continue
        }
        match arg.get(9..arg.len()) {
            None => continue,
            Some(s) => match Execute::from_string(s.to_string()) {
                Err(err) => {
                    println!("got error: {}", err);
                },
                Ok(exec) => {
                    execs.push(exec);
                },
            },
        };
    };

    for e in execs {
        match Command::new(e.cmd).args(e.args).spawn() {
            Ok(mut child) => {
                let _ = child.wait();
            }
            Err(err) => println!("got error: {}", err),
        };
    }
}
