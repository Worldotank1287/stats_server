// defines functions and structs related to the internal database
use serde_derive::{Serialize, Deserialize};
use std::io::{
    Read,
    Write
};
use std::sync::{
    Arc,
    Mutex
};

// define our DatabaseVar class
#[derive(Serialize, Deserialize, Clone)]
pub struct DatabaseVar {
    name: String,
    value: String,
    date: String
}

// implement functions for DatabaseVar
impl DatabaseVar {
    /// creates a new one
    pub fn new(name: String, value: String, date: String) -> Self {
        DatabaseVar {
            name,
            value,
            date
        }
    }

    /// creates a new empty variable
    pub fn empty() -> Self {
        DatabaseVar {
            name: String::new(),
            value: String::new(),
            date: String::new()
        }
    }

    /// returns name
    pub fn name(&self) -> String {
        self.name.clone()
    }
    /// returns value
    pub fn value(&self) -> String {
        self.value.clone()
    }
    /// returns date
    pub fn date(&self) -> String {
        self.date.clone()
    }
}

// define our Database structure
#[derive(Serialize, Deserialize)]
pub struct Database {
    vars: Vec<DatabaseVar>
}

/// implements for Database
impl Database {
    /// add a new entry
    pub fn add_entry(&mut self, new_entry: DatabaseVar) {
        self.vars.push(new_entry);
    }

    /// returns the entries in the database
    pub fn entries(&self) -> Vec<DatabaseVar> {
        self.vars.clone()
    }
}

pub type Db = Arc<Mutex<Database>>;

/// initializes a db (potentially from a file)
pub fn init_db(path: Option<String>) -> Result<Db, String> {
    let db = match path {
        Some(a) => {
            match std::fs::File::open(a){
                Ok(mut file) => {
                    // try to parse it
                    let mut data = String::new();
                    file.read_to_string(&mut data).unwrap();
                    serde_json::from_str(&data[..]).unwrap()
                },
                Err(e) => {
                    return Err(format!("Illicit config file: {}", e));
                }  
            }
        },
        None => 
            Database{
                vars: Vec::new()
            }
    };

    return Ok(Arc::new(Mutex::new(db)))
}

/// saves the db to `path`
pub fn save_db(db: Db, path: String) {
    let db = db.clone();
    let db = db.lock().unwrap();
    let dat = &*db;
    let serialized = serde_json::ser::to_string(&dat).unwrap();

    match std::fs::File::create(path) {
        Ok(mut file) => {
            write!(file, "{}", serialized);
        },
        Err(e) => {
            panic!("Failed to save data to file: {}", e)
        }
    };
}