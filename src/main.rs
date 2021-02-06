use rustyline::Editor;
use anyhow::Result;

fn main() -> Result<()>{
    let mut rl = Editor::<()>::new();

    loop {
        let readline = rl.readline("rsqlite> ");
        match readline {
            Ok(line) => {
                if line == ".exit" {
                    println!("Bye");
                    break
                } else {
                    println!("Unrecognized command {:?}", line);
                }
            },
            Err(err) => {
                println!("Error {:?}", err);
                break
            }
        }
    }

    Ok(())
}
