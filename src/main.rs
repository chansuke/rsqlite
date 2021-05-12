use rustyline::error::ReadlineError;
use rustyline::Editor;

fn main() {
    let mut rl = Editor::<()>::new();

    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }

    loop {
        let readline = rl.readline("db >> ");
        match readline {
            Ok(line) => {
                if line == ".exit" {
                    break;
                } else {
                    println!("Unrecognized command {:?}", line);
                }
            }
            Err(err) => {
                println!("Error reading input");
                break;
            }
        }
    }
}
