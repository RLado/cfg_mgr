//! Crate: cfg_mgr
//! 
//! This crate is built to import a plain text configuration file into a machine 
//! readable structure.
//! 
//! It's use is very simple, just call load(path :&str) on a properly formatted 
//! config file and all it's data will be imported into a HashMap containing a 
//! String key and a value in the form of the CfgData structure. The user 
//! will require prior knowledge whether to access the numeric or string field. 
//! 
//! By default all parseable numerical values will be parsed into the numeric 
//! field as f64. Multiple numerical entries must be separated by ','. If any of 
//! the values of a field can not be parsed the result is dumped in the string 
//! field of CfgData.
//! 
//! The proper format of a configuration file is as such:
//! --------------------
//! file: [config.cfg]
//! --------------------
//! 
//! # This is a comment
//! foo = 3.1415
//! bar = 1e-3 # comment
//! foobar = 3.1415, 1e-3 # multiple arguments are allowed
//! 
//! path = some/path/example.txt # this can't be parsed as f64 so it's a string
//! 
//! --------------------
//! 
//! 
//! [Examples]
//! 
//! The following example loads a file named "config.cfg" and prints out it's 
//! parsed contents.
//! 
//! ```
//! use cfg_mgr;
//!
//! fn main() {
//!     // Open a configuration file (ignoring errors)
//!     let config = cfg_mgr::load("config.cfg").unwrap();
//! 
//!     // Loop over all the keys of the configuration HashMap
//!     for key in config.keys() {
//!         print!("{}: ", key);
//! 
//!         // Print all numerical values (if any) for a particular key
//!         for i in 0..config.get(key).unwrap().numeric.len(){
//!             print!("{}, ", config.get(key).unwrap().numeric[i]);
//!         }
//! 
//!         // Print the string data of a key (if any) (separate using ;)
//!         println!(";{}", config.get(key).unwrap().string);
//!     }
//! }
//! ```
//! 


use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};


#[derive(Clone)]
pub struct CfgData{
    pub numeric: Vec<f64>,
    pub string: String
}

pub fn load(filename: &str) -> Result<HashMap<String, CfgData>,  std::io::Error> {
    // Crate return HashMap
    let mut config: HashMap<String, CfgData> = HashMap::new();

    // Open the file in read-only mode
    let file: File;
    match File::open(filename){
        Ok(f) => file = f,
        Err(e) => return Err(e)
    }
    let reader = BufReader::new(file);

    // Read the file line by line
    for line in reader.lines() {
        let line_read: String;
        match line{
            Ok(l) => line_read = l,
            Err(e) => return Err(e)
        }
        
        // Strip comments
        if !line_read.contains("="){
            continue;
        }

        let mut line_contents: &str = &line_read;
        if line_read.contains("#"){
            (line_contents, _) = line_read.split_once('#').unwrap();
        }

        // Split keys from values
        let values = line_contents.split("=");

        // Let's populate the return HashMap
        let mut key: String = String::from("");
        let mut val = CfgData{numeric: Vec::<f64>::new(), string: String::from("")};

        for (i, mut item) in values.enumerate() {
            item = item.trim();

            // Get key
            if i == 0 {
                key = String::from(item);
            }

            // Get contents
            if i > 0 && !item.contains(",") {
                let parsed = item.parse::<f64>();
                match parsed {
                    Ok(v) => val.numeric.push(v),
                    Err(_) => {
                        val.string = String::from(item);
                    }
                }
            }

            else if i > 0 && item.contains(",") {
                let subitems = item.split(",");
                for mut subitem in subitems{
                    subitem = subitem.trim();
                    let parsed = subitem.parse::<f64>();
                    match parsed {
                        Ok(v) => val.numeric.push(v),
                        Err(_) => {
                            val.numeric = Vec::<f64>::new();
                            val.string = String::from(item);
                            break;
                        }
                    }
                }
            }

            // Add parsed line to the HashMap
            config.insert(key.clone(), val.clone());
        }
    }

    return Ok(config);
}
