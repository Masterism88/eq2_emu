use std::fs;
use std::io;
use std::net::TcpStream;
use std::path;

const LOGO: &str = r#"

    88888888888  ,ad8888ba,     ad888888b,  88888888888  88b           d88               
    88          d8"'    `"8b   d8"     "88  88           888b         d888               
    88         d8'        `8b          a8P  88           88`8b       d8'88               
    88aaaaa    88          88       ,d8P"   88aaaaa      88 `8b     d8' 88  88       88  
    88"""""    88          88     a8P"      88"""""      88  `8b   d8'  88  88       88  
    88         Y8,    "88,,8P   a8P'        88           88   `8b d8'   88  88       88  
    88          Y8a.    Y88P   d8"          88           88    `888'    88  "8a,   ,a88  
    88888888888  `"Y8888Y"Y8a  88888888888  88888888888  88     `8'     88   `"YbbdP'Y8  
       Windows Installer Rust version by Masterism 
           Based off of the the original by Devn00b https://www.eq2emu.com
    "#;

fn main() {
    let server_name: &str = r#"Zeklabs.com"#;
    let file_path = path::Path::new("settings.txt");
    const CONFIG_FILE: &str = "/settings.";

    println!("{}", LOGO);

    println!("Server Name: {}", server_name);

    //Check if config file exists, if not create it
    if file_path.exists() {
        println!("File exists");
    } else {
        // Create the file using `fs::File::create()`
        match fs::File::create(&file_path) {
            Ok(_) => println!("File created successfully"),
            Err(e) => println!("Error creating file: {}", e),
        }
    }

    // Check for internet access
    const ZEKLABS: &str = r#"Zeklabs.com"#;

    let reachable = is_reachable(ZEKLABS);

    if reachable {
        println!("{} is reachable", ZEKLABS);
    } else {
        println!("{} is not reachable", ZEKLABS);
    }
}

fn is_reachable(domain_name: &str) -> bool {
    let domain = format!("{}:80", domain_name); // Use a common port like 80
    match TcpStream::connect(domain) {
        Ok(_) => true,   // Connection successful
        Err(_) => false, // Connection failed
    }
}
