use std::io::{self, Write};

fn main() {
    let mut line = String::new();

    loop {
        print!("rshell> ");
        io::stdout().flush().expect("flush failed");

        line.clear();                                    // <-- key fix
        if io::stdin().read_line(&mut line).is_err() { break; }

        let input = line.trim_end();
        if input == "exit" { break; }

        if input.is_empty() { continue; }

        let parts: Vec<&str> = input.split_whitespace().collect();

        let cmd = parts[0];
        let args = &parts[1..];
     
        
        match cmd {
            "echo" => {
                println!("{}",args.join(" "));

            }

            "ls" => {
                match std::fs::read_dir(".") {
                    Ok(entries) => {
                        for entry in entries {
                            if let Ok(e) = entry {
                                println!("{}", e.file_name().to_string_lossy());
                            }
                        }
                    }
                    Err(err) => eprintln!("ls error: {}", err),
                }
            }
            _ => println!("unkown command: {}", cmd),
        }
        
    }
}
