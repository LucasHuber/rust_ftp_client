/* Creator: Lucas Huber
 * Date: 2024-02-20
 */

mod ftp_client;
mod user_input;

use crate::ftp_client::{init};
use crate::user_input::{get_user_input, Parser, InputCommand};

fn main() {
    let mut client = init();

    let mut stopped = false;

    while !stopped {
        println!("------------------------------");
        println!("Next command:");
        let input = get_user_input();

        match InputCommand::parse(&input) {
            InputCommand::Help => {
                println!("Available Commands:");
                println!("help: \t Lists all available commands");
                println!("list: \t Lists the available files on the FTP server");
                println!("binary: \t Switches FTP transfer mode to binary");
                println!("ascii: \t Switches FTP transfer mode to ascii");
                println!("get %filename%: \t Gets the mentioned file from the FTP server");
                println!("quit: \t Quits the connection to the FTP server and closes the program");
            },
            InputCommand::List => client.list(),
            InputCommand::Binary => client.binary(),
            InputCommand::Ascii => client.ascii(),
            InputCommand::Get(filename) => client.get(&filename),
            InputCommand::Quit => {
                client.quit();
                stopped = true;
            },
            InputCommand::UnknownCommand => println!("Unknown command - use 'help'"),
            InputCommand::NotEnoughCommands => println!("Not enough Arguments listed for this command - use 'help'"),
            InputCommand::TooManyCommands => println!("Too many Arguments listed for this command - use 'help'"),
            InputCommand::Empty => (),
        }
    }

    println!("------------------------------");
    println!("Thanks for using this FTP Client");
}