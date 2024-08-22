pub mod mitre;

use std::{
    collections::HashMap,
    fs::{self, read_to_string, File},
    io::Write,
    path::Path,
};

use chrono::NaiveDateTime;
use mitre::MITRE_JSON;
use rust_xlsxwriter::Workbook;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default)]
struct Command {
    command: String,
    user: String,
    system: String,
    timestamp: NaiveDateTime,
    arguments: String,
    mitre: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Technique {
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Tactic {
    name: String,
    children: Vec<Technique>,
}

#[derive(Debug, Serialize, Deserialize)]
struct BRCommand {
    name: String,
    children: Vec<Tactic>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Root {
    children: Vec<BRCommand>,
}

pub fn combine(log_path: &Path) {
    println!("Combining the log files");
    let log_files = create_logfile_vec(log_path);
    fs::create_dir_all(log_path.join("combined"))
        .expect("Could not create directory for the combined logs");
    let mut combined_contents: HashMap<String, String> = HashMap::new();

    for file in log_files {
        let path = Path::new(&file);
        let contents = fs::read_to_string(path).expect("Could not read log file");
        let key = path.file_name().unwrap().to_str().unwrap();
        if combined_contents.contains_key(key) {
            combined_contents.get_mut(key).unwrap().push_str(&contents);
        } else {
            combined_contents.insert(key.to_owned(), contents);
        }
    }

    for (k, v) in combined_contents.into_iter() {
        let mut file = File::create(log_path.join("combined").join(k))
            .expect("Combined log file already exists");
        file.write_all(v.as_bytes())
            .expect("Could not write into a combined log file");
    }

    println!("Combined the logs");
}

fn parse_connection_string(line: &str) -> Command {
    let parts: Vec<&str> = line.split("[").collect();
    let timestamp_parts = parts[0].trim().split(" ").collect::<Vec<&str>>();
    let timestamp = [timestamp_parts[0], timestamp_parts[1]].join(" ");
    let system = parts[2].split("\\").collect::<Vec<&str>>()[0].to_string();
    let user = parts[2].split("\\").collect::<Vec<&str>>()[1];

    let format = "%Y/%m/%d %H:%M:%S";
    let naive_datetime =
        NaiveDateTime::parse_from_str(timestamp.as_str(), format).expect("Failed to parse date");
    Command {
        command: String::from("Established Connection"),
        user: user[..user.len() - 1].to_string(),
        system,
        timestamp: naive_datetime,
        arguments: String::new(),
        mitre: String::new(),
    }
}

fn create_mitre() -> HashMap<String, String> {
    let root: Root = serde_json::from_str(MITRE_JSON).expect("Failed to deserialize JSON");
    let mut command_map: HashMap<String, String> = HashMap::new();
    for command in root.children {
        let mut techniques = String::new();
        for tactic in command.children {
            let mut ts = Vec::new();
            for technique in tactic.children {
                ts.push(technique.name);
            }

            let sep = if techniques.len() > 0 {
                String::from(", ")
            } else {
                String::new()
            };

            techniques = [techniques, sep, ts.join(", ")].concat();
        }
        command_map.insert(command.name, techniques);
    }
    command_map
}

fn parse_command(
    line: &str,
    user: String,
    system: String,
    mitre_map: &HashMap<String, String>,
) -> Command {
    let mut parts: Vec<&str> = line.split("[").collect();
    let timestamp_parts = parts[0].trim().split(" ").collect::<Vec<&str>>();
    let timestamp = [timestamp_parts[0], timestamp_parts[1]].join(" ");
    let format = "%Y/%m/%d %H:%M:%S";
    let naive_datetime =
        NaiveDateTime::parse_from_str(timestamp.as_str(), format).expect("Failed to parse date");

    parts = line.split("=>").collect();
    parts = parts[1].trim().split_whitespace().collect();

    let command = parts[0].to_string();
    let arguments = if parts.len() > 1 {
        parts[1..].join(" ")
    } else {
        String::new()
    };
    let mitre = match mitre_map.get(&command) {
        Some(m) => m,
        None => &String::new(),
    };

    Command {
        command,
        user,
        system,
        timestamp: naive_datetime,
        arguments,
        mitre: mitre.clone(),
    }
}

pub fn create_excel(log_path: &Path) {
    println!("Generating excel file");
    let log_files = fs::read_dir(log_path.join("combined")).unwrap();
    let mut commands: Vec<Command> = Vec::new();

    let mitre_map = create_mitre();

    for file in log_files {
        let mut connection_details = Default::default();
        for line in read_to_string(file.unwrap().path()).unwrap().lines() {
            if line.contains("::badger authenticated") {
                connection_details = parse_connection_string(line);
            } else if line.contains("[input]") {
                let command_details = parse_command(
                    line,
                    connection_details.user.to_owned(),
                    connection_details.system.to_owned(),
                    &mitre_map,
                );
                commands.push(command_details);
            }
        }
        commands.push(connection_details);
    }

    commands.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

    let mut workbook = Workbook::new();
    let mut sheets: HashMap<String, usize> = HashMap::new();
    let mut sheet_index: usize = 0;
    let mut current_sheet;
    let mut current_row = 1;
    for c in commands {
        let key = &c.timestamp.date().to_string();
        if !sheets.contains_key(key) {
            current_sheet = workbook
                .add_worksheet()
                .set_name(key)
                .expect("Could not add worksheet");
            sheets.insert(key.to_string(), sheet_index);
            sheet_index += 1;
            current_row = 1;
            current_sheet
                .write_string(0, 0, "Time")
                .expect("Could not write timestamp to cell");
            current_sheet
                .write_string(0, 1, "Command")
                .expect("Could not write command to cell");
            current_sheet
                .write_string(0, 2, "User")
                .expect("Could not write user to cell");
            current_sheet
                .write_string(0, 3, "System")
                .expect("Could not write system to cell");
            current_sheet
                .write_string(0, 4, "MITRE Technique")
                .expect("Could not write mitre to cell");
            current_sheet
                .write_string(0, 5, "Additional Information")
                .expect("Could not write arguments to cell");
        } else {
            current_sheet = workbook
                .worksheet_from_index(*sheets.get(key).expect("Unknown key"))
                .expect("Could not load existing sheet");
        };
        current_sheet
            .write_string(current_row, 0, c.timestamp.time().to_string())
            .expect("Could not write timestamp to cell");
        current_sheet
            .write_string(current_row, 1, c.command)
            .expect("Could not write command to cell");
        current_sheet
            .write_string(current_row, 2, c.user)
            .expect("Could not write user to cell");
        current_sheet
            .write_string(current_row, 3, c.system)
            .expect("Could not write system to cell");
        current_sheet
            .write_string(current_row, 4, c.mitre)
            .expect("Could not write mitre to cell");
        current_sheet
            .write_string(current_row, 5, c.arguments)
            .expect("Could not write arguments to cell");
        current_row += 1;
    }

    workbook
        .save(log_path.join("log.xlsx"))
        .expect("Could not create log.xlsx");
    println!("Created excel file");
}

pub fn create_logfile_vec(log_path: &Path) -> Vec<String> {
    let paths = fs::read_dir(log_path).unwrap();
    let mut log_files: Vec<String> = vec![String::new(); 0];

    for path in paths {
        let log_file_paths = fs::read_dir(path.unwrap().path().to_string_lossy().into_owned());
        if let Err(e) = log_file_paths {
            println!("Problem opening the directory: {e:?}");
            continue;
        };

        for p2 in log_file_paths.unwrap() {
            let file_name = p2.unwrap().path().to_string_lossy().into_owned();
            if file_name.contains("b-") {
                log_files.push(file_name);
            }
        }
    }

    println!("Found {} log files", &log_files.len());
    log_files
}
