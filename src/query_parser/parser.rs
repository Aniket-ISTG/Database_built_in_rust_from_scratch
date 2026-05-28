use crate::query_parser::actions::Command;



pub fn parse(input : &str) -> Option<Command> {
    let tokens : Vec<&str> = input.trim().split_whitespace().collect();
    if tokens.is_empty() {
        return None;
    }

    match tokens[0].to_uppercase().as_str() {
        "INSERT" => {
            if tokens.len() < 3 {
                return None; // Not enough arguments
            }
            Some(Command::Insert { key: tokens[1].to_string(), value: tokens[2..].join(" ") })
        },
        "GET" => {
            if tokens.len() != 2 {
                return None; // Invalid number of arguments
            }
            Some(Command::Get { key: tokens[1].to_string() })
        },
        "DELETE" => {
            if tokens.len() != 2 {
                return None; // Invalid number of arguments
            }
            Some(Command::Delete { key: tokens[1].to_string() })
        },
        "EXIT" => {
            Some(Command::Exit)
        },
        _ => None, // Unknown command
    }
}