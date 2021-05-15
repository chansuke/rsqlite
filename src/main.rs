use rustyline::Editor;

use rsqlite::table::{Row, Statement, StatementType, EMAIL_SIZE, USERNAME_SIZE};

enum MetaCommandResult {
    Success,
    UnrecognizedCommand,
}

enum PrepareResult {
    Success,
    UnrecognizedStatement,
}

fn do_meta_command(buffer: &str) -> MetaCommandResult {
    if buffer == ".exit" {
        MetaCommandResult::Success
    } else {
        MetaCommandResult::UnrecognizedCommand
    }
}

fn prepare_statement(buffer: &str, stmt: &mut Statement) -> PrepareResult {
    if buffer.starts_with("insert") {
        stmt.stmt_type = StatementType::Insert;

        if buffer.split_whitespace().count() <= 3 {
            PrepareResult::UnrecognizedStatement
        } else {
            PrepareResult::Success
        }
    } else if buffer == "select" {
        stmt.stmt_type = StatementType::Select;

        PrepareResult::Success
    } else {
        PrepareResult::UnrecognizedStatement
    }
}

//fn execute_statement(statement: StatementType) {
//    match statement {
//        StatementType::Insert => {
//            println!("This is where we would do an insert.");
//        }
//        StatementType::Select => {
//            println!("This is where we would do a select");
//        }
//        StatementType::Empty => {
//            println!("Not selected yet");
//        }
//    }
//}

fn main() {
    let mut rl = Editor::<()>::new();

    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }

    loop {
        let input_buffer = rl.readline("db >> ");

        match input_buffer {
            Ok(buffer) => {
                if buffer.starts_with('.') {
                    match do_meta_command(&buffer) {
                        MetaCommandResult::Success => {
                            continue;
                        }
                        MetaCommandResult::UnrecognizedCommand => {
                            println!("Unrecognized command {:?}", buffer);
                        }
                    }
                }
                let mut statement: Statement = Statement::new(
                    StatementType::Empty,
                    Row::new(0, [0u8; USERNAME_SIZE], [0u8; EMAIL_SIZE]),
                );

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
            Err(_err) => {
                println!("Error reading input");
                break;
            }
        }
    }
}
