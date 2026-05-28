use crate::db::engine::Database;
use crate::query_parser::actions::Command;

pub fn execute(command: Command, db: &mut Database) {  
    match command {
        Command::Insert { key, value } => {
            db.put(&key, &value).expect("Failed to execute INSERT");
            println!("Inserted: {} -> {}", key, value);
        },
        Command::Get { key } => {
            match db.get(&key).expect("Failed to execute GET") {
                Some(value) => println!("{} => {}", key, value),
                None => println!("{} not found", key),
            }
        },
        Command::Delete { key } => {
            db.delete(&key).expect("Failed to execute DELETE");
            println!("Deleted: {}", key);
        },
        Command::Exit => {
            std::process::exit(0);
        }
    }
}