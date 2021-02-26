use derive_new::new;
use rustyline::Editor;

use rsqlite::table::*;

enum MetaCommandResult {
    Success,
    UnrecognizedCommand,
}

enum PrepareResult {
    Success,
    UnrecognizeStatement,
    SyntaxError,
}

#[derive(PartialEq)]
enum StatementType {
    Insert,
    Select,
    Empty,
}

enum ExecuteResult {
    Success,
    TableFull,
}

#[derive(new)]
struct Statement {
    kind: StatementType,
    row_to_insert: Row,
}

fn do_meta_command(buffer: &str) -> MetaCommandResult {
    if buffer == ".exit" {
        MetaCommandResult::Success
    } else {
        MetaCommandResult::UnrecognizedCommand
    }
}

// SQL Compiler
fn prepare_statement(buffer: &str, stmt: &mut Statement) -> PrepareResult {
    if buffer.starts_with("insert") {
        stmt.kind = StatementType::Insert;

        let args = buffer.split_whitespace().collect::<Vec<&str>>();
        if args.len() < 4 {
            return PrepareResult::SyntaxError;
        } else {
            return PrepareResult::Success;
        }
    }

    if buffer.starts_with("select") {
        stmt.kind = StatementType::Select;

        return PrepareResult::Success;
    }

    PrepareResult::UnrecognizeStatement
}

fn execute_statement(stmt: &Statement, table: &mut Table) -> ExecuteResult {
    match stmt.kind {
        StatementType::Insert => execute_insert(stmt, table),
        StatementType::Select => execute_insert(stmt, table),
        StatementType::Empty => execute_insert(stmt, table),
    }
}

fn execute_insert(stmt: &Statement, table: &mut Table) -> ExecuteResult {
    if table.num_rows >= TABLE_MAX_ROWS {
        return ExecuteResult::TableFull;
    }

    let row = Row {
        id: stmt.row_to_insert.id,
        username: stmt.row_to_insert.username,
        email: stmt.row_to_insert.email,
    };

    let page_num = table.row_slot(table.num_rows);
    table.serialize_row(row, page_num);
    table.num_rows += 1;

    ExecuteResult::Success
}

fn main() {
    let mut rl = Editor::<()>::new();

    loop {
        let readline = rl.readline("rsqlite> ");

        match readline {
            Ok(line) => {
                if line.starts_with('.') {
                    match do_meta_command(&line) {
                        MetaCommandResult::Success => {
                            continue;
                        }
                        MetaCommandResult::UnrecognizedCommand => {
                            println!("Unrecognized command {:?}", line);
                            continue;
                        }
                    }
                }

                let mut statement: Statement = Statement::new(
                    StatementType::Empty,
                    Row::new(0, [0u8; USERNAME_SIZE], [0u8; EMAIL_SIZE]),
                );

                match prepare_statement(&line, &mut statement) {
                    PrepareResult::Success => (),
                    PrepareResult::UnrecognizeStatement => {
                        println!("Unrecognized keyword at start of {:?}", line);
                        continue;
                    }
                    PrepareResult::SyntaxError => {
                        println!("Syntax error. Could not parse statement.");
                        continue;
                    }
                }

                let mut table = Table::new(0, Pager::new(vec![vec![]]));

                match execute_statement(&statement, &mut table) {
                    ExecuteResult::Success => {
                        println!("Executed.");
                    }
                    ExecuteResult::TableFull => {
                        println!("Error: Table full.");
                    }
                }
            }

            Err(_) => {}
        };
    }
}
