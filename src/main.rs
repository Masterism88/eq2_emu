use download_rs::async_download::Download;
use serde::de;
//use owo_colors::OwoColorize;  // Linux only? Sadface
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::fs;
use std::fs::create_dir;
//use std::fs::File;
//use std::io::Write;
use mysql::prelude::*;
use mysql::Pool;
use mysql::*;
use std::env;
use std::error::Error;
use std::fs::rename;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{self, BufRead, Read, Write};
use std::net::TcpStream;
use std::path::Path;
use std::path::{self, PathBuf};
use std::process;
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;
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

const MENU: &str = r#"
    1. Run Full SQL Update
    2. Run Partial SQL Update
    3. Update server EXE files
    4. Update server lua files
    5. Update server map files
    6. Start the MariaDB server
    7. Run the SQL update
    8. Download and Import the loginserver database
    9. Update the loginserver database
    10. Upload bug report
    11. Changelog
    12. Exit
    "#;

// Maybe?
const CHANGELOG: &str = r#"
    1.0.0
    - Initial release
    "#;

fn main() {
    let server_name: &str = r#"Zeklabs.com"#;

    println!("{}", LOGO);

    println!("Server Name: {}", server_name);

    // Check for internet access
    const ZEKLABS: &str = r#"Zeklabs.com"#;

    let reachable: bool = is_reachable(ZEKLABS);

    if reachable {
        println!("{}", format!("{} is reachable", ZEKLABS));

        // Check type of OS
        match std::env::consts::OS {
            "linux" => linux(),
            "windows" => windows(),
            _ => println!("Running on an unknown operating system"),
        }
    } else {
        println!(
            "{}",
            format!("{} is not reachable. Cannot get updates.", ZEKLABS)
        );
    }
}

fn linux() {
    println!("Linux Detected");

    /*
       TODO: Check if the user has unrar installed, if not install it

       TODO: check if the user has MariaDB installed, if not install it
    */
}

fn windows() {
    println!("Windows Detected");

    // Check if this is the first run
    if firstrun() {
        println!("This is the first run.");

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
        download(&redist_url, &full_filename, "./redist/").unwrap();

        println!("installing {}, please wait", redist_local);
        install_redist(&PathBuf::from(&redist_local));
        //run_program(&PathBuf::from(&redist_local), None).expect("Failed to execute command");

        // copy unrar.exe to the current directory
        const FILE_UNRAR: &[u8] = include_bytes!("unrar.exe");
        let dest_dir = "./";
        let dest_file = format!("{}/unrar.exe", dest_dir);
        fs::write(&dest_file, FILE_UNRAR).expect("Unable to write file");

        // copy fart.exe to the current directory
        /*  const FILE_FART: &[u8] = include_bytes!("fart.exe");
        let fart_dir = "./";
        let fart_file = format!("{}/fart.exe", fart_dir);
        fs::write(&fart_file, FILE_FART).expect("Unable to write file"); */

        // MariaDB
        // copy mariadb.rar to the current directory
        const FILE_MARIADB: &[u8] = include_bytes!("mariadb.rar");
        let dest_dir = "./";
        let dest_file = format!("{}/mariadb.rar", dest_dir);
        fs::write(&dest_file, FILE_MARIADB).expect("Unable to write file");
        println!("Extracting MariaDB");
        extract("./mariadb.rar", "./");
        delete_file("./mariadb.rar");
        // Start the MariaDB server
        println!("Starting MariaDB server");

        start_mariadb();

        // Download and extract the SQL Full Update
        sql_full_update();

        // Download the  server EXE files
        download_server_exe();

        // Download the server lua files
        download_server_lua();

        // Download the server map files
        //download_server_maps();

        // Run the SQL update
        println!("Running SQL update");

        // Downloading and Importing the loginserver database
        import_loginserver_database();

        //updating the loginserver database
        update_loginserver_database();

        sync_login_to_world();

        menu();
    } else {
        println!("This is not the first run.");
        menu();
    }
}

fn is_reachable(domain_name: &str) -> bool {
    let domain = format!("{}:80", domain_name); // Use a common port like 80
    match TcpStream::connect(domain) {
        Ok(_) => true,   // Connection successful
        Err(_) => false, // Connection failed
    }
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

/*
Main Functions
*/

fn sql_full_update() {
    println!("Downloading SQL Full Update");
    download("https://zeklabs.com/dl/eq2emudb.rar", "eq2emudb.rar", "./").unwrap();
    extract("eq2emudb.rar", "./");
    delete_file("eq2emudb.rar");

    match execute_sql(
        "mysql://eq2emu:eq2emu@localhost:3306/eq2emu", // Modify with your actual credentials
        "./server/eq2emu.sql",                         // Modify with the path to your SQL file
    ) {
        Ok(_) => println!("eq2emu.sql injected successfully"),
        Err(err) => eprintln!("Error: {}", err),
    }
    delete_file("./server/eq2emu.sql");
}

fn sql_partial_update() {
    println!("Downloading SQL Partial Update");
    download(
        "https://zeklabs.com/dl/eq2dbupdate.rar",
        "eq2dbupdate.rar",
        "./",
    )
    .unwrap();
    extract("eq2dbupdate.rar", "./");
    delete_file("eq2dbupdate.rar");

    match execute_sql(
        "mysql://eq2emu:eq2emu@localhost:3306/eq2emu", // Modify with your actual credentials
        "./eq2dbupdate.sql",                           // Modify with the path to your SQL file
    ) {
        Ok(_) => println!("eq2emu.sql injected successfully"),
        Err(err) => eprintln!("Error: {}", err),
    }
    delete_file("./eq2dbupdate.sql");
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

    //Download the server structure files
    println!("Downloading server structure files");
    download_server_structures();
}

fn update_exe() {
    println!("Downloading new server EXE files");

    let source_patterns = vec![
        "./server/SpawnStructs.xml",
        "./server/WorldStructs.xml",
        "./server/EQ2_Structs.xml",
        "./server/ItemStructs.xml",
        "./server/LoginStructs.xml",
        "./server/CommonStructs.xml",
        "./server/EQ2Login__Debug64.exe",
        "./server/EQ2World__Debug_x64.exe",
    ];

    let destination_directory = "./server/oldfiles";

    match move_files(&source_patterns, destination_directory) {
        Ok(_) => println!("Files moved successfully"),
        Err(err) => eprintln!("Could not move {:?}. Error: {}", &source_patterns, err),
    }

    download_server_exe();
    download_server_structures();
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

    // Delete the downloaded RAR files
    for i in 1..=16 {
        let part_number = format!("{:02}", i); // Format part number with leading zero if necessary
        let filename = format!("./server/eq2emumaps.part{}.rar", part_number);
        delete_file(&filename);
    }
}

fn start_mariadb() {
    // Get current working directory
    let cwd = std::env::current_dir().expect("Failed to get current working directory");
    println!("{}", cwd.to_string_lossy());

    let db_bat_path = cwd.join("mariadb").join("bin").join("db.bat");
    println!("{}", db_bat_path.to_string_lossy());

    // Shared boolean flag to indicate whether user input is received
    let user_input_received = Arc::new(Mutex::new(false));

    // Spawn a separate thread to wait for user input
    let join_handle = {
        let user_input_received = Arc::clone(&user_input_received);
        thread::spawn(move || {
            println!("Wait for MariaDB to start, then press Enter to continue...");
            let mut input = String::new();
            match std::io::stdin().read_line(&mut input) {
                Ok(_) => {
                    let mut flag = user_input_received.lock().unwrap();
                    *flag = true; // Set the flag to true indicating user input is received
                }
                Err(e) => eprintln!("Error reading input: {}", e),
            }
        })
    };

    // Execute the command
    Command::new("cmd")
        .args(&[
            "/C",
            "start",
            "cmd",
            "/K",
            &format!(
                "cd {} && {}",
                cwd.join("mariadb").join("bin").display(),
                db_bat_path.display()
            ),
        ])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("Failed to execute command");

    // Wait for user input or timeout (60 seconds)
    let timeout_duration = Duration::from_secs(60);
    let start_time = Instant::now();
    loop {
        if start_time.elapsed() >= timeout_duration {
            println!("Timeout reached, continuing...");
            break;
        }
        if *user_input_received.lock().unwrap() {
            println!("Continuing...");
            break;
        }
        thread::sleep(Duration::from_millis(100)); // Sleep for a short duration to avoid busy waiting
    }

    // Wait for the user input thread to finish
    join_handle.join().unwrap();
}

fn import_loginserver_database() {
    download(
        "https://zeklabs.com/dl/eq2emulssql.rar",
        "eq2emulssql.rar",
        "./server/",
    )
    .unwrap();
    extract("./server/eq2emulssql.rar", "./server/");
    delete_file("./server/eq2emulssql.rar");

    // Connect to the MariaDB server and execute SQL statements
    match execute_sql(
        "mysql://eq2emu:eq2emu@localhost:3306/eq2emu", // Modify with your actual credentials
        "./server/eq2emulssql.sql",                    // Modify with the path to your SQL file
    ) {
        Ok(_) => println!("eq2emulssql.sql injected successfully"),
        Err(err) => eprintln!("eq2emulssql.sql not found? Error: {}", err),
    }

    // Download and import opcode table
    download("https://zeklabs.com/dl/ls.sql", "ls.sql", "./server/").unwrap();

    match execute_sql(
        "mysql://eq2emu:eq2emu@localhost:3306/eq2emu", // Modify with your actual credentials
        "./server/ls.sql",                             // Modify with the path to your SQL file
    ) {
        Ok(_) => println!(" ls.sql injected successfully"),
        Err(err) => eprintln!("ls.sql not found ?Error: {}", err),
    }
    delete_file("./server/ls.sql");
}

fn update_loginserver_database() {
    let database = "eq2ls";
    let tables = ["login_characters", "login_worldservers"];
    let output_file = "./server/lschars.sql";

    match execute_mysqldump(database, &tables, output_file) {
        Ok(_) => println!("mysqldump completed successfully."),
        Err(e) => eprintln!("Error: {}", e),
    }

    download(
        "https://zeklabs.com/dl/eq2emulssql.rar",
        "eq2emulssql.rar",
        "./server/",
    )
    .unwrap();
    extract("./server/eq2emulssql.rar", "./server/");
    delete_file("./server/eq2emulssql.rar");

    // Connect to the MariaDB server and execute SQL statements
    match execute_sql(
        "mysql://eq2emu:eq2emu@localhost:3306/eq2emu", // Modify with your actual credentials
        "./server/eq2emulssql.sql",                    // Modify with the path to your SQL file
    ) {
        Ok(_) => println!("eq2emulssql.sql injected successfully"),
        Err(err) => eprintln!("Error: {}", err),
    }

    match execute_sql(
        "mysql://eq2emu:eq2emu@localhost:3306/eq2emu", // Modify with your actual credentials
        "./server/lschars.sql",                        // Modify with the path to your SQL file
    ) {
        Ok(_) => println!("lschars.sql injected successfully"),
        Err(err) => eprintln!("Error: {}", err),
    }

    // Download and import opcode table
    download("https://zeklabs.com/dl/ls.sql", "ls.sql", "./server/").unwrap();
    match execute_sql(
        "mysql://eq2emu:eq2emu@localhost:3306/eq2emu", // Modify with your actual credentials
        "./server/ls.sql",                             // Modify with the path to your SQL file
    ) {
        Ok(_) => println!("Characters restored successfully"),
        Err(err) => eprintln!("Error: {}", err),
    }
    delete_file("./server/lschars.sql");
    delete_file("./server/ls.sql");

    match move_files(&["./server/eq2emuls*"], "./server/oldfiles") {
        Ok(_) => println!("Files moved successfully"),
        Err(err) => eprintln!("Error: {}", err),
    }
}

fn upload_bug_report() {
    println!("Uploading bug report");
    // Upload the bug report to the server
    match mysql_bug_dump("eq2emu", "bugs", "bugs.sql") {
        Ok(_) => println!("Database dumped successfully"),
        Err(err) => eprintln!("Error: {}", err),
    }

    match execute_sql(
        "mysql://eq2emu:idontgive2shits@eq2db.devn00b.com:3306/eq2emu", // Modify with your actual credentials
        "./server/bugs.sql", // Modify with the path to your SQL file
    ) {
        Ok(_) => println!("bugs.sql injected successfully"),
        Err(err) => eprintln!("Error: {}", err),
    }

    delete_file("./server/bugs.sql");

    println!("Bug report uploaded successfully");
}

fn fix_hostname_bug() -> Result<(), Box<dyn Error>> {
    println!("[ Fixing Windows Bug With Hostname. ]");

    // Get the computer name (hostname)
    let hostname = env::var("COMPUTERNAME").unwrap_or_else(|_| "localhost".to_string());

    // Update the loginserver.ini file
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open("loginserver.ini")?;

    let content = format!("127.0.0.1 {}\n", hostname);
    file.write_all(content.as_bytes())?;

    println!("[ HostIP Should now be {} ]", hostname);

    Ok(())

    /* match fix_windows_bug_with_hostname() {
        Ok(()) => println!("Windows bug fixed successfully."),
        Err(err) => eprintln!("Error: {}", err),
    } */
}

fn inject_starter_admin_account() {
    download(
        "https://www.zeklabs.com/dl/eq2emu-account-insert.sql",
        "eq2emu-account-insert.sql",
        "./",
    )
    .unwrap();

    // Connect to the MariaDB server and execute SQL statements
    match execute_sql(
        "mysql://eq2emu:eq2emu@localhost:3306/eq2emu", // Modify with your actual credentials
        "./server/eq2emu-account-insert.sql",          // Modify with the path to your SQL file
    ) {
        Ok(_) => println!("Starter Account injected successfully"),
        Err(err) => eprintln!("Error: {}", err),
    }

    delete_file("./server/eq2emu-account-insert.sql");

    match create_ranonce_file("eq2emu", "eq2emu", "Eqtwoemu") {
        Ok(_) => println!("Account information written to ranonce.txt"),
        Err(err) => eprintln!("Error: {}", err),
    }
}

fn sync_login_to_world() {
    delete_file("./server/oldfiles/worldtolsbak.sql");
    let database = "eq2ls";
    let tables = ["login_characters"];
    let output_file = "./server/oldfiles/worldtolsbak.sql";

    match execute_mysqldump(database, &tables, output_file) {
        Ok(_) => println!("mysqldump completed successfully."),
        Err(e) => eprintln!("Error: {}", e),
    }

    something().unwrap();

    match execute_sql(
        "mysql://eq2emu:eq2emu@localhost:3306/eq2emu",
        "./server/delchar.sql",
    ) {
        Ok(_) => println!("delchar.sql injected successfully"),
        Err(err) => eprintln!("Error: {}", err),
    }

    delete_file("./delchar.sql");

    match execute_sql_insert("mysql://eq2emu:eq2emu@localhost:3306/eq2ls") {
        Ok(_) => println!("Characters restored successfully"),
        Err(err) => eprintln!("Error: {}", err),
    }

    match execute_sql(
        "mysql://eq2emu:eq2emu@localhost:3306/eq2emu",
        "./server/lschars.sql",
    ) {
        Ok(_) => println!("lschars.sql injected successfully"),
        Err(err) => eprintln!("Error: {}", err),
    }
    delete_file("./server/lschars.sql");

    println!("Characters restored successfully");
}

fn execute_sql_insert(url: &str) -> Result<(), Box<dyn Error>> {
    // Connect to the MariaDB server
    let pool = Pool::new(url)?;

    // Execute the SQL statement
    let sql = r#"
        INSERT INTO eq2ls.login_characters (
            account_id, server_id, name, race, class, gender, body_size, body_age, current_zone_id, 
            level, tradeskill_class, tradeskill_level, soga_wing_type, soga_chest_type, soga_legs_type, 
            soga_hair_type, soga_facial_hair_type, soga_model_type, legs_type, chest_type, wing_type, 
            hair_type, facial_hair_type, model_type, deleted, created_date, char_id
        ) 
        SELECT 
            account_id, server_id, name, race, class, gender, body_size, body_age, current_zone_id, 
            level, tradeskill_class, tradeskill_level, soga_wing_type, soga_chest_type, soga_legs_type, 
            soga_hair_type, soga_facial_hair_type, soga_model_type, legs_type, chest_type, wing_type, 
            hair_type, facial_hair_type, model_type, deleted, created_date, id 
        FROM eq2emu.characters;
    "#;

    // Execute SQL statements
    let mut conn = pool.get_conn()?;
    for statement in sql.split(";") {
        let statement = statement.trim();
        if !statement.is_empty() {
            conn.query_drop(statement)?;
        }
    }

    Ok(())
}

fn create_ranonce_file(
    username: &str,
    password: &str,
    character: &str,
) -> Result<(), Box<dyn Error>> {
    let message = format!(
        "[ Starter Account Inserted. Username: {} Password: {} Character: {}]",
        username, password, character
    );

    let mut file = File::create("./ranonce.txt")?;
    writeln!(file, "{}", message)?;

    Ok(())
}

fn start_database_editor() {
    run_program(&PathBuf::from("./binaries/php8.3.bat"), None).expect("Failed to execute command");
}

/*
    END MAIN FUNCTIONS
*/

fn menu() {
    loop {
        // Print the menu
        println!("{}", MENU);

        // Prompt the user for input
        print!("Enter your choice: ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // Parse the user input as an integer
        let choice: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };

        // Call the appropriate function based on the user's choice
        match choice {
            1 => sql_full_update(),
            2 => sql_partial_update(),
            3 => download_server_exe(),
            4 => download_server_lua(),
            5 => download_server_maps(),
            6 => start_mariadb(),
            7 => update_exe(),
            8 => import_loginserver_database(),
            9 => update_loginserver_database(),
            10 => upload_bug_report(),
            11 => println!("{}", CHANGELOG),
            12 => process::exit(0),
            _ => println!("Invalid choice. Please select a valid option."),
        }
    }
}

/*
   Helper Functions
*/

fn move_files(
    source_patterns: &[&str],
    destination_directory: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Create the destination directory if it doesn't exist
    if !Path::new(destination_directory).exists() {
        match fs::create_dir_all(destination_directory) {
            Ok(()) => println!("Directory created successfully."),
            Err(e) => {
                eprintln!("Error creating directory: {}", e);
                return Err(e.into());
            }
        }
    }

    for source_pattern in source_patterns {
        // Use the glob crate to find files matching the pattern
        let source_files = glob::glob(source_pattern)?;
        for source_file in source_files {
            let source_file = source_file?;
            let file_name = source_file
                .file_name()
                .unwrap()
                .to_string_lossy()
                .into_owned();
            let destination_path = Path::new(destination_directory).join(&file_name);

            // Move the file to the destination directory
            fs::rename(&source_file, &destination_path)?;
        }
    }

    Ok(())
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
    let file_path = "./ranonce.txt";

    // Check if the file exists
    if fs::metadata(file_path).is_ok() {
        // File exists
        false
    } else {
        // File doesn't exist
        true
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

fn execute_mysqldump(
    database: &str,
    tables: &[&str],
    output_file: &str,
) -> Result<(), Box<dyn Error>> {
    let mut dump_command = Command::new("./mariqadb/bin/mysqldump");

    // Add command-line arguments
    dump_command.arg("-ueq2emu").arg("-peq2emu").arg(database);

    for table in tables {
        dump_command.arg(*table);
    }

    // Set output redirection
    dump_command.arg(">").arg(output_file);

    // Execute the command
    let status = dump_command.status()?;
    if status.success() {
        println!("mysqldump executed successfully.");
        Ok(())
    } else {
        Err("mysqldump command failed.".into())
    }
}

fn mysql_bug_dump(database: &str, table: &str, output_file: &str) -> Result<(), Box<dyn Error>> {
    let output = Command::new("mysqldump")
        .arg("-ueq2emu")
        .arg("-peq2emu")
        .arg("--no-create-db")
        .arg("--no-create-info")
        .arg("--complete-insert")
        .arg("--skip-add-locks")
        .arg("--skip-add-drop-table")
        .arg("--skip-comments")
        .arg("--compact")
        .arg(database)
        .arg(table)
        .stdout(Stdio::from(std::fs::File::create(output_file)?))
        .output()?;

    if output.status.success() {
        Ok(())
    } else {
        Err("Failed to execute mysqldump".into())
    }
}

fn delete_login_characters_and_write_sql(
    pool: &Pool,
    file_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Connect to the MariaDB server
    let mut conn = pool.get_conn()?;

    // Delete all rows from the login_characters table
    conn.query_drop("DELETE FROM login_characters")?;

    // Write the SQL statement to the file
    std::fs::write(file_path, "DELETE FROM login_characters")?;

    Ok(())
}

fn something() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to the MariaDB server
    let pool = mysql::Pool::new("mysql://eq2emu:eq2emu@localhost:3306/eq2emu")?;

    // Specify the file path to write the SQL statement
    let file_path = "delchar.sql";

    // Call the function to delete rows from the table and write the SQL statement to the file
    delete_login_characters_and_write_sql(&pool, file_path)?;

    Ok(())
}
