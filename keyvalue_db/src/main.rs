use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::{Read, Write};

struct Database {
    data: HashMap<String, String>,
}

trait Persistence {
    fn save(&self, file_name: &str);
    fn load(&self, file_name: &str) -> Result<Database, String>;
}

trait DatabaseOp {
    fn set(&mut self, key: String, value: String);
    fn get(&self, key: &str) -> Option<&String>;
    fn remove(&mut self, key: &str);
    fn list(&mut self) -> HashMap<String, String>;
}

impl Database {
    fn new() -> Self {
        Database {
            data: HashMap::new(),
        }
    }
}

impl DatabaseOp for Database {
    fn set(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }

    fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }

    fn remove(&mut self, key: &str) {
        self.data.remove(key);
    }

    fn list(&mut self) -> HashMap<String, String> {
        self.data.clone()
    }
}

impl Persistence for Database {
    fn save(&self, file_name: &str) {
        match serde_json::to_string(&self.data) {
            Ok(json) => {
                println!("{}", json);
                match File::create(file_name) {
                    Ok(mut file) => {
                        match file.write_all(json.as_bytes()) {
                            Ok(_) => {}
                            _ => { println!("Unable to write to file") }
                        }
                    }
                    _ => { println!("Unable to create file") }
                }
            }
            _ => { println!("Unable to convert to json") }
        }
    }

    fn load(&self, file_name: &str) -> Result<Database, String> {
        match OpenOptions::new().read(true).open(file_name) {
            Ok(mut file) => {
                let mut data = String::new();
                match file.read_to_string(&mut data) {
                    Ok(_) => {
                        match serde_json::from_str(&data) {
                            Ok(map) => {
                                Ok(Database {
                                    data: map
                                })
                            }
                            _ => { Err(format!("Error while desarilization of json {}", &data)) }
                        }
                    }
                    _ => { Err(format!("Error reading file {} as string", &file_name)) }
                }
            }
            Err(e) => {
                Err(format!("Error during opening file {}", &file_name))
            }
        }
    }
}


fn main() {
    println!("Starting keyValue database...");
    let persistence_file = "keyvalue_db.json";
    let mut db = Database::new();
    match db.load(&persistence_file) {
        Ok(database) => {
            db = database
        }
        Err(_) => {
            print!("Error loading database");
        }
    }

    loop {
        let mut input = String::new();
        println!("Enter your operation: ");
        io::stdin().read_line(&mut input).expect("Failed to read line");


        let args: Vec<&str> = input.trim().split_whitespace().collect();
        match args.get(0) {
            Some(&"set") => {
                if args.len() == 3 {
                    match args.get(1) {
                        Some(key) => {
                            match args.get(2) {
                                Some(value) => {
                                    db.set(key.to_string(), value.to_string());
                                }
                                None => println!("No value was provided"),
                            }
                        }
                        None => println!("No key was provided"),
                    }
                }
            }
            Some(&"get") => {
                if args.len() == 2 {
                    match args.get(1) {
                        Some(key) => {
                            match db.get(key) {
                                Some(value) => { println!("{}", value); }
                                None => { println!("Could not found any record with {}", key) }
                            }
                        }
                        None => println!("No key was provided"),
                    }
                }
            }
            Some(&"remove") => {
                if args.len() == 2 {
                    match args.get(1) {
                        None => println!("No key was provided"),
                        Some(key) => {
                            db.remove(key);
                        }
                    }
                }
            }
            Some(&"list") => {
                let data = db.list();
                println!("{}", serde_json::to_string_pretty(&data).unwrap());
            }
            _ => {}
        }
        db.save(&persistence_file);
    }
}
