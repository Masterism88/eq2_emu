use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io;
use std::io::Write;
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

#[derive(Serialize, Deserialize)]
struct Updates {
    sqlupdate: i32,
    exeupdate: i32,
    luaupdate: i32,
    mapupdate: i32,
    sendbugs: i32,
    lsupdate: i32,
    updatels: i32,
    dbeditor: i32,
}
fn main() {
    let server_name: &str = r#"Zeklabs.com"#;
    let file_path = path::Path::new("config.json");

    println!("{}", LOGO);

    println!("Server Name: {}", server_name);

    // Check type of OS
    match std::env::consts::OS {
        "linux" => linux(),
        "windows" => windows(),
        _ => println!("Running on an unknown operating system"),
    }

    let update = Updates {
        sqlupdate: 1,
        exeupdate: 1,
        luaupdate: 1,
        mapupdate: 1,
        sendbugs: 1,
        lsupdate: 1,
        updatels: 1,
        dbeditor: 1,
    };
    // Serialize the data to a JSON string
    let json_data = serde_json::to_string(&update).expect("Failed to serialize to JSON");

    // Check if config file exists, if not create it
    if file_path.exists() {
        println!("Config file exists");
        fs::write(file_path, json_data.as_bytes()).expect("Unable to write file");
    } else {
        // Create the file using `fs::File::create()`
        match fs::File::create(&file_path) {
            Ok(_) => println!("Config file created successfully"),
            Err(e) => println!("Error creating file: {}", e),
        }
        fs::write(file_path, json_data.as_bytes()).expect("Unable to write file");
    }
}

fn is_reachable(domain_name: &str) -> bool {
    let domain = format!("{}:80", domain_name); // Use a common port like 80
    match TcpStream::connect(domain) {
        Ok(_) => true,   // Connection successful
        Err(_) => false, // Connection failed
    }
}

fn linux() {
    println!("Running on Linux");

    // Check for internet access
    const ZEKLABS: &str = r#"Zeklabs.com"#;
    let reachable: bool = is_reachable(ZEKLABS);

    if reachable {
        println!("{}", format!("{} is reachable", ZEKLABS).green());
    } else {
        println!("{}", format!("{} is not reachable", ZEKLABS).red());
    }
}

fn windows() {
    println!("Running on Windows");

    // Check for internet access
    const ZEKLABS: &str = r#"Zeklabs.com"#;
    let reachable: bool = is_reachable(ZEKLABS);

    if reachable {
        println!("{} is not reachable", ZEKLABS);
    } else {
        println!("{} is not reachable", ZEKLABS);
    }

    // Ask the user for input before exiting
    println!("Press Enter to exit...");
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line");
}
