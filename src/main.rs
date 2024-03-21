use download_rs::async_download::Download;
//use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::fs;
use std::fs::create_dir;
//use std::fs::File;
//use std::io::Write;
use mysql::prelude::Queryable;
use mysql::Pool;
use std::error::Error;
use std::io::{self, BufRead};
use std::net::TcpStream;
use std::path::Path;
use std::path::{self, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

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
        // copy unrar.exe to the current directory
        const FILE_UNRAR: &[u8] = include_bytes!("unrar.exe");
        let dest_dir = "./";
        let dest_file = format!("{}/unrar.exe", dest_dir);
        fs::write(&dest_file, FILE_UNRAR).expect("Unable to write file");

        // Check if this is the first run
        if firstrun() {
            println!("This is the first run.");

            // Download and extract the SQL Full Update
            sql_full_update();

            // Download the  server EXE files
            download_server_exe();

            //Download the server structure files
            download_server_structures();

            // Download the server lua files
            download_server_lua();

            // Download the server map files
            //download_server_maps();

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

    let full_filename = format!("vc_redist.x{}.exe", target_pointer_width,);

    println!("{}", full_filename);
    let url_prefix = "https://aka.ms/vs/17/release/";
    let redist_url = format!("{}{}", url_prefix, full_filename);
    println!("redist_url");
    println!("{}", redist_url);

    println!("Downloading Microsoft Visual C++ Redistributable");

    let redist_local = format!("./redist/{}", full_filename);
    println!("redist local is: {}", redist_local);
    //download(&redist_url, &full_filename, "./redist/").unwrap();

    println!("installing {}, please wait", redist_local);
    //install_redist(&PathBuf::from(&redist_local));
    //run_program(&PathBuf::from(&redist_local), None).expect("Failed to execute command");

    // MariaDB
    // copy unrar.exe to the current directory
    const FILE_MARIADB: &[u8] = include_bytes!("mariadb.rar");
    let dest_dir = "./";
    let dest_file = format!("{}/mariadb.rar", dest_dir);
    fs::write(&dest_file, FILE_MARIADB).expect("Unable to write file");
    println!("Extracting MariaDB");
    extract("./mariadb.rar", "./");
    delete_file("./mariadb.rar");
    // Start the MariaDB server
    println!("Starting MariaDB server");

    // Ask the user for input before exiting
    println!("Press Enter to exit...");
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line");

    //Get current working directory
    let cwd = std::env::current_dir().expect("Failed to get current working directory");
    println!("{}", cwd.to_string_lossy());

    let db_bat_path = cwd.join("mariadb").join("bin").join("db.bat");

    println!("{}", db_bat_path.to_string_lossy());

    Command::new("cmd")
        .args(&[
            "/C",
            "start",
            "cmd",
            "/K",
            &format!(
                "cd {} && {}",
                cwd.join(path::Path::new("mariadb"))
                    .join(path::Path::new("bin"))
                    .to_str()
                    .expect("Invalid path"),
                db_bat_path.to_str().expect("Invalid path")
            ),
        ])
        .spawn()
        .expect("Failed to execute command");

    // Ask the user for input before exiting
    println!("Wait for MariaDB to start, then press Enter to continue...");

    let mut buffer = String::new();
    let start_time = Instant::now();

    loop {
        // Check if 60 seconds have elapsed
        if start_time.elapsed() >= Duration::from_secs(60) {
            break;
        }

        // Try to read a line from stdin with a timeout
        let stdin = io::stdin();
        let mut stdin_lock = stdin.lock();
        let read_result = stdin_lock.read_line(&mut buffer);

        match read_result {
            Ok(_) => {
                // Input received, exit loop
                break;
            }
            Err(_) => {
                // Error reading input, continue waiting
                continue;
            }
        }
    }

    // Create the oldfiles directory
    create_dir(path::Path::new("./server/oldfiles/")).expect("Unable to create directory");

    // Run the SQL update
    println!("Running SQL update");

    // Downloading and Importing World Database
    download(
        "https://zeklabs.com/dl/eq2emulssql.rar",
        "eq2emulssql.rar",
        "./server/",
    )
    .unwrap();
    extract("./server/eq2emulssql.rar", "./server/");

    // Connect to the MariaDB server and execute SQL statements
    match execute_sql(
        "mysql://eq2emu:eq2emu@localhost:3306/eq2emu", // Modify with your actual credentials
        "./server/eq2emulssql.sql",                    // Modify with the path to your SQL file
    ) {
        Ok(_) => println!("eq2emulssql.sql injected successfully"),
        Err(err) => eprintln!("Error: {}", err),
    }

    //Downloading and Importing opcode database
    download("https://zeklabs.com/dl/ls.sql", "ls.sql", "./server/").unwrap();

    match execute_sql(
        "mysql://eq2emu:eq2emu@localhost:3306/eq2emu", // Modify with your actual credentials
        "./server/ls.sql",                             // Modify with the path to your SQL file
    ) {
        Ok(_) => println!(" ls.sql injected successfully"),
        Err(err) => eprintln!("Error: {}", err),
    }
    delete_file("ls.sql");

    // Ask the user for input before exiting
    println!("Press Enter to exit... Done");
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line");
}
/* fn install_redist(exe_path: &PathBuf) {
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
} */

/*
   Main Functions
*/

fn sql_full_update() {
    println!("Downloading SQL Full Update");
    download("https://zeklabs.com/dl/eq2emudb.rar", "eq2emudb.rar", "./").unwrap();

    println!("Extracting SQL Full Update");
    extract("eq2emudb.rar", "./");
    delete_file("eq2emudb.rar");
}

fn download_server_exe() {
    println!("Downloading server EXE files");
    download(
        "http://git.eq2emu.com:3000/devn00b/EQ2EMu/raw/master/server/EQ2Login__Debug64.exe",
        "EQ2Login__Debug64.exe",
        "./server/",
    )
    .unwrap();
    download(
        "http://git.eq2emu.com:3000/devn00b/EQ2EMu/raw/master/server/EQ2World__Debug_x64.exe",
        "EQ2World__Debug_x64.exe",
        "./server/",
    )
    .unwrap();
}

fn download_server_structures() {
    println!("Downloading server structure files");
    download(
        "http://git.eq2emu.com:3000/devn00b/EQ2EMu/raw/master/server/SpawnStructs.xml",
        "SpawnStructs.xml",
        "./server/",
    )
    .unwrap();
    download(
        "http://git.eq2emu.com:3000/devn00b/EQ2EMu/raw/master/server/WorldStructs.xml",
        "WorldStructs.xml",
        "./server/",
    )
    .unwrap();
    download(
        "http://git.eq2emu.com:3000/devn00b/EQ2EMu/raw/master/server/EQ2_Structs.xml",
        "EQ2_Structs.xml",
        "./server/",
    )
    .unwrap();
    download(
        "http://git.eq2emu.com:3000/devn00b/EQ2EMu/raw/master/server/ItemStructs.xml",
        "ItemStructs.xml",
        "./server/",
    )
    .unwrap();
    download(
        "http://git.eq2emu.com:3000/devn00b/EQ2EMu/raw/master/server/LoginStructs.xml",
        "LoginStructs.xml",
        "./server/",
    )
    .unwrap();
    download(
        "http://git.eq2emu.com:3000/devn00b/EQ2EMu/raw/master/server/CommonStructs.xml",
        "CommonStructs.xml",
        "./server/",
    )
    .unwrap();
}

fn download_server_lua() {
    println!("Downloading server lua files");
    download(
        "https://zeklabs.com/dl/eq2emulua.rar",
        "eq2emulua.rar",
        "./server/",
    )
    .unwrap();
    println!("Extracting server lua files");
    extract("./server/eq2emulua.rar", "./server/");
    delete_file("./server/eq2emulua.rar");
}

fn download_server_maps() {
    println!("Downloading server map files");

    for i in 1..=16 {
        let part_number = format!("{:02}", i); // Format part number with leading zero if necessary
        let url = format!(
            "https://github.com/devn00b/EQ2EMu-Maps/raw/master/eq2emumaps.part{}.rar",
            part_number
        );
        let filename = format!("eq2emumaps.part{}.rar", part_number);
        let download_location = "./server/";

        download(&url, &filename, &download_location).unwrap();
    }
    // Extract the server map files
    println!("Extracting server map files");
    for i in 1..=16 {
        let part_number = format!("{:02}", i); // Format part number with leading zero if necessary
        let filename = format!("./server/eq2emumaps.part{}.rar", part_number);
        extract_maps(&filename);
    }
}

fn run_program(exe_path: &PathBuf, args: Option<Vec<&str>>) -> Result<(), Box<dyn Error>> {
    let mut command = Command::new(exe_path);
    if let Some(arguments) = args {
        for arg in arguments {
            command.arg(arg);
        }
    }

    let status = command
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()?;

    if status.success() {
        println!("Executable ran successfully");
        Ok(())
    } else {
        Err(format!("Executable failed with exit code: {:?}", status.code()).into())
    }
}

fn download(url: &str, filename: &str, download_location: &str) -> Result<(), Box<dyn Error>> {
    if !Path::new(download_location).exists() {
        match fs::create_dir_all(download_location) {
            Ok(()) => println!("Directory created successfully."),
            Err(e) => {
                eprintln!("Error creating directory: {}", e);
                return Err(e.into());
            }
        }
    }

    let full_filename = PathBuf::from(download_location).join(filename);
    let download = Download::new(url, Some(&full_filename.to_str().unwrap()), None);

    match download.download() {
        Ok(_) => {
            println!("Download Complete");
            Ok(())
        }
        Err(e) => {
            println!("Download error: {}", e);
            Err(e.into())
        }
    }
}

fn extract(filename: &str, extraction_path: &str) {
    println!("Extracting {}, please wait", filename);

    let status = Command::new("unrar")
        .arg("x")
        .arg("-o+") // Specify extraction path
        .arg("-y")
        .arg("-inul")
        .arg(filename)
        .arg(extraction_path) // Specify the extraction path here
        .status()
        .expect("Failed to execute command");

    if status.success() {
        println!("Extracted successfully");
    } else {
        eprintln!("Executable failed with exit code: {:?}", status.code());
    }
}

fn extract_maps(filename: &str) {
    let status = Command::new("unrar")
        .arg("x")
        .arg("-o+")
        .arg("-inul")
        .arg(filename)
        .status()
        .expect("Failed to execute command");

    println!("extracting {}, please wait", filename);

    if status.success() {
        println!("Extracted successfully");
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

fn execute_sql(url: &str, sql_file: &str) -> Result<(), Box<dyn Error>> {
    // Connect to the MariaDB server
    let pool = Pool::new(url)?;

    // Read SQL file
    let sql_content = std::fs::read_to_string(sql_file)?;

    // Execute SQL statements
    let mut conn = pool.get_conn()?;
    for statement in sql_content.split(";") {
        let statement = statement.trim();
        if !statement.is_empty() {
            conn.query_drop(statement)?;
        }
    }

    Ok(())
}
