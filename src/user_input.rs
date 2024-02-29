/* Creator: Lucas Huber
 * Date: 2024-02-20
 */

pub trait Parser {
    fn parse(data: &String) -> Self;
}

pub enum InputCommand {
    Help,
    List,
    Binary,
    Ascii,
    Get(String),
    Quit,
    UnknownCommand,
    NotEnoughCommands,
    TooManyCommands,
    Empty,
}

impl Parser for InputCommand {
    fn parse(data: &String) -> InputCommand {
        let elements = data.split_whitespace().collect::<Vec<&str>>();

        match elements.len() {
            0 => InputCommand::Empty,
            1 => match elements[0] {
                "help" => InputCommand::Help,
                "list" => InputCommand::List,
                "binary" => InputCommand::Binary,
                "ascii" => InputCommand::Ascii,
                "get" => InputCommand::NotEnoughCommands,
                "quit" => InputCommand::Quit,
                _ => InputCommand::UnknownCommand,
            },
            2 => match elements[0] {
                "get" => InputCommand::Get(elements[1].to_string()),
                "help" | "list" | "binary" | "ascii" | "quit" => InputCommand::TooManyCommands,
                _ => InputCommand::UnknownCommand,
            },
            _ => match elements[0] {
                "help" | "list" | "binary" | "ascii" | "get" | "quit" => InputCommand::TooManyCommands,
                _ => InputCommand::UnknownCommand,
            },
        }
    }
}

pub fn get_user_input() -> String {
    let mut input = String::new();

    std::io::stdin().read_line(&mut input).unwrap();

    input.trim().to_string()
}
