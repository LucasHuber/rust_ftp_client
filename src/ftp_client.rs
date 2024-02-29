/* Creator: Lucas Huber
 * Date: 2024-02-20
 */

use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;

struct FtpResponse {
    response_code: u16,
    response_message: String,
}

pub struct FTPClient {
    control_stream: TcpStream,
    data_stream: Option<TcpStream>,
}

impl FTPClient {
    pub fn new(host: &str) -> Self {
        let mut stream = TcpStream::connect(format!("{}", host)).unwrap_or_else(|_| {
            println!("Unable to connect to FTP Server ({})", host);
            std::process::exit(1);
        });
        get_response(&mut stream, true);

        FTPClient { control_stream: stream, data_stream: None }
    }

    fn send_command(&mut self, cmd: &str) {
        let mut stream = &self.control_stream;
        stream.write_all(cmd.as_bytes()).unwrap();
        std::thread::sleep(Duration::from_millis(10));
    }

    pub fn login(&mut self, username: &str, password: &str) {
        self.send_command(&format!("USER {}\r\n", username));
        get_response(&mut self.control_stream, true);
        self.send_command(&format!("PASS {}\r\n", password));
        let login_return_code = get_response(&mut self.control_stream, true).response_code;
        
        if login_return_code >= 400 {
            println!("Login has failed, please check username and password again");
            std::process::exit(1);
        }
    }

    fn enter_passive_mode(&mut self) {
        self.send_command(&"PASV\r\n".to_string());
        let response = get_response(&mut self.control_stream, true);

        if response.response_code >= 400 {
            println!("Unable to enter Passive Mode");
            std::process::exit(1);
        }

        let pasv_port = self.get_pasv_port(response.response_message);

        let stream = TcpStream::connect(format!("127.0.0.1:{}", pasv_port)).unwrap_or_else(|_| {
            println!("Unable to establish Data Stream connection");
            std::process::exit(1);
        });

        self.data_stream = Some(stream);
    }

    pub fn list(&mut self) {
        self.enter_passive_mode();
        self.send_command("LIST\r\n");
        let response = get_response(&mut self.control_stream, true);

        if response.response_code >= 400 {
            println!("Unable to list FTP Server contents");
            std::process::exit(1);
        }

        let response = get_response(&mut self.data_stream.as_mut().unwrap(), false);

        println!("FTP content list:");
        println!("{}", response.response_message);

        self.quit_data_connection();
    }

    pub fn get(&mut self, filename: &str) {
        self.enter_passive_mode();
        self.send_command(&format!("RETR {}\r\n", filename));
        let response = get_response(&mut self.control_stream, true);

        if response.response_code >= 400 {
            println!("Unable to get content from FTP Server");
            std::process::exit(1);
        }

        let response = get_response(&mut self.data_stream.as_mut().unwrap(), false);

        println!("File content:");
        println!("'''");
        println!("{}", response.response_message);
        println!("'''");

        self.quit_data_connection();
    }

    pub fn ascii(&mut self) {
        self.send_command("TYPE A\r\n");
        let response = get_response(&mut self.control_stream, true);

        if response.response_code >= 400 {
            println!("Unable to change mode to ASCII");
            std::process::exit(1);
        }
    }

    pub fn binary(&mut self) {
        self.send_command("TYPE I\r\n");
        let response = get_response(&mut self.control_stream, true);

        if response.response_code >= 400 {
            println!("Unable to change mode to BINARY");
            std::process::exit(1);
        }
    }

    pub fn quit(&mut self) {
        self.send_command("QUIT\r\n");
        let response = get_response(&mut self.control_stream, true);

        if response.response_code >= 400 {
            println!("Unable to quit FTP connection");
            std::process::exit(1);
        }
    }

    fn quit_data_connection(&mut self) {
        self.data_stream.as_mut().unwrap().shutdown(std::net::Shutdown::Both).unwrap();
        self.data_stream = None;
    }

    fn get_pasv_port(&mut self, response: String) -> u16 {
        let response_ip_values: Vec<&str> = response.split("(").collect();
        let response_ip_values: Vec<&str> = response_ip_values[1].split(")").collect();
        let response_ip_values: Vec<&str> = response_ip_values[0].split(",").collect();

        let port1 = response_ip_values[4].parse::<u16>().unwrap_or_else(|err| {
            println!(
                "Failed to parse PASV port value: {} Error: {}",
                response_ip_values[4], err
            );
            std::process::exit(1);
        });

        let port2 = response_ip_values[5].parse::<u16>().unwrap_or_else(|err| {
            println!(
                "Failed to parse PASV port value: {} Error: {}",
                response_ip_values[5], err
            );
            std::process::exit(1);
        });

        (port1 << 8) + port2
    }
}

fn get_response(socket: &mut TcpStream, print_response: bool) -> FtpResponse {
    let mut raw_data = Vec::new();
    let mut buf = vec![0; 1024];

    let mut read = buf.len();

    while read == buf.len() {
        read = socket.read(&mut buf).unwrap_or_else(|_| {
            println!("Reading from socket has been interrupted! Restart the program.");
            std::process::exit(1);
        });
        // read until EOF
        raw_data.extend_from_slice(&buf[..read]);
    }

    let response = String::from_utf8(raw_data).unwrap();
    if print_response {
        println!("{}", response);
    }

    let mut return_code = 0;

    let elements = response.split_whitespace().collect::<Vec<&str>>();
    if elements[0].parse::<u16>().is_ok() {
        return_code = elements[0].parse::<u16>().unwrap();
    }

    FtpResponse {response_code: return_code, response_message: response}
}

pub fn init() -> FTPClient {
    let username = "user";
    let password = "test123";
    let ip_address = "127.0.0.1";
    let port = "21";
    let ftp_address = format!("{}:{}", ip_address, port);

    println!("This is a FTP Client which connects to {}@{} using the password '{}'.", username, ftp_address, password);
    println!("Settings can only be changed in code.");
    println!("Too see available commands use 'help'");
    println!();


    let mut client = FTPClient::new(&ftp_address);
    client.login(&username, &password);

    client
}