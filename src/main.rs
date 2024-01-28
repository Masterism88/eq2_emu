use download_rs::async_download::Download;
//use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::fs;
use std::fs::File;
use std::io;
use std::io::Write;
use std::net::TcpStream;
use std::path::Path;
use std::path::{self, PathBuf};
use std::process::Command;

const LOGO: &str = r#"

    88888888888  ,ad8888ba,     ad888888b,  88888888888  88b           d88               
    88          d8"'    `"8b   d8"     "88  88           888b         d888               
    88         d8'        `8b          a8P  88           88`8b       d8'88               
    88aaaaa    88          88       ,d8P"   88aaaaa      88 `8b     d8' 88  88       88  
    88"""""    88          88     a8P"      88"""""      88  `8b   d8'  88  88       88  
    88         Y8,    "88,,8P   a8P'        88           88   `8b d8'   88  88       88  
    88          Y8a.    Y88P   d8"          88           88    `888'    88  "8a,   ,a88  
    88888888888  `"Y8888Y"Y8a  88888888888  88888888888  88     `8'     88   `"YbbdP'Y8  
       EQ2EMu Installer Rust version by Masterism 
           Based off of the original by Devn00b https://www.eq2emu.com https://discord.gg/5Cavm9NYQf
    "#;

#[derive(Serialize, Deserialize)]
struct Updates {
    firstrun: i32,
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

    println!("{}", LOGO);

    println!("Server Name: {}", server_name);

    let update = Updates {
        firstrun: 1,
        sqlupdate: 1,
        exeupdate: 1,
        luaupdate: 1,
        mapupdate: 1,
        sendbugs: 1,
        lsupdate: 1,
        updatels: 1,
        dbeditor: 1,
    };

    // Check for internet access
    const ZEKLABS: &str = r#"Zeklabs.com"#;

    let reachable: bool = is_reachable(ZEKLABS);

    if reachable {
        println!("{}", format!("{} is reachable", ZEKLABS));

        // Serialize the data to a JSON string
        let json_data = serde_json::to_string(&update).expect("Failed to serialize to JSON");

        // Check if config file exists, if not create it
        let file_path = path::Path::new("config.json");
        println!("Creating Config File");
        if file_path.exists() {
            println!("Config file exists");
        } else {
            // Create the file using `fs::File::create()`
            match fs::File::create(&file_path) {
                Ok(_) => println!("Config file created successfully"),
                Err(e) => println!("Error creating file: {}", e),
            }
            fs::write(file_path, json_data.as_bytes()).expect("Unable to write file");
        }

        if firstrun() {
            println!("This is the first run.");
            println!("Downloading SQL Full Update");
            download("https://zeklabs.com/dl/eq2emudb.rar", "eq2emudb.rar", "./");

            println!("Extracting SQL Full Update");
            extract("eq2emudb.rar");
            delete_file("eq2emudb.rar");

            // Check type of OS
            match std::env::consts::OS {
                "linux" => linux(),
                "windows" => windows(),
                _ => println!("Running on an unknown operating system"),
            }
        } else {
            println!("This is not the first run.");
        }
    } else {
        println!(
            "{}",
            format!("{} is not reachable. Cannot continue.", ZEKLABS)
        );
    }
    // Ask the user for input before exiting
    println!("Press Enter to exit...");
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line");
}

fn is_reachable(domain_name: &str) -> bool {
    let domain = format!("{}:80", domain_name); // Use a common port like 80
    match TcpStream::connect(domain) {
        Ok(_) => true,   // Connection successful
        Err(_) => false, // Connection failed
    }
}

fn linux() {
    println!("Linux Detected");

    /*
       TODO: Check if the user has unrar installed, if not install it
    */
}

fn windows() {
    println!("Windows Detected");

    // Detect if OS is 32bit or 64 bit
    println!("Detecting if OS is 32bit or 64 bit");

    let target_pointer_width = if cfg!(target_pointer_width = "32") {
        println!("32 bit OS detected");
        "32"
    } else {
        println!("64 bit OS detected");
        "64"
    }
    .to_string();

    //Get current working directory
    let cwd = std::env::current_dir().expect("Failed to get current working directory");
    println!("{}", cwd.to_string_lossy());

    let full_filename = format!("vc_redist.x{}.exe", target_pointer_width,);

    println!("{}", full_filename);
    let url_prefix = "https://aka.ms/vs/17/release/";
    let redist_url = format!("{}{}", url_prefix, full_filename);
    println!("redist_url");
    println!("{}", redist_url);

    println!("This is the first run. Windows");

    println!("Downloading Microsoft Visual C++ Redistributable");

    let redist_local = format!("./redist/{}", full_filename);
    println!("redist local is: {}", redist_local);
    download(&redist_url, &full_filename, "./redist/");

    println!("installing {}, please wait", redist_local);
    // Ask the user for input before exiting
    println!("Press Enter to exit...");
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line");

    install_redist(&PathBuf::from(&redist_local));
}
fn install_redist(exe_path: &PathBuf) {
    let status = Command::new(exe_path)
        //.arg("/q")
        //.arg("/norestart")
        .status()
        .expect("Failed to execute command");

    if status.success() {
        println!("Executable ran successfully");
    } else {
        eprintln!("Executable failed with exit code: {:?}", status.code());
    }
}

fn download(url: &str, filename: &str, download_location: &str) {
    if !Path::new(download_location).exists() {
        match fs::create_dir_all(download_location) {
            Ok(()) => println!("Directory created successfully."),
            Err(e) => {
                eprintln!("Error creating directory: {}", e);
                return;
            }
        }
    }
    let full_filename = PathBuf::from(download_location).join(filename);
    let download = Download::new(url, Some(&full_filename.to_str().unwrap()), None);

    match download.download() {
        Ok(_) => println!("Download Complete"),
        Err(e) => println!("Download error: {}", e.to_string()),
    }
}

fn extract(filename: &str) {
    let status = Command::new("unrar")
        .arg("x")
        .arg("-y")
        .arg("-inul")
        .arg(filename)
        .status()
        .expect("Failed to execute command");

    println!("extracting {}, please wait", filename);

    if status.success() {
        println!("Executable ran successfully");
    } else {
        eprintln!("Executable failed with exit code: {:?}", status.code());
    }
}

fn firstrun() -> bool {
    // Read the JSON data from the file
    let json_data = fs::read_to_string("config.json").expect("Failed to read file");

    // Parse the JSON data
    let mut data: Value = serde_json::from_str(&json_data).expect("Failed to parse JSON");

    // Extract the value of "firstrun" field
    let firstrun = data["firstrun"].as_u64().unwrap_or(0);
    // Check the value of "firstrun" and perform actions accordingly
    if firstrun == 1 {
        println!("First run");
        // Update "firstrun" field to 2
        data["firstrun"] = json!(2);
        // Write the updated JSON data back to the file
        fs::write("config.json", serde_json::to_string_pretty(&data).unwrap())
            .expect("Failed to write file");
        true
    } else if firstrun == 2 {
        println!("Performing action for firstrun == 2");
        // Do something else
        false
    } else {
        println!("Unknown value for firstrun");
        false
    }
}

fn delete_file(filename: &str) {
    if Path::new(filename).exists() {
        match fs::remove_file(filename) {
            Ok(_) => println!("{} deleted successfully", filename),
            Err(e) => println!("Error deleting file: {}", e),
        }
    } else {
        println!("File does not exist");
    }
}
