use derive_new::new;
use rustyline::error::ReadlineError;
use rustyline::Editor;

enum MetaCommandResult {
    Success,
    UnrecognizedCommand,
}

enum PrepareResult {
    Success,
    UnrecognizedStatement,
}

enum StatementType {
    Insert,
    Select,
    Empty,
}

#[derive(new)]
struct Statement {
    stmt_type: StatementType,
}

fn do_meta_command(buffer: &str) -> MetaCommandResult {
    if buffer == ".exit" {
        MetaCommandResult::Success
    } else {
        MetaCommandResult::UnrecognizedCommand
    }
}

fn prepare_statement(buffer: &str, stmt: &mut Statement) -> PrepareResult {
    if buffer == "insert" {
        stmt.stmt_type = StatementType::Insert;

        PrepareResult::Success
    } else if buffer == "select" {
        stmt.stmt_type = StatementType::Select;

        PrepareResult::Success
    } else {
        PrepareResult::UnrecognizedStatement
    }
}

fn execute_statement(statement: StatementType) {
    match statement {
        StatementType::Insert => {
            println!("This is where we would do an insert.");
        }
        StatementType::Select => {
            println!("This is where we would do a select");
        }
        StatementType::Empty => {
            println!("Not selected yet");
        }
    }
}

fn main() {
    let mut rl = Editor::<()>::new();

    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }

    loop {
        let input_buffer = rl.readline("db >> ");

        match input_buffer {
            Ok(buffer) => {
                if buffer.starts_with(".") {
                    match do_meta_command(&buffer) {
                        MetaCommandResult::Success => {
                            continue;
                        }
                        MetaCommandResult::UnrecognizedCommand => {
                            println!("Unrecognized command {:?}", buffer);
                        }
                    }
                }
                let mut statement: Statement = Statement::new(StatementType::Empty);

                match prepare_statement(&buffer, &mut statement) {
                    PrepareResult::Success => {
                        break;
                    }
                    PrepareResult::UnrecognizedStatement => {
                        println!("Unrecongnized keyword at start of {:?}", buffer);
                        continue;
                    }
                }
            }
            Err(err) => {
                println!("Error reading input");
                break;
            }
        }
    }
}
